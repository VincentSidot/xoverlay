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

pub struct Rectangle {
    anchor: Anchor, // Describes where the coordinate is relative to the shape
    position: Coord,
    size: Size,
    color: Color,
    filled: bool,
}

impl Rectangle {
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

    pub fn anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    pub fn position(&self) -> &Coord {
        &self.position
    }

    pub fn set_position(&mut self, position: Coord) {
        self.position = position;
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<C: Connection> Shape<C> for Rectangle {
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

    fn color(&self) -> &Color {
        &self.color
    }
}
