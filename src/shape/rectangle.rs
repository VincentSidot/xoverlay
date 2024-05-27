use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ConnectionExt,
        Gcontext,
        Rectangle as XRectangle
    }
};

use crate::{
    color::Color,
    drawable::Drawable
};

use super::Shape;

pub struct Rectangle {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    color: Color,
}

impl Rectangle {

    pub fn pixels(
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        color: Color,
        drawable: &dyn Drawable,
    ) -> Result<Box<Self>, Box<dyn Error>> {

        // Ensure that the rectangle is within the bounds of the drawable
        if x < 0 || 
           y < 0 ||
           x + width as i16 > drawable.width() as i16 ||
           y + height as i16 > drawable.height() as i16
        {
            Err("Rectangle is outside the bounds of the drawable")?;
        }
        
        Ok(Box::new(Self {
            x,
            y,
            width,
            height,
            color,
        }))
    }

    pub fn percent(
        fx: f32,
        fy: f32,
        fwidth: f32,
        fheight: f32,
        color: Color,
        drawable: &dyn Drawable,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        let x = (fx * drawable.width() as f32) as i16;
        let y = (fy * drawable.height() as f32) as i16;
        let width = (fwidth * drawable.width() as f32) as u16;
        let height = (fheight * drawable.height() as f32) as u16;

        Self::pixels(x, y, width, height, color, drawable)
    }

}

impl<C: Connection> Shape<C> for Rectangle {
    fn draw(
        &self,
        conn: &C,
        gc: &Gcontext,
        drawable: &dyn Drawable
    ) -> Result<(), Box<dyn Error>> {
        conn.poly_fill_rectangle(
            drawable.id(),
            *gc,
            &[XRectangle {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height,
            }]
        )?;

        Ok(())
    }

    fn color(&self) -> &Color {
        &self.color
    }
}