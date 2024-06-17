//! Overlay module, contains the overlay struct and its methods

use std::{cell::RefCell, error::Error, rc::Rc};

use x11rb::{
    connection::Connection,
    protocol::{
        shape::{self as shape, ConnectionExt as ShapeConnectionExt},
        xproto::{
            ChangeGCAux, ConnectionExt, CreateGCAux, Drawable as XDrawable, Window as XWindow,
        },
    },
    rust_connection::RustConnection,
};

use crate::{
    drawable::{
        pixmap::Pixmap,
        window::{Mapping, Window},
        Drawable,
    }, event::Event, math::vec::Vec2, shape::{
        coord::{Anchor, Coord, Size},
        Rectangle, Shape,
    }, utils, Color
};

pub struct Overlay {
    pub conn: RustConnection,
    parent: Window,
    window: Window,
    render_queue: Vec<Rc<RefCell<dyn Shape<RustConnection>>>>,
    last_mouse_pos: Coord,
}

impl Overlay {
    /// Initialize a new overlay, binding it to the parent window
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent window
    /// * `host` - The host to connect to (if None, connect to $DISPLAY)
    ///
    /// # Returns
    ///
    /// A new overlay struct
    ///
    /// # Errors
    ///
    /// If the overlay could not be created
    ///
    pub fn init(
        parent: XWindow,
        mapping: &Mapping,
        host: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        // Create a new connection
        let (conn, screen_num) = x11rb::connect(host)?;

        // Fetch the root window
        let root = conn.setup().roots[screen_num].root;

        // Create a new window
        let parent = Window::from(&conn, parent, root)?;
        let window = Window::new(&conn, &parent, mapping)?;

        // Create the overlay
        Ok(Self {
            conn,
            parent,
            window,
            render_queue: Vec::new(),
            last_mouse_pos: Coord::new(0.0, 0.0),
        })
    }

    pub fn init_with_name(
        parent: &String,
        mapping: &Mapping,
        host: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let (conn, screen_num) = x11rb::connect(host)?;

        // Fetch the root window
        let root = conn.setup().roots[screen_num].root;

        // Get the parent id
        let id = if let Some(id) = utils::get_best_match(&conn, root, parent)? {
            id
        } else {
            return Err("No window found".into());
        };

        // Create a new window
        let parent = Window::from(&conn, id, root)?;
        let window = Window::new(&conn, &parent, mapping)?;

        // Create the overlay
        Ok(Self {
            conn,
            parent,
            window,
            render_queue: Vec::new(),
            last_mouse_pos: Coord::new(0.0, 0.0),
        })

    }


    pub fn add_shape(&mut self, shape: Rc<RefCell<dyn Shape<RustConnection>>>) -> &mut Self {
        self.render_queue.push(shape);
        self
    }

    pub fn add_shapes<I>(&mut self, shapes: I) -> &mut Self
    where
        I: IntoIterator<Item = Rc<RefCell<dyn Shape<RustConnection>>>>,
    {
        self.render_queue.extend(shapes);
        self
    }

    /// Get the window of the overlay
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Get the parent window of the overlay
    pub fn parent(&self) -> &Window {
        &self.parent
    }

    /// Draw the shapes in the overlay
    pub fn draw(&self) -> Result<&Self, Box<dyn Error>> {
        // Let's build the shape pixmap
        let pixmap = Pixmap::new(&self.conn, &self.window, Some(1))?;

        // Create the graphics context
        let not_transparent_gc = self.conn.generate_id()?;
        self.conn.create_gc(
            not_transparent_gc,
            pixmap.id(),
            &CreateGCAux::new().foreground(1),
        )?;
        let transparent_gc = self.conn.generate_id()?;
        self.conn.create_gc(
            transparent_gc,
            pixmap.id(),
            &CreateGCAux::new().foreground(0),
        )?;

        // Draw the shapes
        for shape in &self.render_queue {
            let shape = shape.borrow();
            if shape.color() == &Color::TRANSPARENT {
                shape.draw(&self.conn, &transparent_gc, &pixmap)?;
            } else {
                shape.draw(&self.conn, &not_transparent_gc, &pixmap)?;
            }
        }

        // Compute the shape to window
        self.conn.shape_mask(
            shape::SO::SET,
            shape::SK::BOUNDING,
            self.window.id(),
            0,
            0,
            pixmap.id(),
        )?;

        // Free the pixmap
        pixmap.free(&self.conn)?;
        // Free the graphics contexts
        self.conn.free_gc(transparent_gc)?;
        self.conn.free_gc(not_transparent_gc)?;

        // Create a new pixmap
        let pixmap = Pixmap::new(&self.conn, &self.window, None)?;

        // Create the graphics context for the shape
        let gc = self.conn.generate_id()?;
        self.conn.create_gc(gc, pixmap.id(), &CreateGCAux::new())?;

        // Draw the pixmap to the window
        for shape in &self.render_queue {
            let shape = shape.borrow();
            if shape.color() != &Color::TRANSPARENT {
                // Set the color
                self.conn.change_gc(
                    gc,
                    &ChangeGCAux::new().foreground(shape.color().value(pixmap.depth())),
                )?;
                // Draw the shape
                shape.draw(&self.conn, &gc, &pixmap)?;
            }
        }

        // Copy the pixmap to the window
        self.conn.copy_area(
            pixmap.id(),
            self.window.id(),
            gc,
            0,
            0,
            0,
            0,
            self.window.width(),
            self.window.height(),
        )?;

        // Free the pixmap
        pixmap.free(&self.conn)?;
        // Free the graphics context
        self.conn.free_gc(gc)?;

        // Flush the connection
        self.conn.flush()?;

        Ok(self)
    }

    /// Return the last mouse position
    pub fn mouse_coord(&self) -> Coord {
        self.last_mouse_pos
    }

    /// Clear the shapes in the overlay
    fn clear_shapes(&mut self) -> &mut Self {
        self.render_queue.clear();
        self
    }

    /// Clear the overlay window
    pub fn clear(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        // Create a new rectangle
        let clear_rect = Rectangle::fill(
            Anchor::default(),
            Coord::new(0.0, 0.0),
            Size::new(1.0, 1.0),
            Color::TRANSPARENT,
        )?;

        self.clear_shapes();
        self.add_shape(clear_rect);
        self.draw()?;
        self.clear_shapes();

        Ok(self)
    }

    fn refresh(&mut self, new_size: Vec2<u16>) -> Result<&mut Self, Box<dyn Error>> {
        if new_size == self.window.size() {
            // No need to resize
            return Ok(self);
        }
        self.parent.resize_event(new_size);
        self.window.refresh(&self.conn, Some(&self.parent))?;
        Ok(self)
    }

    fn handle_event<F>(&mut self, event: Event, mut callback: F) -> Result<bool, Box<dyn Error>>
    where
        F: FnMut(&mut Self, Event) -> Option<Event>,
    {
        match event {
            Event::ParentResize(size) => {
                self.refresh(size)?.draw()?;
            }
            Event::Redraw => {
                self.draw()?;
            }
            Event::MouseMotion { coord } => {
                self.last_mouse_pos = coord;
            }
            Event::StopEventLoop => {
                return Ok(false);
            }
            _ => {}
        }
        // Call the event handler
        let new_event = callback(self, event);
        // Handle the new event
        if let Some(event) = new_event {
            self.handle_event(event, callback)
        } else {
            Ok(true) // Continue the event loop as event does not trigger an event
        }
    }

    pub fn event_loop<F>(&mut self, mut callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&mut Self, Event) -> Option<Event>,
    {
        let mut is_running = true;
        // Draw at least once
        self.draw()?;
        // Main event loop
        while is_running {
            // Poll the event
            let event = Event::wait(self)?;
            is_running = self.handle_event(event, &mut callback)?;
        }
        Ok(())
    }

    pub fn has_focus(&self) -> Result<bool, Box<dyn Error>> {
        self.parent.has_focus(&self.conn)
    }
}

impl Drawable for Overlay {
    fn id(&self) -> XDrawable {
        self.window.id()
    }

    fn depth(&self) -> u8 {
        self.window.depth()
    }

    fn size(&self) -> Vec2<u16> {
        self.window.size()
    }

    fn position(&self) -> Vec2<i16> {
        self.window.position()
    }
}
