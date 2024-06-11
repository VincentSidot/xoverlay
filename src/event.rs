use std::{error::Error, fmt::Debug};

use x11rb::{
    connection::Connection,
    protocol::{
        xproto::ConfigureNotifyEvent,
        xinput::ButtonPressEvent,
        Event as XEvent,
    },
};

use crate::{key::Key, math::vec::Vec2, shape::coord::Coord, Drawable, Overlay};

#[derive(Debug, PartialEq)]
pub enum Button {
    Left,
    Middle,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    ParentResize(Vec2<u16>),
    MousePress { button: Button, coord: Coord },
    MouseMotion { coord: Coord },
    KeyPress(Key),
    KeyRelease(Key),
    Redraw,
    Nothing,
    Unkown,
}

impl Event {
    pub fn wait(overlay: &Overlay) -> Result<Self, Box<dyn Error>> {
        let xevent = overlay.conn.wait_for_event()?;

        match xevent {
            XEvent::XinputMotion(ButtonPressEvent {
                event_x,
                event_y,
                button_mask,
                ..
            }) => {
                // ButtonPressEvent define event_x and event_y as i32
                // But they are actually u16 values
                // So we convert to u16 values using modulo
                let (x, y) = (
                    event_x % (u16::MAX as i32),
                    event_y % (u16::MAX as i32)
                );
                let screen = overlay.size();
                let coord = Coord::new( // Convert to f32 as Coord as percentage values
                    x as f32 / screen.x as f32,
                    y as f32 / screen.y as f32
                );
                println!("Button mask: {:?}", button_mask);
                Ok(Self::MouseMotion { coord })
            }
            XEvent::ConfigureNotify(ConfigureNotifyEvent {
                window,
                width,
                height,
                ..
            }) => {
                let new_size = Vec2::new(width, height);
                if window == overlay.parent().id() && new_size != overlay.size() {
                    return Ok(Self::ParentResize(new_size));
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
