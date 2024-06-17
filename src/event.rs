//! # Event handling module
//! 
//! This module is used to define the event handling system for the overlay library
//! 
//! # Next steps
//!
//! - Add more events
//! - Add tests (Currently no idea how to test this module)

use std::{error::Error, fmt::Debug};

use x11rb::{
    connection::Connection,
    protocol::{xinput::{ButtonPressEvent, RawButtonPressEvent, RawKeyPressEvent}, xproto::ConfigureNotifyEvent, Event as XEvent},
};

use crate::{key::Key, math::vec::Vec2, shape::coord::Coord, Drawable, Overlay};

/// Represents the different mouse buttons.
#[derive(Debug, PartialEq)]
pub enum Button {
    Left,
    Middle,
    Right,
    Unknown,
}

/// Represents the different types of events that can occur.
#[derive(Debug, PartialEq)]
pub enum Event {
    /// Event indicating that the parent window has been resized.
    ParentResize(Vec2<u16>),
    /// Event indicating that a mouse button has been pressed.
    /// 
    /// This trigger only when the parent window is the source of the event
    MousePress { button: Button, coord: Coord },
    /// Event indicating that the mouse has moved.
    /// 
    /// This trigger only when the parent window is the source of the event
    MouseMotion { coord: Coord },
    /// Event indicating that a key has been pressed.
    /// 
    /// This trigger only when the parent window is the source of the event
    KeyPress(Key),
    /// Event indicating that a key has been released.
    /// 
    /// This trigger only when the parent window is the source of the event
    KeyRelease(Key),
    /// Event indicating that a redraw is needed.
    Redraw,
    /// Event indicating that the event loop should stop.
    StopEventLoop,
    /// Event indicating that nothing has happened.
    Nothing,
    /// Event indicating an unknown event.
    Unkown,
}

/// Implement the event handling system for the overlay.
impl Event {
    /// Waits for an event to occur and returns the corresponding `Event` value.
    pub fn wait(overlay: &Overlay) -> Result<Self, Box<dyn Error>> {
        let xevent = overlay.conn.wait_for_event()?;

        match xevent {
            XEvent::XinputMotion(ButtonPressEvent {
                event_x,
                event_y,
                ..
            }) => {
                // ButtonPressEvent define event_x and event_y as i32
                // But they are actually u16 values
                // So we convert to u16 values using modulo
                let (x, y) = (event_x % (u16::MAX as i32), event_y % (u16::MAX as i32));
                let screen = overlay.size();
                let coord = Coord::new(
                    // Convert to f32 as Coord as percentage values
                    x as f32 / screen.x as f32,
                    y as f32 / screen.y as f32,
                );
                Ok(Self::MouseMotion { coord })
            }
            XEvent::XinputRawKeyPress(RawKeyPressEvent{
                detail,
                ..
            }) => {
                // Check if parent window is the source of the event
                if !overlay.has_focus()? {
                    return Ok(Self::Nothing);
                }

                let key = Key::from_xorg_raw(detail as u8);
                Ok(Self::KeyPress(key))
            }
            XEvent::XinputRawButtonPress(RawButtonPressEvent{
                detail,
                ..
            }) => {
                // Check if parent window is the source of the event
                if !overlay.has_focus()? {
                    return Ok(Self::Nothing);
                }

                let button = match detail {
                    1 => Button::Left,
                    2 => Button::Middle,
                    3 => Button::Right,
                    _ => Button::Unknown,
                };
                Ok(Self::MousePress {
                    button,
                    coord: overlay.mouse_coord(),
                })
            }
            XEvent::ConfigureNotify(ConfigureNotifyEvent {
                window,
                width,
                height,
                ..
            }) => {
                let new_size = Vec2::new(width, height);
                if window == overlay.parent().id() && new_size != overlay.size() {
                    Ok(Self::ParentResize(new_size))
                } else {
                    Ok(Self::Unkown)
                }
            }
            XEvent::MapNotify(_) => Ok(Self::Redraw),
            XEvent::NoExposure(_) => Ok(Self::Redraw),
            _ => {
                println!("Unkown event: {:?}", xevent);
                Ok(Self::Unkown)
            }
        }
    }
}
