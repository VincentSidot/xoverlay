//! [![GitHub](https://img.shields.io/badge/GitHub-xoverlay-4d76ae?style=for-the-badge)](https://github.com/VincentSidot/xoverlay)
//! [![Build Status](https://img.shields.io/github/actions/workflow/status/vincentsidot/xoverlay/rust.yml?branch=master&style=for-the-badge)](https://github.com/VincentSidot/xoverlay/actions/workflows/rust.yml)
//! [![License MIT](https://img.shields.io/badge/License-MIT-red.svg?style=for-the-badge)](https://github.com/VincentSidot/xoverlay/blob/master/LICENSE.md)
//! 
//! # Description
//! 
//! XOverlay is a simple and easy-to-use crate for creating Linux application overlays using X11 with minimal extensions (shape and xinput). The overlay system is designed to function without a window compositor, eliminating the need for transparency.
//! 
//! This library is designed for creating overlays for applications like games and video players.
//! It allows to create simple shaped window, with a simple event system.
//! Current handled events are:
//! - Key press
//! - Mouse click
//! - Mouse motion
//! - Resize
//! 
//! # Prerequisites
//! 
//! - Linux Only: This library is only compatible with Linux.
//! - X11 Server: Requires an X11 server with the following extensions:
//!     - XShape: Core of the overlay system for creating shaped windows.
//!     - XInput: For mouse and keyboard events. (This dependency may become optional in future versions.)
//! 
//! # Design Philosophy
//! 
//! XOverlay aims to provide a straightforward API that reduces the complexity of creating application overlays on Linux. The library is intended to be easy to use, with minimal setup required.
//! 
//! # Example
//! 
//!
//! ```no_run
//!
//! use xoverlay::{
//!     event::Event, key::{Key, KeyRef}, shape::{
//!         coord::{Anchor, Coord, Size},
//!         Rectangle,
//!     }, Color, Drawable, Mapping, Overlay, Parent
//! };
//! 
//! use std::{env, error::Error};
//! 
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Fetch window from argument
//!     let args: Vec<String> = env::args().collect();
//!     let window = if args.len() > 1 {
//!         let arg = &args[1];
//!         let num = if let Some(num) = arg.strip_prefix("0x") {
//!             num
//!         } else {
//!             arg
//!         };
//! 
//!         u32::from_str_radix(num, 16)?
//!     } else {
//!         eprintln!("Usage: {} <window>", args[0]);
//!         Err("No window provided")?
//!     };
//! 
//!     // println!("Window: {:#x}", window);
//! 
//!     // Initialize the overlay
//!     let mut overlay = Overlay::init(Parent::Id(window), &Mapping::FullScreen, None)?;
//! 
//!     // Display parent and overlay window ids
//!     println!("Parent window: {:#x}", overlay.parent().id());
//!     println!("Overlay window: {:#x}", overlay.window().id());
//! 
//!     let color_tab = [
//!         Color::RED,
//!         Color::GREEN,
//!         Color::BLUE,
//!         Color::YELLOW,
//!         Color::CYAN,
//!         Color::MAGENTA,
//!         Color::WHITE,
//!     ];
//!     let mut current_color = 0;
//! 
//!     // Create rectangles
//!     let rec = Rectangle::fill(
//!         Anchor::Center,
//!         Coord::new(0.5, 0.5),
//!         Size::new(0.1, 0.1),
//!         color_tab[current_color],
//!     )?;
//! 
//!     // Add the rectangles to the overlay
//!     overlay.add_shape(rec.clone());
//!     overlay.event_loop(|_, event| {
//!         match event {
//!             Event::MouseMotion { coord } => {
//!                 let mut rec = rec.borrow_mut();
//!                 rec.set_position(coord);
//!                 Some(Event::Redraw)
//!             }
//!             Event::KeyPress(Key(KeyRef::ArrowUp)) => {
//!                 println!("ArrowUp pressed");
//!                 Some(Event::StopEventLoop)
//!             }
//!             Event::MousePress { button, coord } => {
//!                 println!("MousePress: {:?} at {:?}", button, coord);
//!                 current_color = (current_color + 1) % color_tab.len();
//! 
//!                 let mut rec = rec.borrow_mut();
//!                 rec.set_forground_color(color_tab[current_color]);
//! 
//!                 Some(Event::Redraw)
//!             }
//!             _ => {
//!                 // Print the event
//!                 // println!("Event: {:?}", event);
//!                 None
//!             }
//!         }
//!     })?;
//! 
//! 
//!     Ok(())
//! }
//! ```
//! You can run this example with the following command:
//! ```sh
//! cargo run --example hello <window_id>
//! cargo run --example hello -- $(xwininfo | grep "Window id:" | grep -o "0x[0-9a-f]*") # Just click on the wanted window
//! ```
//! 
//! # Next steps
//! 
//! Those are the next steps for the lib (the list is not exhaustive and not in order):
//! 
//! - Update and improve the documentation
//! - Add text support (with font)
//! - Add more shapes (circle, triangle, etc.)
//! - Optimize the drawing system regarding the number of shapes
//! - Add more event (resize, etc.)
//! - Add more examples
//! - Use compositor for transparency
//! - Make XInput optional
//! - Add more tests (at least add some tests)
//! - Create github action for CI (build, test, etc.)
//! - Improve the error handling

/// Color module is used to define color for the shapes
mod color;

/// Drawable module is used to define the drawable object (window, pixmap, etc.)
mod drawable;

/// Event module is used to define the event system
pub mod event;

/// Key module is used to define the key event
pub mod key;

/// Math module is used to define the math object (vector, etc.)
/// Very basic euclidean vector implementation
mod math;

/// Overlay module is used to define the overlay object
/// It is the main object of the lib, it is used to create the overlay
mod overlay;

/// Utils module is used to define the utils object
/// Currently it is used to define the window find by name functions
mod utils;

/// Shape module is used to define the shape object
/// Current shapes are:
///    - Rectangle
///         - Fill
///         - Stroke
///    - Arc
///         - Fill
///         - Stroke
///         - Circle
///             - Fill
///             - Stroke
pub mod shape;

/// Export Color enum from color module
pub use color::Color;

/// Export Window Find functions from utils module
pub use utils::find_window_by_name;

/// Export drawable object from drawable module
pub use drawable::{
    window::{Mapping, Window},
    Drawable,
};

/// Export Overlay object from overlay module
pub use overlay::{Overlay, Parent, ResizePolicy};

/// Re-export x11rb crate to allow to use it in the lib
pub use x11rb;
