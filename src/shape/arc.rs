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
    protocol::xproto::{Arc as XArc, ConnectionExt},
};

use crate::{color::Color, drawable::Drawable};

use super::{
    coord::{Anchor, Coord, CoordExt, Size, SizeExt}, GcontextWrapperExt, Shape
};

/// Represents an arc shape.
pub struct Arc {
    anchor: Anchor,
    position: Coord,
    size: Size,
    start_angle: f32,
    end_angle: f32,
    forground: Color,
    background: Color,
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
    /// * `forground` - The color of the edge of the arc.
    /// * `background` - The color of the background of the arc.
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
        forground: Color,
        background: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size,
            start_angle,
            end_angle,
            forground,
            background,
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
            forground: color,
            background: color, // Not used
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
        forground: Color,
        background: Color,
    ) -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Self {
            anchor,
            position,
            size: Size::new(radius, radius),
            start_angle: 0.0,
            end_angle: 360.0,
            forground,
            background,
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
            forground: color,
            background: color, // Not used
            filled: true,
        })))
    }

    /// Returns the position of the arc.
    pub fn position(&self) -> &Coord {
        &self.position
    }

    /// Sets the position of the arc.
    pub fn set_position(&mut self, position: Coord) {
        self.position = position;
    }

    /// Returns the size of the arc.
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Sets the size of the arc.
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Sets the color of the arc.
    pub fn set_forground_color(&mut self, color: Color) {
        self.forground = color;
    }

    /// Sets the background color of the arc.
    pub fn set_background_color(&mut self, color: Color) {
        self.background = color;
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
    fn draw(&self, conn: &C, gc: &GcontextWrapperExt<C>, drawable: &dyn Drawable) -> Result<(), Box<dyn Error>> {
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
                gc.gcontext(),
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
                gc.gcontext(),
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
    fn forground(&self) -> &Color {
        &self.forground
    }

    /// Returns the background color of the arc shape.
    fn background(&self) -> &Color {
        &self.background
    }

    /// Returns the shape size.
    fn size(&self) -> Size {
        self.size
    }

    /// Resizes the shape to the specified size.
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    
    fn anchor(&self) -> &Anchor {
        &self.anchor
    }
    
    fn position(&self) -> Coord {
        self.position
    }
    
    fn set_position(&mut self, position: Coord) {
        self.position = position;
    }
}
