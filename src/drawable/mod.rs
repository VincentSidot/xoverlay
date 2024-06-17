//! This module contains the `Drawable` trait, which represents an object that can be drawn on the screen.
//! 
//! The `Drawable` trait is implemented for `Pixmap` and `Window` to provide a common interface for drawing operations.

use x11rb::protocol::xproto::Drawable as XDrawable;

use crate::{color::Depth, math::vec::Vec2};

pub mod pixmap;
pub mod window;

/// The `Drawable` trait represents an object that can be drawn on the screen.
pub trait Drawable {
    /// Returns the ID of the drawable object.
    fn id(&self) -> XDrawable;

    /// Returns the size of the drawable object.
    fn size(&self) -> Vec2<u16>;

    /// Returns the position of the drawable object.
    fn position(&self) -> Vec2<i16>;

    /// Returns the depth of the drawable object.
    /// 
    /// Depth corresponds to the number of bits per pixel.
    /// 
    /// Allowed depths are:
    /// - Depth::D1 (1 bit per pixel)
    /// - Depth::D8 (8 bits per pixel - grayscale)
    /// - Depth::D16 (16 bits per pixel - high color)
    /// - Depth::D24 (24 bits per pixel - true color)
    /// - Depth::D32 (32 bits per pixel - true color with alpha.)
    /// 
    /// The alpha channel is a hack to represent full transparency.
    /// Currently the lib does not support XComposite extension (To maximize compatibility with all X servers.)
    fn depth(&self) -> Depth;

    /// Returns the width of the drawable object.
    fn width(&self) -> u16 {
        self.size().x
    }

    /// Returns the height of the drawable object.
    fn height(&self) -> u16 {
        self.size().y
    }

    /// Returns the x-coordinate of the drawable object's position.
    fn x(&self) -> i16 {
        self.position().x
    }

    /// Returns the y-coordinate of the drawable object's position.
    fn y(&self) -> i16 {
        self.position().y
    }
}
