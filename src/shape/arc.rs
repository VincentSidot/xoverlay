use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{Arc as XArc, ConnectionExt, Gcontext},
};

use crate::{color::Color, drawable::Drawable};

use super::{
    coord::{Anchor, Coord, CoordExt, Size, SizeExt},
    Shape,
};

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
    pub fn new(
        anchor: Anchor,
        position: Coord,
        size: Size,
        start_angle: f32,
        end_angle: f32,
        color: Color,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(Self {
            anchor,
            position,
            size,
            start_angle,
            end_angle,
            color,
            filled: false,
        }))
    }

    pub fn filled(
        anchor: Anchor,
        position: Coord,
        size: Size,
        start_angle: f32,
        end_angle: f32,
        color: Color,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(Self {
            anchor,
            position,
            size,
            start_angle,
            end_angle,
            color,
            filled: true,
        }))
    }

    pub fn circle(
        anchor: Anchor,
        position: Coord,
        radius: f32,
        color: Color,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(Self {
            anchor,
            position,
            size: Size::new(radius, radius),
            start_angle: 0.0,
            end_angle: 360.0,
            color,
            filled: false,
        }))
    }

    pub fn filled_circle(
        anchor: Anchor,
        position: Coord,
        radius: f32,
        color: Color,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(Self {
            anchor,
            position,
            size: Size::new(radius, radius),
            start_angle: 0.0,
            end_angle: 360.0,
            color,
            filled: true,
        }))
    }
}

impl<C: Connection> Shape<C> for Arc {
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

    fn color(&self) -> &Color {
        &self.color
    }
}
