# XOverlay

[![Documentation][doc-img]][doc-url]
[![Build Status][build-img]][build-img]
[![License: MIT][license-image]][license-url]

[build-img]: https://img.shields.io/github/actions/workflow/status/vincentsidot/xoverlay/rust.yml?branch=master&style=for-the-badge
[build-url]: https://github.com/VincentSidot/xoverlay/actions/workflows/rust.yml
[doc-img]: https://img.shields.io/badge/docs.rs-xoverlay-4d76ae?style=for-the-badge
[doc-url]: https://vincentsidot.github.io/xoverlay/
[license-image]: https://img.shields.io/badge/License-MIT-red.svg?style=for-the-badge
[license-url]: https://github.com/VincentSidot/xoverlay/blob/master/LICENSE.md

## Description

XOverlay lib crate, is a simple and easy to use crate for creating linux application
overlay using X11 and minimal extensions (shape and xinput). The overlay system is 
designed to be fonctional without window compositor (no need for transparency).

This lib is designed to be used for creating overlay for games, video player, etc.
It allow to create simple shaped window, with a simple event system.
Current handled events are:
- Key press
- Mouse click
- Mouse motion
- Resize

Current version is only compatible with linux, and require X11 server. With the
following extensions:
- XShape (for shaped window)
    - It is the core of the overlay system, allowing to create shaped window (later I will allow to use compositor for transparency).
- XInput (for mouse and keyboard event)
    - Later I will make this dependency optional, to allow to use the lib.

The lib is designed to be simple to use, and to be used with the minimum of boilerplate.

## Usage

> Warning: This lib is only compatible with linux, and require X11 server.

You can use this lib by adding the following line to your `Cargo.toml` file:
```toml
[dependencies]
xoverlay = { git = "https://github.com/VincentSidot/xoverlay.git", branch = "master"}
```

Currently the lib is not published on crates.io, so you need to use the git repository.

Also this is still a work in progress, so the API can change.

## Example

```rust
use xoverlay::{
    event::Event, key::{Key, KeyRef}, shape::{
        coord::{Anchor, Coord, Size},
        Rectangle,
    }, Color, Drawable, Mapping, Overlay
};

use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // Fetch window from argument
    let args: Vec<String> = env::args().collect();
    let window = if args.len() > 1 {
        let arg = &args[1];
        let num = if let Some(num) = arg.strip_prefix("0x") {
            num
        } else {
            arg
        };

        u32::from_str_radix(num, 16)?
    } else {
        eprintln!("Usage: {} <window>", args[0]);
        Err("No window provided")?
    };

    // println!("Window: {:#x}", window);

    // Initialize the overlay
    let mut overlay = Overlay::init(window, &Mapping::FullScreen, None)?;

    // Display parent and overlay window ids
    println!("Parent window: {:#x}", overlay.parent().id());
    println!("Overlay window: {:#x}", overlay.window().id());

    let color_tab = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
        Color::WHITE,
    ];
    let mut current_color = 0;

    // Create rectangles
    let rec = Rectangle::fill(
        Anchor::Center,
        Coord::new(0.5, 0.5),
        Size::new(0.1, 0.1),
        color_tab[current_color],
    )?;

    // Add the rectangles to the overlay
    overlay.add_shape(rec.clone());
    overlay.event_loop(|_, event| {
        match event {
            Event::MouseMotion { coord } => {
                let mut rec = rec.borrow_mut();
                rec.set_position(coord);
                Some(Event::Redraw)
            }
            Event::KeyPress(Key(KeyRef::ArrowUp)) => {
                println!("ArrowUp pressed");
                Some(Event::StopEventLoop)
            }
            Event::MousePress { button, coord } => {
                println!("MousePress: {:?} at {:?}", button, coord);
                current_color = (current_color + 1) % color_tab.len();

                let mut rec = rec.borrow_mut();
                rec.set_color(color_tab[current_color]);

                Some(Event::Redraw)
            }
            _ => {
                // Print the event
                // println!("Event: {:?}", event);
                None
            }
        }
    })?;


    Ok(())
}
```

You can run this example with the following command:
```sh
cargo run --example hello <window_id>
cargo run --example hello -- $(xwininfo | grep "Window id:" | grep -o "0x[0-9a-f]*") # Just click on the wanted window
```


## Library modules description

## Next steps

Those are the next steps for the lib (the list is not exhaustive and not in order):

- Add X server extension check
- Update and improve the documentation
- Add text support (with font)
- Add more shapes (circle, triangle, etc.)
- Optimize the drawing system regarding the number of shapes
- Add more event (resize, etc.)
- Add more examples
- Use compositor for transparency
- Make XInput optional
- Add more tests (at least add some tests)
- Update the github repository
- Create github action for CI (build, test, etc.)
- Improve the error handling

## Built With

* [x11rb](https://crates.io/crates/x11rb) - The X11 library used


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
