//! Overlay module, contains the overlay struct and its methods
//! 
//! The overlay is the main object of the library, it is used to create the overlay
//! 
//! # Example
//! 
//! ```no_run
//! use xoverlay::{shape::{coord::{Anchor, Coord, Size}, Rectangle}, Color, Mapping, Overlay, Parent};
//! 
//! use std::error::Error;
//! 
//! const PARENT_WINDOW: &str = "My Beautiful Window";  // The parent window name
//! const MAPPING: Mapping = Mapping::FullScreen;       // The mapping of the overlay
//! const SIZE: Size = Size {x: 0.1, y: 0.1};             // The size of the rectangle
//! const ANCHOR: Anchor = Anchor::Center;              // The anchor of the rectangle
//! const COORD: Coord = Coord {x: 0.5, y: 0.5};          // The position of the rectangle
//! const COLOR: Color = Color::RED;                    // The color of the rectangle
//! const HOST: Option<&str> = None;                    // The host to connect to
//! 
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut overlay = Overlay::init(Parent::Name(PARENT_WINDOW), &MAPPING, HOST)?;
//! 
//!     // Create a new rectangle
//!     let rec = Rectangle::fill(ANCHOR, COORD, SIZE, COLOR)?;
//!     
//!     // Add the rectangle to the overlay
//!     overlay.add_shape(rec);
//!     // Draw the overlay
//!     overlay.draw()?;
//! 
//!     loop {
//!         std::thread::sleep(std::time::Duration::from_secs(1)); // Infinite loop
//!     }
//! }

use std::{cell::RefCell, error::Error, rc::Rc};

use x11rb::{
    connection::Connection,
    protocol::{
        shape::{self as shape, ConnectionExt as ShapeConnectionExt},
        xproto::{
            ConnectionExt, Drawable as XDrawable, FontWrapper, Fontable, Window as XWindow
        },
    },
    rust_connection::RustConnection,
};

use crate::{
    color::Depth, drawable::{
        pixmap::Pixmap,
        window::{Mapping, Window},
        Drawable,
    }, event::Event, math::vec::Vec2, shape::{
        coord::{Anchor, Coord, Size}, GcontextWrapperExt, Rectangle, Shape
    }, utils, Color
};

const SELECTED_FONT: &str = "-misc-fixed-*";

/// The overlay struct
/// 
/// The overlay is the main object of the library, it is used to create the overlay
pub struct Overlay<C>
where
    C: Connection,
{
    /// The connection to the X server
    pub conn: Rc<C>,
    /// The parent window
    parent: Window,
    /// The overlay window
    window: Window,
    /// The render queue (shapes to draw)
    render_queue: Vec<Rc<RefCell<dyn Shape<C>>>>,
    /// The last mouse position
    last_mouse_pos: Coord,
    /// The selected font
    font: FontWrapper<Rc<C>>,
    /// The debounce table
    debounce_table: [std::time::Instant; Event::DB_SIZE],
    /// The resize policy
    resize_policy: ResizePolicy,
}

pub enum Parent<'a> {
    Id(XWindow),
    Name(&'a str),
}

/// The resize policy of the overlay
/// 
/// The resize policy is used to determine how the overlay should be resized
#[derive(Debug, Clone, Copy, Default)]
pub enum ResizePolicy {
    #[default]
    KeepAspectRatio,
    KeepWidth,
    KeepHeight,
    KeepBoth,
}

impl Overlay<RustConnection> {

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
    /// # Example
    /// 
    /// ```no_run
    /// use xoverlay::{Mapping, Overlay, Parent};
    /// let parent = 0x12345678; // The parent window id
    /// let overlay = Overlay::init(Parent::Id(parent), &Mapping::FullScreen, None);
    /// ```
    /// 
    pub fn init(
        parent: Parent,
        mapping: &Mapping,
        host: Option<&str>,
    ) -> Result<Overlay<RustConnection>, Box<dyn Error>> {
        // Create a new connection
        let (conn, screen_num) = x11rb::connect(host)?;

        // Fetch the root window
        let root = conn.setup().roots[screen_num].root;

        // Compute parent id
        let parent_id = match parent {
            Parent::Id(id) => id as XWindow,
            Parent::Name(name) => {
                // Get the parent id
                if let Some(id) = utils::find_window_by_name(&conn, root, name)? {
                    id as XWindow
                } else {
                    return Err("No window found".into());
                }
            }
        };

        Overlay::init_with_conn(parent_id, mapping, conn, screen_num)
    }
}

impl<C> Overlay<C>
where
    C: Connection,
{
    pub fn init_with_conn(parent: XWindow, mapping: &Mapping, conn: C, screen_num: usize) -> Result<Self, Box<dyn Error>>
    {
        // Encapsulate the connection
        let conn = Rc::new(conn);

        // Fetch the root window
        let root = conn.setup().roots[screen_num].root;

        // Create a new window
        let parent = Window::from(&conn, parent, root)?;
        let window = Window::new(&conn, &parent, mapping)?;

        // Create a new font
        let font = FontWrapper::open_font(conn.clone(), SELECTED_FONT.as_bytes())?;

        // Create the overlay
        Ok(Self {
            conn,
            parent,
            window,
            render_queue: Vec::new(),
            last_mouse_pos: Coord::new(0.0, 0.0),
            font,
            debounce_table: Event::gen_debounce_table(),
            resize_policy: ResizePolicy::default(),
        })
    }

    /// Add a shape to the overlay
    /// 
    /// # Arguments
    /// 
    /// * `shape` - The shape to add (any shape implementing the Shape trait)
    /// 
    /// # Returns
    /// 
    /// The overlay struct
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use xoverlay::{shape::{coord::{Anchor, Coord, Size}, Rectangle}, Color, Mapping, Overlay, Parent};
    /// 
    /// use std::error::Error;
    /// 
    /// const PARENT_WINDOW: &str = "My Beautiful Window";  // The parent window name
    /// const MAPPING: Mapping = Mapping::FullScreen;       // The mapping of the overlay
    /// const HOST: Option<&str> = None;                    // The host to connect to
    /// 
    /// const SIZE: Size = Size {x: 0.1, y: 0.1};             // The size of the rectangle
    /// const ANCHOR: Anchor = Anchor::Center;              // The anchor of the rectangle
    /// const COORD: Coord = Coord {x: 0.5, y: 0.5};          // The position of the rectangle
    /// const COLOR: Color = Color::RED;                    // The color of the rectangle
    /// 
    /// let mut overlay = Overlay::init(Parent::Name(PARENT_WINDOW), &MAPPING, HOST).unwrap();
    /// 
    /// // Create a new rectangle
    /// let rec = Rectangle::fill(ANCHOR, COORD, SIZE, COLOR).unwrap();
    ///     
    /// // Add the rectangle to the overlay
    /// overlay.add_shape(rec);
    /// ```
    pub fn add_shape(&mut self, shape: Rc<RefCell<dyn Shape<C>>>) -> &mut Self {
        self.render_queue.push(shape);
        self
    }

    /// Add multiple shapes to the overlay
    /// 
    /// # Arguments
    /// 
    /// * `shapes` - An iterator of shapes to add
    /// 
    /// # Returns
    /// 
    /// The overlay struct
    /// 
    /// # Example
    ///
    /// ```no_run
    /// use xoverlay::{shape::{coord::{Anchor, Coord, Size}, Rectangle, Arc, Shape}, Color, Mapping, Overlay, Parent};
    /// use xoverlay::x11rb::rust_connection::RustConnection;
    /// use std::{cell::RefCell, rc::Rc};
    /// 
    /// use std::error::Error;
    /// 
    /// const PARENT_WINDOW: &str = "My Beautiful Window";  // The parent window name
    /// const MAPPING: Mapping = Mapping::FullScreen;       // The mapping of the overlay
    /// const HOST: Option<&str> = None;                    // The host to connect to
    /// 
    /// let mut overlay = Overlay::init(Parent::Name(PARENT_WINDOW), &MAPPING, HOST).unwrap();
    /// 
    /// // Create a new rectangle
    /// let to_draw: Vec<Rc<RefCell<dyn Shape<RustConnection>>>> = vec![
    ///     Rectangle::fill(Anchor::Center, Coord::new(0.5, 0.5), Size::new(0.1, 0.1), Color::RED).unwrap(),
    ///     Arc::filled_circle(Anchor::Center, Coord::new(0.5, 0.5), 0.1, Color::BLUE).unwrap(),
    /// ];
    ///     
    /// // Add the rectangle to the overlay
    /// overlay.add_shapes(to_draw);
    /// ```
    /// 
    pub fn add_shapes<I>(&mut self, shapes: I) -> &mut Self
    where
        I: IntoIterator<Item = Rc<RefCell<dyn Shape<C>>>>,
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

    pub fn resize_policy(&self) -> ResizePolicy {
        self.resize_policy
    }

    pub fn set_resize_policy(&mut self, policy: ResizePolicy) -> &mut Self {
        self.resize_policy = policy;
        self
    }

    /// Draw the shapes in the overlay
    /// 
    /// # Returns
    /// 
    /// The overlay struct
    /// 
    /// # Errors
    /// 
    /// If the shapes could not be drawn
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use xoverlay::{shape::{coord::{Anchor, Coord, Size}, Rectangle}, Color, Mapping, Overlay, Parent};
    /// 
    /// use std::error::Error;
    /// 
    /// const PARENT_WINDOW: &str = "My Beautiful Window";  // The parent window name
    /// const MAPPING: Mapping = Mapping::FullScreen;       // The mapping of the overlay
    /// const HOST: Option<&str> = None;                    // The host to connect to
    /// 
    /// const SIZE: Size = Size {x: 0.1, y: 0.1};             // The size of the rectangle
    /// const ANCHOR: Anchor = Anchor::Center;              // The anchor of the rectangle
    /// const COORD: Coord = Coord {x: 0.5, y: 0.5};          // The position of the rectangle
    /// const COLOR: Color = Color::RED;                    // The color of the rectangle
    /// 
    /// let mut overlay = Overlay::init(Parent::Name(PARENT_WINDOW), &MAPPING, HOST).unwrap();
    /// 
    /// // Create a new rectangle
    /// let rec = Rectangle::fill(ANCHOR, COORD, SIZE, COLOR).unwrap();
    ///     
    /// // Add the rectangle to the overlay
    /// overlay.add_shape(rec);
    /// 
    /// // Draw the overlay
    /// overlay.draw().unwrap();
    /// ```
    /// 
    pub fn draw(&self) -> Result<&Self, Box<dyn Error>> {
        // Let's build the shape pixmap
        let pixmap = Pixmap::new(&self.conn, &self.window, Some(Depth::D1))?;

        // Create the graphics context
        // let not_transparent_gc = self.conn.generate_id()?;
        // self.conn.create_gc(
        //     not_transparent_gc,
        //     pixmap.id(),
        //     &CreateGCAux::new().foreground(1),
        // )?;
        // let transparent_gc = self.conn.generate_id()?;
        // self.conn.create_gc(
        //     transparent_gc,
        //     pixmap.id(),
        //     &CreateGCAux::new().foreground(0),
        // )?;

        // let not_transparent_gc = GcontextWrapperExt::init(
        //     self.conn.as_ref(),
        //     pixmap.id(),
        //     Some(Color::WHITE.value(&pixmap.depth())),
        //     Some(Color::WHITE.value(&pixmap.depth())),
        //     Some(self.font.font()),
        // )?;

        // let transparent_gc = GcontextWrapperExt::init(
        //     self.conn.as_ref(),
        //     pixmap.id(),
        //     Some(Color::BLACK.value(&pixmap.depth())),
        //     Some(Color::BLACK.value(&pixmap.depth())),
        //     Some(self.font.font()),
        // )?;

        // // Draw the shapes
        // for shape in &self.render_queue {
        //     let shape = shape.borrow();
        //     if shape.forground() == &Color::TRANSPARENT {
        //         shape.draw(&self.conn, &transparent_gc, &pixmap)?;
        //     } else {
        //         shape.draw(&self.conn, &not_transparent_gc, &pixmap)?;
        //     }
        // }

        let mut shape_gc = GcontextWrapperExt::init(
            self.conn.as_ref(),
            pixmap.id(),
            None,
            None,
            Some(self.font.font()),
        )?;

        for shape in self.render_queue.iter() {
            let shape = shape.borrow();

            shape_gc.set_foreground(
                self.conn.as_ref(),
                if shape.forground() == &Color::TRANSPARENT {
                    Some(Color::BLACK.value(&pixmap.depth()))
                } else {
                    Some(shape.forground().value(&pixmap.depth()))
                }
            )?;

            shape_gc.set_background(
                self.conn.as_ref(),
                if shape.background() == &Color::TRANSPARENT {
                    Some(Color::BLACK.value(&pixmap.depth()))
                } else {
                    Some(shape.background().value(&pixmap.depth()))
                }
            )?;

            // Draw the shape
            shape.draw(&self.conn, &shape_gc, &pixmap)?;
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

        // Create a new pixmap
        let pixmap = Pixmap::new(&self.conn, &self.window, None)?;

        // Create the graphics context for the shape
        // let gc = self.conn.generate_id()?;
        // self.conn.create_gc(gc, pixmap.id(), &CreateGCAux::new())?;
        let mut gc = GcontextWrapperExt::init(
            self.conn.as_ref(),
            pixmap.id(),
            None,
            None,
            Some(self.font.font()),
        )?;

        // Draw the pixmap to the window
        for shape in &self.render_queue {
            let shape = shape.borrow();
            if shape.forground() != &Color::TRANSPARENT {
                // Set the color
                gc.set_foreground(&self.conn, Some(shape.forground().value(&pixmap.depth())))?;
            }

            if shape.background() != &Color::TRANSPARENT {
                // Set the background color
                gc.set_background(&self.conn, Some(shape.background().value(&pixmap.depth())))?;
            }

            // Draw the shape
            shape.draw(&self.conn, &gc, &pixmap)?;
        }

        // Copy the pixmap to the window
        self.conn.copy_area(
            pixmap.id(),
            self.window.id(),
            gc.gcontext(),
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
        // self.conn.free_gc(gc)?;

        // Flush the connection
        self.conn.flush()?;

        Ok(self)
    }

    /// Return the last mouse position
    /// 
    /// # Returns
    /// 
    /// The last mouse position
    /// 
    pub fn mouse_coord(&self) -> Coord {
        self.last_mouse_pos
    }

    /// Clear the shapes in the overlay
    /// 
    /// # Returns
    /// 
    /// The overlay struct
    /// 
    fn clear_shapes(&mut self) -> &mut Self {
        self.render_queue.clear();
        self
    }

    /// Clear the overlay window
    /// 
    /// # Returns
    /// 
    /// The overlay struct
    /// 
    /// # Errors
    /// 
    /// If the overlay could not be cleared
    /// 
    /// # Example
    /// ```no_run
    /// use xoverlay::{shape::{coord::{Anchor, Coord, Size}, Rectangle}, Color, Mapping, Overlay, Parent};
    /// 
    /// use std::error::Error;
    /// 
    /// const PARENT_WINDOW: &str = "My Beautiful Window";  // The parent window name
    /// const MAPPING: Mapping = Mapping::FullScreen;       // The mapping of the overlay
    /// const HOST: Option<&str> = None;                    // The host to connect to
    /// 
    /// const SIZE: Size = Size {x: 0.1, y: 0.1};             // The size of the rectangle
    /// const ANCHOR: Anchor = Anchor::Center;              // The anchor of the rectangle
    /// const COORD: Coord = Coord {x: 0.5, y: 0.5};          // The position of the rectangle
    /// const COLOR: Color = Color::RED;                    // The color of the rectangle
    /// 
    /// let mut overlay = Overlay::init(Parent::Name(PARENT_WINDOW), &MAPPING, HOST).unwrap();
    /// 
    /// // Create a new rectangle
    /// let rec = Rectangle::fill(ANCHOR, COORD, SIZE, COLOR).unwrap();
    ///     
    /// // Add the rectangle to the overlay
    /// overlay.add_shape(rec);
    /// 
    /// // Clear the shapes
    /// overlay.clear().unwrap();
    /// ```
    /// 
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

    /// Refresh the overlay
    /// 
    /// # Arguments
    /// 
    /// * `new_size` - The new size of the overlay
    /// 
    /// # Returns
    /// 
    /// The overlay struct
    /// 
    /// # Errors
    /// 
    /// If the overlay could not be refreshed
    /// 
    fn refresh(&mut self, new_size: Vec2<u16>) -> Result<&mut Self, Box<dyn Error>> {

        if new_size == self.window.size() {
            // No need to resize
            return Ok(self);
        }

        self.parent.resize_event(new_size);

        let previous_size = self.window.size();
        self.window.refresh(&self.conn, Some(&self.parent))?;

        match self.resize_policy {
            ResizePolicy::KeepAspectRatio => {
                // Nothing to do
            }
            ResizePolicy::KeepWidth => {
                // Keep the width
                for shape in self.render_queue.iter_mut() {
                    let mut shape = shape.borrow_mut();
                    let size = shape.size();
                    let pos = shape.position();

                    let new_size_width = size.x * (previous_size.x as f32) / (new_size.x as f32);
                    let new_pos_width = pos.x * (previous_size.x as f32) / (new_size.x as f32);

                    shape.set_size(Size::new(new_size_width, size.y));
                    shape.set_position(Coord::new(new_pos_width, pos.y));
                }
            }
            ResizePolicy::KeepHeight => {
                // Keep the height
                // Keep the width
                for shape in self.render_queue.iter_mut() {
                    let mut shape = shape.borrow_mut();
                    let size = shape.size();
                    let pos = shape.position();

                    let new_size_height = size.y * (previous_size.y as f32) / (new_size.y as f32);
                    let new_pos_height = pos.y * (previous_size.y as f32) / (new_size.y as f32);

                    shape.set_size(Size::new(size.x, new_size_height));
                    shape.set_position(Coord::new(pos.x, new_pos_height));
                }
            }
            ResizePolicy::KeepBoth => {
                // Keep the size
                // Keep the width
                for shape in self.render_queue.iter_mut() {
                    let mut shape = shape.borrow_mut();
                    let size = shape.size();
                    let pos = shape.position();

                    let size = size.hammard(previous_size.convert()).inv_hammard(new_size.convert());
                    let pos = pos.hammard(previous_size.convert()).inv_hammard(new_size.convert());

                    shape.set_size(size);
                    shape.set_position(pos);
                }
            }
        }
        Ok(self)
    }

    /// Handle an event
    /// 
    /// # Arguments
    /// 
    /// * `event` - The event to handle
    /// * `callback` - The callback to call when an event is triggered
    /// 
    /// # Returns
    /// 
    /// A boolean indicating if the event loop should continue
    /// 
    /// # Errors
    /// 
    /// If the event could not be handled
    ///
    fn handle_event<F>(&mut self, event: Event, mut callback: F) -> Result<bool, Box<dyn Error>>
    where
        F: FnMut(&mut Self, Event) -> Option<Event>,
    {

        if event.is_debounce(&mut self.debounce_table) {
            // Debounced
            // Return Ok(true) to continue the event loop
            return Ok(true);
        }


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

    /// Start the event loop
    /// 
    /// # Arguments
    /// 
    /// * `callback` - The callback to call when an event is triggered
    /// 
    /// # Returns
    /// 
    /// Nothing, may never return if no StopEventLoop event is triggered
    /// 
    /// # Errors
    /// 
    /// If the event loop could not be started
    /// 
    /// # Notes
    /// 
    /// * The callback should return an Option<Event> to trigger an event.
    /// * The callback should return None to continue the event loop
    /// * The callback take the overlay and the event as arguments
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use xoverlay::{event::Event, key::{Key, KeyRef}, shape::{coord::{Anchor, Coord, Size}, Rectangle}, Color, Mapping, Overlay, Parent};
    /// 
    /// use std::error::Error;
    /// 
    /// const PARENT_WINDOW: &str = "My Beautiful Window";  // The parent window name
    /// const MAPPING: Mapping = Mapping::FullScreen;       // The mapping of the overlay
    /// const HOST: Option<&str> = None;                    // The host to connect to
    /// 
    /// const SIZE: Size = Size {x: 0.1, y: 0.1};             // The size of the rectangle
    /// const ANCHOR: Anchor = Anchor::Center;              // The anchor of the rectangle
    /// const COORD: Coord = Coord {x: 0.5, y: 0.5};          // The position of the rectangle
    /// const COLOR: Color = Color::RED;                    // The color of the rectangle
    /// 
    /// let mut overlay = Overlay::init(Parent::Name(PARENT_WINDOW), &MAPPING, HOST).unwrap();
    /// 
    /// // Create a new rectangle
    /// let rec = Rectangle::fill(ANCHOR, COORD, SIZE, COLOR).unwrap();
    ///     
    /// // Add the rectangle to the overlay
    /// overlay.add_shape(rec);
    /// 
    /// // Start the event loop
    /// overlay.event_loop(|_, event| {
    ///     match event {
    ///         Event::KeyPress(Key(KeyRef::ArrowUp)) => {
    ///             println!("ArrowUp pressed");
    ///             Some(Event::StopEventLoop)
    ///         }
    ///        _ => None
    ///     }
    /// }).unwrap();
    /// ```
    pub fn event_loop<F>(mut self, mut callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&mut Self, Event) -> Option<Event>,
    {
        let mut is_running = true;
        // Draw at least once
        self.draw()?;

        // Main event loop
        while is_running {
            
            // Poll the event
            let event = Event::wait(&self)?;

            is_running = self.handle_event(event, &mut callback)?;

        }
        self.free()?;
        Ok(())
    }

    pub fn poll_event(&mut self) -> Result<Option<Event>, Box<dyn Error>> {
        if let Some(event) = Event::poll(self)? {

            if event.is_debounce(&mut self.debounce_table) {
                // Debounced
                // We do not handle the event
                Ok(None)
            } else {
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
                    _ => {
                    }
                }
                Ok(Some(event))
            }
        } else {
            Ok(None)
        }
    }

    /// Check if the overlay has focus
    /// 
    /// # Returns
    /// 
    /// A boolean indicating if the overlay has focus
    /// 
    /// # Errors
    /// 
    /// If the focus could not be checked
    /// 
    pub fn has_focus(&self) -> Result<bool, Box<dyn Error>> {
        self.parent.has_focus(&self.conn)
    }

    /// Free the overlay
    /// 
    /// # Returns
    /// 
    /// Nothing
    /// 
    /// # Errors
    /// 
    /// If the overlay could not be freed
    /// 
    fn free(self) -> Result<(), Box<dyn Error>> {
        self.window.free(&self.conn)?;
        Ok(())
    }

    /// Get the connection to the X server
    /// 
    /// # Returns
    /// 
    /// The connection to the X server
    /// 
    pub fn conn(&self) -> &C {
        &self.conn
    }

    /// Get the font of the overlay
    /// 
    /// # Returns
    /// 
    /// The font of the overlay
    /// 
    pub fn font(&self) -> Option<Fontable> {
        Some(self.font.font())
    }

}

impl<C> Drawable for Overlay<C>
where
    C: Connection,
{
    /// Get the id of the overlay window
    fn id(&self) -> XDrawable {
        self.window.id()
    }

    /// Get the depth of the overlay window
    fn depth(&self) -> Depth {
        self.window.depth()
    }

    /// Get the size of the overlay window
    fn size(&self) -> Vec2<u16> {
        self.window.size()
    }

    /// Get the position of the overlay window
    fn position(&self) -> Vec2<i16> {
        self.window.position()
    }
}
