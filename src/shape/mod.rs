//! This module contains the shape trait and the shapes that implement it.
//! 
//! The shapes are used to draw on the overlay.

#![allow(dead_code)]

use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::Gcontext};

use crate::{drawable::Drawable, Color};

mod arc;
pub mod coord;
mod rectangle;

pub use arc::Arc;
pub use rectangle::Rectangle;

/// The `Shape` trait represents a shape that can be drawn on the overlay.
pub trait Shape<C>
where
    C: Connection,
{
    /// Draws the shape on the overlay.
    ///
    /// # Arguments
    ///
    /// * `conn` - The X11 connection.
    /// * `gc` - The graphics context used for drawing.
    /// * `drawable` - The drawable object on which the shape will be drawn.
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem drawing the shape.
    fn draw(&self, conn: &C, gc: &Gcontext, drawable: &dyn Drawable) -> Result<(), Box<dyn Error>>;

    /// Returns the color of the shape.
    fn color(&self) -> &Color;
}
