//! Describe the window content

use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ConnectionExt, CreateWindowAux, Window as XWindow, WindowClass
    }
};

use super::Drawable;


/// Describe how the window should be mapped
/// 
/// The window can be mapped in three ways:
/// - FullScreen: The window will be mapped to the full screen
/// - Pixels: The window will be mapped to the specified coordinates
/// - Percent: The window will be mapped to the specified percentages of the parent window
pub enum Mapping {
    FullScreen,
    Pixels {
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    },
    Percent {
        fx: f32,
        fy: f32,
        fwidth: f32,
        fheight: f32,
    },    
}

#[derive(Debug)]
pub struct Window {
    depth: u8,
    id: XWindow,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
}

impl Window{
    pub fn new<C: Connection>(
        conn: &C,
        parent: &Window,
        mapping: &Mapping,
    ) -> Result<Self, Box<dyn Error>> {
        let xwindow = conn.generate_id()?;

        let (x, y, width, height) = {
            let (parent_width, parent_height) = (parent.width, parent.height);

            match mapping {
                Mapping::FullScreen => (0, 0, parent_width, parent_height),
                Mapping::Pixels { x, y, width, height } => {
                    if *x < 0 || *y < 0 || (*x as u16 + width) > parent_width || (*y as u16 + height) > parent_height {
                        Err("Invalid coordinates")?;
                    }
                    (*x, *y, *width, *height)
                },
                Mapping::Percent { fx, fy, fwidth, fheight } => {

                    if *fx < 0.0 || *fy < 0.0 || *fwidth < 0.0 || *fheight < 0.0 || fx + fwidth > 1.0 || fy + fheight > 1.0 {
                        Err("Invalid percentages")?;
                    }

                    let (x, y, width, height) = (
                        (parent_width as f32 * fx) as i16,
                        (parent_height as f32 * fy) as i16,
                        (parent_width as f32 * fwidth) as u16,
                        (parent_height as f32 * fheight) as u16,
                    );

                    (x, y, width, height)
                },
            }
        };

        let depth = parent.depth;

        conn.create_window(
            depth,
            xwindow,
            parent.id,
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new()
                .override_redirect(1)
            ,
        )?;

        conn.map_window(xwindow)?;

        Ok(Self {
            id: xwindow,
            depth,
            width,
            height,
            x,
            y,
        })
    }

    pub fn free<C: Connection>(
        &self,
        conn: &C,
    ) -> Result<(), Box<dyn Error>> {
        conn.destroy_window(self.id)?;
        Ok(())
    }

    pub fn from<C: Connection>(
        conn: &C,
        id: XWindow,
    ) -> Result<Self, Box<dyn Error>> {

        let (depth, width, height, x, y) = {
            let geometry = conn.get_geometry(id)?.reply()?;

            (
                geometry.depth,
                geometry.width,
                geometry.height,
                geometry.x,
                geometry.y,
            )
        };

        Ok(Self {
            id,
            depth,
            width,
            height,
            x,
            y,
        })
    }
    

}

impl Drawable for Window {
    fn id(&self) -> XWindow {
        self.id
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn x(&self) -> i16 {
        self.x
    }

    fn y(&self) -> i16 {
        self.y
    }

    fn depth(&self) -> u8 {
        self.depth
    }
}