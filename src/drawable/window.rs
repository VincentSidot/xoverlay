//! Describe the window content
//! 
//! This module provides a struct `Window` that represents an X11 window. It allows you to create and manage windows with different mappings and event masks.
//!
//! # Example
//!
//! ```no_run
//! use xoverlay::{Mapping, Window};
//! use x11rb::connection::Connection;
//! use x11rb::protocol::xproto::ConnectionExt as _;
//! 
//! let (connection, screen_num) = x11rb::connect(None).unwrap();
//! let root = connection.setup().roots[screen_num].root;
//! let parent_id = 0x12345; // The parent window id
//!
//! // Create a new window with fullscreen mapping
//! let parent = Window::from(&connection, parent_id, root).unwrap();
//! let window = Window::new(&connection, &parent, &Mapping::FullScreen).unwrap();
//! 
//! // Free the window resources
//! window.free(&connection).unwrap();
//! ```

use std::error::Error;

use x11rb::{
    connection::Connection,
    protocol::{
        xinput::{
            ConnectionExt as _, DeviceUse, EventMask as XIEventMask, XIEventMask as XIEventMaskRef,
        },
        xproto::{
            AtomEnum, ChangeWindowAttributesAux, ConfigureWindowAux, ConnectionExt as _, CreateWindowAux, EventMask as XEventMask, Window as XWindow, WindowClass
        },
    },
};

use crate::{color::Depth, math::vec::Vec2};

use super::Drawable;

/// Describe how the window should be mapped
///
/// The window can be mapped in three ways:
/// - FullScreen: The window will be mapped to the full screen
/// - Pixels: The window will be mapped to the specified coordinates
/// - Percent: The window will be mapped to the specified percentages of the parent window
#[derive(Clone, Debug)]
pub enum Mapping {
    /// The window will be mapped to the full screen
    FullScreen,
    /// The window will be mapped to the specified coordinates
    Pixels { pos: Vec2<i16>, size: Vec2<u16> },
    /// The window will be mapped to the specified percentages of the parent window
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
        XEventMask::STRUCTURE_NOTIFY          // Notify when the parent window is resized
    };
    (parent) => {
        XEventMask::STRUCTURE_NOTIFY          // Notify when the parent window is resized 
    };
}

x11rb::atom_manager! {
    /// Atoms used by the window
    /// 
    /// - _NET_ACTIVE_WINDOW: The active window atom
    /// property of the root window
    Atoms:
    AtomsCookie {
        _NET_ACTIVE_WINDOW,
    }
}

/// The window struct
#[derive(Debug)]
pub struct Window {
    /// The window color depth
    depth: Depth,
    /// The window id
    id: XWindow,
    /// The x11 root window
    root: XWindow,
    /// The window mapping
    mapping: Mapping,
    /// The window position regarding the parent window
    pos: Vec2<i16>,
    /// The window size
    size: Vec2<u16>,
}

impl Window {
    /// Create a new window
    /// 
    /// This window will be mapped to the parent window with the specified mapping
    /// 
    /// # Arguments:
    /// 
    /// * `conn` - The X11 connection
    /// * `parent` - The parent window
    /// * `mapping` - The window mapping
    /// 
    /// # Returns:
    /// 
    /// A new window
    /// 
    /// # Errors:
    /// 
    /// This method can return an error if the coordinates or percentages are invalid
    /// 
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

        let depth = Depth::from(parent.depth as u8);

        conn.create_window(
            depth.value(),
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
            root: parent.root,
            depth,
            pos: (x, y).into(),
            size: (width, height).into(),
            mapping: mapping.clone(),
        })
    }

    /// Fetch new size and position of the window
    /// regarding the mapping and the parent window.
    /// 
    /// # Arguments:
    /// 
    /// * `conn` - The X11 connection
    /// * `parent` - The parent window
    /// 
    /// # Returns:
    /// 
    /// Nothing as the window is updated in place
    /// 
    /// # Errors:
    /// 
    /// This method can return an error if the coordinates or percentages are invalid
    /// 
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

    /// Free the window resources
    /// 
    /// # Arguments:
    /// 
    /// * `conn` - The X11 connection
    /// 
    /// # Returns:
    /// 
    /// Nothing if the window is successfully destroyed
    /// 
    /// # Errors:
    /// 
    /// This method can return an error if the window cannot be destroyed
    pub fn free<C: Connection>(self, conn: &C) -> Result<(), Box<dyn Error>> {
        // Use self to consume the window
        conn.destroy_window(self.id)?;
        Ok(())
    }

    /// Create a new window from an existing xwindow
    /// 
    /// This method is used to define the overlay parent window
    /// 
    /// # Arguments:
    /// 
    /// * `conn` - The X11 connection
    /// * `id` - The window id
    /// * `root` - The x11 root window
    /// 
    /// # Returns:
    /// 
    /// A new window object mapped to the parent window
    /// 
    /// # Errors:
    /// 
    /// This method can return an error if the window geometry cannot be fetched
    /// 
    pub fn from<C: Connection>(
        conn: &C,
        id: XWindow,
        root: XWindow,
    ) -> Result<Self, Box<dyn Error>> {
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

        // Add event mask for parent window
        conn.change_window_attributes(
            id,
            &ChangeWindowAttributesAux::new().event_mask(EVENT_MASK!(parent)),
        )?;

        // Allow device events (mouse and keyboard) with xinput
        let devices = conn.xinput_list_input_devices()?.reply()?;
        for device in devices.devices {
            match device {
                device if device.device_use == DeviceUse::IS_X_KEYBOARD => {
                    conn.xinput_xi_select_events(
                        root,
                        &[XIEventMask {
                            deviceid: device.device_id as u16,
                            mask: vec![XIEventMaskRef::RAW_KEY_PRESS],
                        }],
                    )?
                    .check()?;
                }
                device if device.device_use == DeviceUse::IS_X_POINTER => {
                    conn.xinput_xi_select_events(
                        id,
                        &[XIEventMask {
                            deviceid: device.device_id as u16,
                            mask: vec![XIEventMaskRef::MOTION],
                        }],
                    )?
                    .check()?;
                    conn.xinput_xi_select_events(
                        root,
                        &[XIEventMask {
                            deviceid: device.device_id as u16,
                            mask: vec![XIEventMaskRef::RAW_BUTTON_PRESS],
                        }],
                    )?
                    .check()?;
                }
                _ => {}
            }
        }

        Ok(Self {
            id,
            root,
            depth: Depth::from(depth),
            pos: (x, y).into(),
            size: (width, height).into(),
            mapping: Mapping::FullScreen,
        })
    }

    /// Change the window size (field value only as the window is already resized)
    /// Note: The window is not resized here, only the field value is updated
    /// This method is called by the event handler when the window is resized
    /// 
    /// # Arguments:
    /// 
    /// * `size` - The new window size
    /// 
    /// # Returns:
    /// 
    /// Nothing as the window size is updated in place
    /// 
    /// # Errors:
    /// 
    /// This method can't return an error
    /// 
    pub fn resize_event(&mut self, size: Vec2<u16>) {
        self.size = size;
    }

    /// Check if the window has focus
    /// 
    /// # Arguments:
    /// 
    /// * `conn` - The X11 connection
    /// 
    /// # Returns:
    /// 
    /// A boolean indicating if the window has focus
    /// 
    /// # Errors:
    /// 
    /// This method can return an error if the atom cannot be fetched
    /// 
    pub fn has_focus<C: Connection>(&self, conn: &C) -> Result<bool, Box<dyn Error>> {
        // Fetch atom _NET_ACTIVE_WINDOW from the root window
        let atom = Atoms::new(conn)?.reply()?;
        if let Some(mut selected) = conn.get_property(
            false,
            self.root,
            atom._NET_ACTIVE_WINDOW,
            AtomEnum::WINDOW,
            0,
            1,
        )?
        .reply()?
        .value32()  {
            if let Some(selected) = selected.next() {
                Ok(selected == self.id)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl Drawable for Window {

    /// Get the window id
    fn id(&self) -> XWindow {
        self.id
    }

    /// Get the window size
    fn size(&self) -> Vec2<u16> {
        self.size
    }

    /// Get the window position
    fn position(&self) -> Vec2<i16> {
        self.pos
    }

    /// Get the window depth
    fn depth(&self) -> Depth {
        self.depth
    }
}
