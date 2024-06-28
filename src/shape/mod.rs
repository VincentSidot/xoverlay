//! This module contains the shape trait and the shapes that implement it.
//! 
//! The shapes are used to draw on the overlay.

#![allow(dead_code)]

use std::error::Error;
use coord::{Anchor, Coord, Size};
use x11rb::{connection::Connection, protocol::xproto::{ChangeGCAux, ConnectionExt, CreateGCAux, Drawable as XDrawable, Fontable, Gcontext, GcontextWrapper}};

use crate::{drawable::Drawable, Color};

pub type XColor = u32;

mod arc;
pub mod coord;
mod rectangle;
mod text;

pub use arc::Arc;
pub use rectangle::Rectangle;
pub use text::Text;

pub struct GcontextWrapperExt<'c, C: Connection> {
    gc: GcontextWrapper<&'c C>,
    font: Option<Fontable>,
    fg: Option<XColor>,
    bg: Option<XColor>,
}

impl<'c, C: Connection> GcontextWrapperExt<'c, C> {

    pub fn init(conn: &'c C, drawable: XDrawable , fg: Option<XColor>, bg: Option<XColor>, font: Option<Fontable>) -> Result<Self, Box<dyn Error>> {
        
        let value_list = CreateGCAux {
            foreground: fg,
            background: bg,
            font,
            ..CreateGCAux::new()
        };


        let gc = GcontextWrapper::create_gc(conn, drawable, &value_list)?;
        Ok(Self {
            gc,
            font,
            fg,
            bg,
        })
    }

    pub fn set_foreground(&mut self, conn: &C, fg: Option<XColor>) -> Result<(), Box<dyn Error>> {
        self.fg = fg;

        let value_list = ChangeGCAux {
            foreground: fg,
            ..ChangeGCAux::new()
        };

        conn.change_gc(self.gc.gcontext(), &value_list)?;
        
        Ok(())
    }

    pub fn set_background(&mut self, conn: &C, bg: Option<XColor>) -> Result<(), Box<dyn Error>> {
        self.bg = bg;

        let value_list = ChangeGCAux {
            background: bg,
            ..ChangeGCAux::new()
        };

        conn.change_gc(self.gc.gcontext(), &value_list)?;
        
        Ok(())
    }

    pub fn set_font(&mut self, conn: &C, font: Option<Fontable>) -> Result<(), Box<dyn Error>> {
        self.font = font;

        let value_list = ChangeGCAux {
            font,
            ..ChangeGCAux::new()
        };

        conn.change_gc(self.gc.gcontext(), &value_list)?;
        
        Ok(())
    }


    pub fn gcontext(&self) -> Gcontext {
        self.gc.gcontext()
    }

    pub fn font(&self) -> Option<Fontable> {
        self.font
    }
}




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
    fn draw(&self, conn: &C, gc: &GcontextWrapperExt<C>, drawable: &dyn Drawable) -> Result<(), Box<dyn Error>>;

    /// Returns the forground color of the shape.
    fn forground(&self) -> &Color;

    /// Returns the background color of the shape.
    fn background(&self) -> &Color;

    /// Returns the shape size.
    fn size(&self) -> Size;

    /// Resizes the shape to the specified size.
    fn set_size(&mut self, size: Size);

    /// Returns the anchor point of the shape.
    fn anchor(&self) -> &Anchor;

    /// Returns the shape's position.
    fn position(&self) -> Coord;

    /// Sets the shape's position.
    fn set_position(&mut self, position: Coord);
}
