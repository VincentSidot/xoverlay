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
    },
    shape::Shape, Color
};

pub struct Overlay {
    conn: RustConnection,
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

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn draw(&mut self) -> Result<&mut Self, Box<dyn Error>> {

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

        self.render_queue.clear();

        // Free the pixmap
        pixmap.free(&self.conn)?;
        // Free the graphics context
        self.conn.free_gc(gc)?;

        // Flush the connection
        self.conn.flush()?;

        Ok(self)
    }
}

impl Drawable for Overlay {
    fn id(&self) -> XDrawable {
        self.window.id()
    }

    fn width(&self) -> u16 {
        self.window.width()
    }

    fn height(&self) -> u16 {
        self.window.height()
    }

    fn x(&self) -> i16 {
        self.window.x()
    }

    fn y(&self) -> i16 {
        self.window.y()
    }

    fn depth(&self) -> u8 {
        self.window.depth()
    }
}