//! Describe the window content

use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ChangeWindowAttributesAux, ConfigureWindowAux, ConnectionExt, CreateWindowAux,
        EventMask as XEventMask, Window as XWindow, WindowClass,
    },
};

use crate::math::vec::Vec2;

use super::Drawable;

/// Describe how the window should be mapped
///
/// The window can be mapped in three ways:
/// - FullScreen: The window will be mapped to the full screen
/// - Pixels: The window will be mapped to the specified coordinates
/// - Percent: The window will be mapped to the specified percentages of the parent window
#[derive(Clone, Debug)]
pub enum Mapping {
    FullScreen,
    Pixels { pos: Vec2<i16>, size: Vec2<u16> },
    Percent { fpos: Vec2<f32>, fsize: Vec2<f32> },
}

/// Macro to define the event mask for the overlay window
///
/// Arguments:
///     - $window: The window to define the event mask for
/// can be either `overlay` or `parent`
///
macro_rules! EVENT_MASK {
    (overlay) => {
        XEventMask::STRUCTURE_NOTIFY
    };
    (parent) => {
        XEventMask::STRUCTURE_NOTIFY | XEventMask::KEY_PRESS
    };
}

#[derive(Debug)]
pub struct Window {
    depth: u8,
    id: XWindow,
    mapping: Mapping,
    pos: Vec2<i16>,
    size: Vec2<u16>,
}

impl Window {
    pub fn new<C: Connection>(
        conn: &C,
        parent: &Window,
        mapping: &Mapping,
    ) -> Result<Self, Box<dyn Error>> {
        let xwindow = conn.generate_id()?;

        let (x, y, width, height) = {
            let (parent_width, parent_height) = parent.size.into();

            match mapping {
                Mapping::FullScreen => (0, 0, parent_width, parent_height),
                Mapping::Pixels { pos, size } => {
                    let (x, y) = (*pos).into();
                    let (width, height) = (*size).into();

                    if x < 0
                        || y < 0
                        || (x as u16 + width) > parent_width
                        || (y as u16 + height) > parent_height
                    {
                        Err("Invalid coordinates")?;
                    }
                    (x, y, width, height)
                }
                Mapping::Percent { fpos, fsize } => {
                    let (fx, fy) = (*fpos).into();
                    let (fwidth, fheight) = (*fsize).into();

                    if fx < 0.0
                        || fy < 0.0
                        || fwidth < 0.0
                        || fheight < 0.0
                        || fx + fwidth > 1.0
                        || fy + fheight > 1.0
                    {
                        Err("Invalid percentages")?;
                    }

                    let (x, y, width, height) = (
                        (parent_width as f32 * fx) as i16,
                        (parent_height as f32 * fy) as i16,
                        (parent_width as f32 * fwidth) as u16,
                        (parent_height as f32 * fheight) as u16,
                    );

                    (x, y, width, height)
                }
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
                .event_mask(EVENT_MASK!(overlay)),
        )?;

        conn.map_window(xwindow)?;

        Ok(Self {
            id: xwindow,
            depth,
            pos: (x, y).into(),
            size: (width, height).into(),
            mapping: mapping.clone(),
        })
    }

    /// Fetch new size and position of the window
    /// regarding the mapping and the parent window.
    pub fn refresh<C: Connection>(
        &mut self,
        conn: &C,
        parent: Option<&Window>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = parent.as_ref() {
            // Fetch info from the parent and apply the mapping
            let (parent_width, parent_height) = parent.size.into();
            match self.mapping {
                Mapping::FullScreen => {
                    self.pos = (0, 0).into();
                    self.size = (parent_width, parent_height).into();
                }
                Mapping::Pixels { pos, size } => {
                    let (x, y) = pos.into();
                    let (width, height) = size.into();

                    if x < 0
                        || y < 0
                        || (x as u16 + width) > parent_width
                        || (y as u16 + height) > parent_height
                    {
                        Err("Invalid coordinates")?;
                    }
                    self.pos = (x, y).into();
                    self.size = (width, height).into();
                }
                Mapping::Percent { fpos, fsize } => {
                    let (fx, fy) = fpos.into();
                    let (fwidth, fheight) = fsize.into();

                    if fx < 0.0
                        || fy < 0.0
                        || fwidth < 0.0
                        || fheight < 0.0
                        || fx + fwidth > 1.0
                        || fy + fheight > 1.0
                    {
                        Err("Invalid percentages")?;
                    }

                    let (x, y, width, height) = (
                        (parent_width as f32 * fx) as i16,
                        (parent_height as f32 * fy) as i16,
                        (parent_width as f32 * fwidth) as u16,
                        (parent_height as f32 * fheight) as u16,
                    );

                    self.pos = (x, y).into();
                    self.size = (width, height).into();
                }
            }
            // Apply the new size and position to the window
            conn.configure_window(
                self.id,
                &ConfigureWindowAux::new()
                    .x(Some(self.pos.x as i32))
                    .y(Some(self.pos.y as i32))
                    .width(Some(self.size.x as u32))
                    .height(Some(self.size.y as u32)),
            )?;
        } else {
            // Fetch info from the geometry
            let geometry = conn.get_geometry(self.id)?.reply()?;
            let (x, y, width, height) = (geometry.x, geometry.y, geometry.width, geometry.height);
            self.pos = (x, y).into();
            self.size = (width, height).into();
        }
        Ok(())
    }

    pub fn free<C: Connection>(&self, conn: &C) -> Result<(), Box<dyn Error>> {
        conn.destroy_window(self.id)?;
        Ok(())
    }

    pub fn from<C: Connection>(conn: &C, id: XWindow) -> Result<Self, Box<dyn Error>> {
        // Fetch the window attributes
        let rep = conn.get_window_attributes(id)?.reply()?;
        // Display the event mask
        println!("Event mask: {:?}", rep.all_event_masks);

        // Fetch the window geometry
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

        // Add the EventMask::STRUCTURE_NOTIFY to the window event mask
        // So we are able to catch the resize event and update the window
        // size
        conn.change_window_attributes(
            id,
            &ChangeWindowAttributesAux::new().event_mask(EVENT_MASK!(parent)),
        )?;

        Ok(Self {
            id,
            depth,
            pos: (x, y).into(),
            size: (width, height).into(),
            mapping: Mapping::FullScreen,
        })
    }

    /// Change the window size (field value only as the window is already resized)
    /// Note: The window is not resized here, only the field value is updated
    /// This method is called by the event handler when the window is resized
    pub fn resize_event(&mut self, size: Vec2<u16>) {
        self.size = size;
    }
}

impl Drawable for Window {
    fn id(&self) -> XWindow {
        self.id
    }

    fn size(&self) -> Vec2<u16> {
        self.size
    }

    fn position(&self) -> Vec2<i16> {
        self.pos
    }

    fn depth(&self) -> u8 {
        self.depth
    }
}
