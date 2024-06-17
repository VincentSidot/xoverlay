//! Rectangle shape module
//! 
//! This module is used to define the rectangle shape object used by the overlay library

use std::{cell::RefCell, error::Error, rc::Rc};

use x11rb::{
    connection::Connection,
    protocol::xproto::{ConnectionExt, Gcontext, Rectangle as XRectangle},
};

use crate::{color::Color, drawable::Drawable};

use super::{
    coord::{Anchor, Coord, CoordExt, Size, SizeExt},
    Shape,
};

/// Represents a rectangle shape object used by the overlay library.
pub struct Rectangle {
    anchor: Anchor, // Describes where the coordinate is relative to the shape
    position: Coord,
    size: Size,
    color: Color,
    filled: bool,
}

impl Rectangle {
    /// Creates a new filled rectangle shape object.
    ///
    /// # Arguments
    ///
    /// * `anchor` - The anchor point of the rectangle.
    /// * `position` - The position of the rectangle.
    /// * `size` - The size of the rectangle.
    /// * `color` - The color of the rectangle.
    ///
    /// # Returns
    ///
    /// A `Result` containing a reference-counted `RefCell` of the created `Rectangle` object, or a `Box` containing an error if the creation fails.
    pub fn fill(
        anchor: Anchor,
        position: Coord,
        size: Size,
        color: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size,
            color,
            filled: true,
        })))
    }

    /// Creates a new unfilled rectangle shape object.
    ///
    /// # Arguments
    ///
    /// * `anchor` - The anchor point of the rectangle.
    /// * `position` - The position of the rectangle.
    /// * `size` - The size of the rectangle.
    /// * `color` - The color of the rectangle.
    ///
    /// # Returns
    ///
    /// A `Result` containing a reference-counted `RefCell` of the created `Rectangle` object, or a `Box` containing an error if the creation fails.
    pub fn new(
        anchor: Anchor,
        position: Coord,
        size: Size,
        color: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size,
            color,
            filled: false,
        })))
    }

    /// Returns the anchor point of the rectangle.
    pub fn anchor(&self) -> &Anchor {
        &self.anchor
    }

    /// Sets the anchor point of the rectangle.
    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    /// Returns the position of the rectangle.
    pub fn position(&self) -> &Coord {
        &self.position
    }

    /// Sets the position of the rectangle.
    pub fn set_position(&mut self, position: Coord) {
        self.position = position;
    }

    /// Returns the size of the rectangle.
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Sets the size of the rectangle.
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Returns the color of the rectangle.
    pub fn color(&self) -> &Color {
        &self.color
    }

    /// Sets the color of the rectangle.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<C: Connection> Shape<C> for Rectangle {
    /// Draws the rectangle on the specified drawable using the given graphics context.
    ///
    /// # Arguments
    ///
    /// * `conn` - The X11 connection.
    /// * `gc` - The graphics context.
    /// * `drawable` - The drawable to draw on.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if the drawing is successful, or a `Box` containing an error if the drawing fails.
    fn draw(&self, conn: &C, gc: &Gcontext, drawable: &dyn Drawable) -> Result<(), Box<dyn Error>> {
        // Calculate the position of the rectangle
        let coord = self
            .position
            .top_left(&self.anchor, &self.size)
            .to_real_coord(drawable.size());
        let size = self.size.to_real_size(drawable.size());

        let (x, y) = (coord.x as i16, coord.y as i16);
        let (width, height) = (size.x as u16, size.y as u16);

        match self.filled {
            true => conn.poly_fill_rectangle(
                drawable.id(),
                *gc,
                &[XRectangle {
                    x,
                    y,
                    width,
                    height,
                }],
            )?,
            false => conn.poly_rectangle(
                drawable.id(),
                *gc,
                &[XRectangle {
                    x,
                    y,
                    width,
                    height,
                }],
            )?,
        };

        Ok(())
    }

    /// Returns the color of the rectangle.
    fn color(&self) -> &Color {
        &self.color
    }
}
