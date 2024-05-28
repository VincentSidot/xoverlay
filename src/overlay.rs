//! Overlay module, contains the overlay struct and its methods

use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::{
        shape::{
            self as shape, ConnectionExt as ShapeConnectionExt
        },
        xproto::{
            ChangeGCAux,
            ConnectionExt,
            CreateGCAux,
            Drawable as XDrawable,
            Window as XWindow
        }
    },
    rust_connection::RustConnection
};

use crate::{
    drawable::{
        pixmap::Pixmap,
        window::{
        Mapping,
        Window,
        },
        Drawable
    }, event::Event, math::vec::Vec2, shape::{coord::{Anchor, Coord, Size}, Rectangle, Shape}, Color
};

pub struct Overlay {
    pub conn: RustConnection,
    parent: Window,
    window: Window,
    render_queue: Vec<Box<dyn Shape<RustConnection>>>
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
    pub fn init(parent: XWindow, mapping: &Mapping, host: Option<&str>) -> Result<Self, Box<dyn Error>> {
        // Create a new connection
        let (conn, _) = x11rb::connect(host)?;

        // Create a new window
        let parent = Window::from(&conn, parent)?;
        let window = Window::new(&conn, &parent, mapping)?;

        Ok(Self {
            conn,
            parent,
            window,
            render_queue: Vec::new()
        })
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape<RustConnection>>) -> &mut Self{
        self.render_queue.push(shape);
        self
    }

    pub fn add_shapes<I>(&mut self, shapes: I) -> &mut Self
    where
        I: IntoIterator<Item = Box<dyn Shape<RustConnection>>>
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
            &CreateGCAux::new().foreground(1)
        )?;
        let transparent_gc = self.conn.generate_id()?;
        self.conn.create_gc(
            transparent_gc,
            pixmap.id(),
            &CreateGCAux::new().foreground(0)
        )?;

        // Draw the shapes
        for shape in &self.render_queue {
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
            pixmap.id()
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
        self.conn.create_gc(
            gc,
            pixmap.id(),
            &CreateGCAux::new()
        )?;

        // Draw the pixmap to the window
        for shape in &self.render_queue {
            if shape.color() != &Color::TRANSPARENT {
                // Set the color
                self.conn.change_gc(
                    gc,
                    &ChangeGCAux::new()
                        .foreground(shape.color().value(pixmap.depth()))
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
            self.window.height()
        )?;

        // Free the pixmap
        pixmap.free(&self.conn)?;
        // Free the graphics context
        self.conn.free_gc(gc)?;

        // Flush the connection
        self.conn.flush()?;

        Ok(self)
    }

    /// Clear the shapes in the overlay
    fn clear_shapes(&mut self) -> &mut Self{
        self.render_queue.clear();
        self
    }

    /// Clear the overlay window
    pub fn clear(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        let rectangle = Rectangle::new(
            Anchor::default(),
            Coord::new(0.0, 0.0),
            Size::new(1.0, 1.0),
            Color::TRANSPARENT
        )?;

        self.clear_shapes();
        self.add_shape(rectangle);
        self.draw()?;
        self.clear_shapes();
        
        Ok(self)
    }

    fn refresh(&mut self, new_size: Vec2<u16>) -> Result<&mut Self, Box<dyn Error>> {
        self.parent.resize_event(new_size);
        self.window.refresh(&self.conn, Some(&self.parent))?;
        Ok(self)
    }

    pub fn event_loop<F>(
        &mut self,
        mut callback: F
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&mut Self, Event) -> ()
    {
        // Draw at least once
        self.draw()?;
        // Main event loop
        loop {
            // Poll the event
            let event = Event::wait(&self)?;
            match event {
                Event::ParentResize(size) => {
                    self.refresh(size)?.draw()?;
                },
                Event::Redraw => {
                    self.draw()?;
                },
                _ => {
                    // Call the event handler
                    callback(self, event);
                }
            }
        }
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