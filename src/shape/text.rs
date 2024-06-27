//! Text shape module
//! 
//! This module is used to define the text shape object used by the overlay library

use std::{cell::RefCell, error::Error, rc::Rc};

use x11rb::{connection::Connection, protocol::xproto::{Char2b, ConnectionExt, Fontable}};

use crate::{math::vec::Vec2, Color, Drawable, Overlay};

use super::{coord::{Anchor, Coord, CoordExt, Size}, GcontextWrapperExt, Shape};


/// Represents a text shape object used by the overlay library.
pub struct Text {
    anchor: Anchor, // Describes where the coordinate is relative to the shape
    position: Coord,
    forground: Color,
    background: Color,
    text: String,
    content: Vec<Char2b>,
    previous: Rc<RefCell<Option<(Size, Fontable)>>>
}

fn string_to_char2b(text: &str) -> Vec<Char2b> {
    text.chars().filter(|c| c.is_ascii()).map(|c| Char2b { byte2: c as u8, byte1: 0x0 }).collect()
}

impl Text {

    pub fn text<T: ToString>(
        anchor: Anchor,
        position: Coord,
        forground: Color,
        background: Color,
        text: T,
    ) -> Rc<RefCell<Self>> {
        let text = text.to_string();
        let content = string_to_char2b(&text);
        
        Rc::new(RefCell::new(Self {
            anchor,
            position,
            forground,
            background,
            text,
            content,
            previous: Rc::new(RefCell::new(None))
        }))
    }

    pub fn get_string(&self) -> &str {
        &self.text
    }

    pub fn set_string<T: ToString>(&mut self, text: T) {
        let text = text.to_string();
        if text == self.text {
            return // No need to update the content
        }
        self.text = text;
        self.content = string_to_char2b(&self.text);
        // Force a recalculation of the size as the text has changed
        *self.previous.borrow_mut() = None;
    }

    pub fn get_size<C: Connection>(&self, overlay: &Overlay<C>) -> Result<Size, Box<dyn Error>> {
        self.get_size_raw(overlay.conn(), overlay.font().ok_or("No Font Selected")?, overlay.size())
    }

    /// Returns the position of the text.
    pub fn position(&self) -> &Coord {
        &self.position
    }

    /// Sets the position of the text.
    pub fn set_position(&mut self, position: Coord) {
        self.position = position;
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.anchor = anchor;
    }

    /// Sets the color of the text.
    pub fn set_forground_color(&mut self, color: Color) {
        self.forground = color;
    }

    /// Sets the background color of the text.
    pub fn set_background_color(&mut self, color: Color) {
        self.background = color;
    }

    fn get_size_raw<C: Connection>(&self, conn: &C, font: Fontable, size: Vec2<u16>) -> Result<Size, Box<dyn Error>> {
        
        let (size, previous) = match self.previous.as_ref().borrow().as_ref() {
            Some(previous) if previous.1 == font => {
                // The font has not changed, we can reuse the previous size
                return Ok(previous.0)
            },
            _ => {
                // First we need to compute the bounding box of the text
                let extents = conn.query_text_extents(font, &self.content)?.reply()?;
        
                let raw_width = extents.overall_width;
                let raw_height = extents.overall_ascent as i32 + extents.overall_descent as i32;
        
                println!("Raw Width: {}, Raw Height: {}", raw_width, raw_height);
                println!("Size: {:?}", size);
        
                // Translate the size to portion of the screen
                let width = raw_width as f32 / size.x() as f32;
                let height = raw_height as f32 / size.y() as f32;
        
                println!("Width: {}, Height: {}", width, height*0.4);
        
        
                let size = Size::new(width, height); // Source: trust me bro

                // Let's store the size for future use
                (size.clone(), Some((size, font)))
            }
        };

        *self.previous.borrow_mut() = previous;

        Ok(size)
    }
}

impl<C: Connection> Shape<C> for Text {
    fn draw(&self, conn: &C, gc: &GcontextWrapperExt<C>, drawable: &dyn crate::Drawable) -> Result<(), Box<dyn std::error::Error>> {
        
        // Build the content of the text
        let font = gc.font.ok_or("No font set")?;
        
        // Then we need to compute the bounding box of the text
        let size = self.get_size_raw(conn, font, drawable.size())?;

        let coord = self
            .position
            .bottom_left(&self.anchor, &size)
            .to_real_coord(drawable.size());

        let (x, y) = (coord.x as i16, coord.y as i16);

        // Draw the text
        conn.image_text16(
            drawable.id(),
            gc.gcontext(),
            x, y,
            &self.content
        )?;

        Ok(())
    }

    fn forground(&self) -> &Color {
        &self.forground
    }

    fn background(&self) -> &Color {
        &self.background
    }
}