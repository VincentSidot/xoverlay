//! Pixmap drawable object.
//! 
//! A `Pixmap` is a drawable object that can be drawn on the screen.
//! 
//! It is used to create off-screen buffers for drawing operations.
//! 

use std::error::Error;

use x11rb::{connection::Connection, protocol::xproto::ConnectionExt};

use super::{window::Window, Drawable};

use crate::{color::Depth, math::vec::Vec2};

/// Represents a pixmap drawable object.
/// 
/// The `Pixmap` object is a drawable object that can be drawn on the screen.
/// 
/// It is used to create off-screen buffers for drawing operations.
/// 
/// The `Pixmap` object is associated with a window, and has the same size and depth as the window.
/// It also have the same lifetime as the window.
/// 
pub struct Pixmap<'w> {
    /// The ID of the pixmap.
    id: u32,
    /// The window that the pixmap is associated with.
    /// Lifetime of the pixmap is the same as the window.
    window: &'w Window,
    /// The depth of the pixmap. (Number of bits per pixel.)
    depth: Depth,
}

/// Implementation of the `Pixmap` object.
impl<'w> Pixmap<'w> {
    /// Creates a new pixmap drawable object.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The X11 connection.
    /// * `window` - The window that the pixmap is associated with.
    /// * `depth` - The depth of the pixmap. (Number of bits per pixel.)
    /// 
    /// # Returns
    /// 
    /// A new `Pixmap` object.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the pixmap could not be created.
    /// 
    pub fn new<C: Connection>(
        conn: &C,
        window: &'w Window,
        depth: Option<Depth>,
    ) -> Result<Self, Box<dyn Error>> {
        let depth = depth.unwrap_or(window.depth());

        let id = conn.generate_id()?;

        conn.create_pixmap(depth.value(), id, window.id(), window.width(), window.height())?;

        Ok(Self { id, window, depth })
    }

    /// Frees the pixmap drawable object.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The X11 connection.
    /// 
    /// # Returns
    /// 
    /// An empty result.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the pixmap could not be freed.
    /// 
    pub fn free<C: Connection>(self, conn: &C) -> Result<(), Box<dyn Error>> {
        conn.free_pixmap(self.id)?;
        Ok(())
    }
}

impl<'w> Drawable for Pixmap<'w> {

    /// Returns the ID of the pixmap.
    fn id(&self) -> u32 {
        self.id
    }

    /// Returns the depth of the pixmap.
    fn depth(&self) -> Depth {
        self.depth
    }

    /// Returns the window that the pixmap is associated with.
    fn size(&self) -> Vec2<u16> {
        self.window.size()
    }

    /// Returns the position of the pixmap.
    fn position(&self) -> Vec2<i16> {
        self.window.position()
    }
}
