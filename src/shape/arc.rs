//! Arc shape module.
//! 
//! This module is used to define the arc shape object used by the overlay library.
//! 
//! Arc shape corresponds to a part of an ellipse.
//! It also offers the possibility to draw a circle directly.
//! 
//! # Future improvements
//! 
//! - Add more options to the arc shape
//! - Improve circle drawing (currently it has a constant width/height ratio)

use std::{cell::RefCell, error::Error, rc::Rc};

use x11rb::{
    connection::Connection,
    protocol::xproto::{Arc as XArc, ConnectionExt, Gcontext},
};

use crate::{color::Color, drawable::Drawable};

use super::{
    coord::{Anchor, Coord, CoordExt, Size, SizeExt},
    Shape,
};

/// Represents an arc shape.
pub struct Arc {
    anchor: Anchor,
    position: Coord,
    size: Size,
    start_angle: f32,
    end_angle: f32,
    color: Color,
    filled: bool,
}

impl Arc {
    /// Creates a new arc shape.
    ///
    /// # Arguments
    ///
    /// * `anchor` - The anchor point of the arc.
    /// * `position` - The position of the arc.
    /// * `size` - The size of the arc.
    /// * `start_angle` - The start angle of the arc in degrees.
    /// * `end_angle` - The end angle of the arc in degrees.
    /// * `color` - The color of the arc.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boxed `Arc` object or an error.
    pub fn new(
        anchor: Anchor,
        position: Coord,
        size: Size,
        start_angle: f32,
        end_angle: f32,
        color: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size,
            start_angle,
            end_angle,
            color,
            filled: false,
        })))
    }

    /// Creates a new filled arc shape.
    ///
    /// # Arguments
    ///
    /// * `anchor` - The anchor point of the arc.
    /// * `position` - The position of the arc.
    /// * `size` - The size of the arc.
    /// * `start_angle` - The start angle of the arc in degrees.
    /// * `end_angle` - The end angle of the arc in degrees.
    /// * `color` - The color of the arc.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boxed `Arc` object or an error.
    pub fn filled(
        anchor: Anchor,
        position: Coord,
        size: Size,
        start_angle: f32,
        end_angle: f32,
        color: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size,
            start_angle,
            end_angle,
            color,
            filled: true,
        })))
    }

    /// Creates a new circle shape.
    ///
    /// # Arguments
    ///
    /// * `anchor` - The anchor point of the circle.
    /// * `position` - The position of the circle.
    /// * `radius` - The radius of the circle.
    /// * `color` - The color of the circle.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boxed `Arc` object or an error.
    pub fn circle(
        anchor: Anchor,
        position: Coord,
        radius: f32,
        color: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size: Size::new(radius, radius),
            start_angle: 0.0,
            end_angle: 360.0,
            color,
            filled: false,
        })))
    }

    /// Creates a new filled circle shape.
    ///
    /// # Arguments
    ///
    /// * `anchor` - The anchor point of the circle.
    /// * `position` - The position of the circle.
    /// * `radius` - The radius of the circle.
    /// * `color` - The color of the circle.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boxed `Arc` object or an error.
    pub fn filled_circle(
        anchor: Anchor,
        position: Coord,
        radius: f32,
        color: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size: Size::new(radius, radius),
            start_angle: 0.0,
            end_angle: 360.0,
            color,
            filled: true,
        })))
    }
}

impl<C: Connection> Shape<C> for Arc {
    /// Draws the arc shape on the specified drawable.
    ///
    /// # Arguments
    ///
    /// * `conn` - The X11 connection.
    /// * `gc` - The graphics context.
    /// * `drawable` - The drawable to draw on.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error.
    fn draw(&self, conn: &C, gc: &Gcontext, drawable: &dyn Drawable) -> Result<(), Box<dyn Error>> {
        let coord = self
            .position
            .top_left(&self.anchor, &self.size)
            .to_real_coord(drawable.size());
        let size = self.size.to_real_size(drawable.size());

        let (x, y) = (coord.x as i16, coord.y as i16);
        let (width, height) = (size.x as u16, size.y as u16);

        match self.filled {
            true => conn.poly_fill_arc(
                drawable.id(),
                *gc,
                &[XArc {
                    x,
                    y,
                    width,
                    height,
                    angle1: (self.start_angle * 64.0) as i16,
                    angle2: (self.end_angle * 64.0) as i16,
                }],
            )?,
            false => conn.poly_arc(
                drawable.id(),
                *gc,
                &[XArc {
                    x,
                    y,
                    width,
                    height,
                    angle1: (self.start_angle * 64.0) as i16,
                    angle2: (self.end_angle * 64.0) as i16,
                }],
            )?,
        };

        Ok(())
    }

    /// Returns the color of the arc shape.
    ///
    /// # Returns
    ///
    /// A reference to the color of the arc shape.
    fn color(&self) -> &Color {
        &self.color
    }
}
