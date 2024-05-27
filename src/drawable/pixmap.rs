use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::ConnectionExt,
};

use super::{
    window::Window,
    Drawable,
};


pub struct Pixmap<'w> {
    id: u32,
    window: &'w Window,
    depth: u8,
}

impl<'w> Pixmap<'w> {

    pub fn new<C: Connection>(
        conn: &C,
        window: &'w Window,
        depth: Option<u8>
    ) -> Result<Self, Box<dyn Error>> {
        let depth = depth.unwrap_or(window.depth());

        let id = conn.generate_id()?;

        conn.create_pixmap(
            depth,
            id,
            window.id(),
            window.width(),
            window.height()
        )?;

        Ok(Self {
            id,
            window,
            depth
        })
    }

    pub fn free<C: Connection>(
        &self,
        conn: &C
    ) -> Result<(), Box<dyn Error>> {
        conn.free_pixmap(self.id)?;
        Ok(())
    }

}

impl<'w> Drawable for Pixmap<'w> {
    fn id(&self) -> u32 {
        self.id
    }

    fn width(&self) -> u16 {
        self.window.width()
    }

    fn height(&self) -> u16 {
        self.window.height()
    }

    fn x(&self) -> i16 {
        self.window.x()
    }

    fn y(&self) -> i16 {
        self.window.y()
    }

    fn depth(&self) -> u8 {
        self.depth
    }
}