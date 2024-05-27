use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ConnectionExt,
        Gcontext,
        Arc as XArc
    }
};


use crate::{
    color::Color,
    drawable::Drawable
};

use super::Shape;

pub struct Arc {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    start_angle: f32,
    end_angle: f32,
    color: Color,
}

impl Arc {

    pub fn pixels(
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        start_angle: f32,
        end_angle: f32,
        color: Color,
        drawable: &dyn Drawable,
    ) -> Result<Box<Self>, Box<dyn Error>> {

        // Ensure that the arc is within the bounds of the drawable
        if x < 0 || 
           y < 0 ||
           x + width as i16 > drawable.width() as i16 ||
           y + height as i16 > drawable.height() as i16
        {
            Err("Arc is outside the bounds of the drawable")?;
        }
        
        Ok(Box::new(Self {
            x,
            y,
            width,
            height,
            start_angle,
            end_angle,
            color,
        }))
    }

    pub fn percent(
        fx: f32,
        fy: f32,
        fwidth: f32,
        fheight: f32,
        start_angle: f32,
        end_angle: f32,
        color: Color,
        drawable: &dyn Drawable,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        let x = (fx * drawable.width() as f32) as i16;
        let y = (fy * drawable.height() as f32) as i16;
        let width = (fwidth * drawable.width() as f32) as u16;
        let height = (fheight * drawable.height() as f32) as u16;

        Ok(Self::pixels(x, y, width, height, start_angle, end_angle, color, drawable)?)
    }

    pub fn circle(
        x: i16,
        y: i16,
        radius: u16,
        color: Color,
        drawable: &dyn Drawable,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        Self::pixels(x - radius as i16, y - radius as i16, radius * 2, radius * 2, 0.0, 360.0, color, drawable)
    }
    
    pub fn circle_percent(
        fx: f32,
        fy: f32,
        fradius: f32,
        color: Color,
        drawable: &dyn Drawable,
    ) -> Result<Box<Self>, Box<dyn Error>> {
        let x = (fx * drawable.width() as f32) as i16;
        let y = (fy * drawable.height() as f32) as i16;
        let radius = (fradius * drawable.width() as f32) as u16;

        Self::circle(x, y, radius, color, drawable)
    }

}

impl<C: Connection> Shape<C> for Arc {

    fn draw(&self, conn: &C, gc: &Gcontext, drawable: &dyn Drawable) ->Result<(), Box<dyn Error>> {
        conn.poly_fill_arc(
            drawable.id(),
            *gc,
            &[XArc {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height,
                angle1: (self.start_angle * 64.0) as i16,
                angle2: (self.end_angle * 64.0) as i16,
            }]
        )?;

        Ok(())
    }

    fn color(&self) -> &Color {
        &self.color
    }
}