use std::{error::Error, fmt::Debug};

use x11rb::{
    connection::Connection,
    protocol::{xproto::{ButtonPressEvent, ConfigureNotifyEvent, KeyButMask}, Event as XEvent},
};

use crate::{
    math::vec::Vec2, shape::coord::Coord, Drawable, Overlay
};

#[derive(Debug, PartialEq)]
pub enum Button {
    Left,
    Middle,
    Right
}

#[derive(Debug, PartialEq)]
pub enum Event {
    ParentResize(Vec2<u16>),
    MousePress{
        button: Button,
        coord: Coord
    },
    Redraw,
    Unkown
}

impl Event {

    pub fn wait(overlay: &Overlay) -> Result<Self, Box<dyn Error>> {
        let xevent = overlay.conn.wait_for_event()?;

        match xevent {
            XEvent::ButtonPress(ButtonPressEvent{
                same_screen: true,
                event_x,
                event_y,
                state,
                ..
            }) => {
                println!("ButtonPress event: x: {}, y: {}, state: {:?}", event_x, event_y, state);
                let button = match state {
                    KeyButMask::BUTTON1 => Button::Left,
                    KeyButMask::BUTTON2 => Button::Right,
                    KeyButMask::BUTTON3 => Button::Middle,
                    _ => return Ok(Self::Unkown)
                };

                // Ensure the event is within the window
                let (xpos, ypos) = overlay.position().into();
                let (width, height) = overlay.size().into();

                if event_x < xpos || event_x > xpos + width as i16 || event_y < ypos || event_y > ypos + height as i16 {
                    return Ok(Self::Unkown)
                }

                // Calculate the relative position
                let relative = overlay.position() - Vec2::new(event_x, event_y);
                let coord = Coord::new(
                    relative.x as f32 / width as f32,
                    relative.y as f32 / height as f32
                );

                Ok(Self::MousePress{
                    button,
                    coord
                })
            }
            XEvent::ConfigureNotify(ConfigureNotifyEvent{
                window,
                width,
                height,
                ..
            }) => {
                let new_size = Vec2::new(width, height);
                if window == overlay.parent().id() && new_size != overlay.size() {
                    return Ok(Self::ParentResize(new_size))
                } else {
                    Ok(Self::Unkown)
                }
            }
            XEvent::MapNotify(_) => Ok(Self::Redraw),
            _ => {
                println!("Unkown event: {:?}", xevent);
                Ok(Self::Unkown)
            }
        }
    }
}