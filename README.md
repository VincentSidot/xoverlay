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

XOverlay is a simple and easy-to-use crate for creating Linux application overlays using X11 with minimal extensions (shape and xinput). The overlay system is designed to function without a window compositor, eliminating the need for transparency.

This library is designed for creating overlays for applications like games and video players.
It allows to create simple shaped window, with a simple event system.

Currently handled events include:
- Key press
- Mouse click
- Mouse motion
- Resize

## Prerequisites

- Linux Only: This library is only compatible with Linux.
- X11 Server: Requires an X11 server with the following extensions:
    - XShape: Core of the overlay system for creating shaped windows.
    - XInput: For mouse and keyboard events. (This dependency may become optional in future versions.)

## Design Philosophy

XOverlay aims to provide a straightforward API that reduces the complexity of creating application overlays on Linux. The library is intended to be easy to use, with minimal setup required.

## Usage

> Warning:
> - This library is only compatible with Linux and requires an X11 server.
> - This is still a work in progress, so the API can change.

You can use this library by adding the following line to your `Cargo.toml` file:
```toml
[dependencies]
xoverlay = { git = "https://github.com/VincentSidot/xoverlay.git", branch = "master"}
```

Currently, the library is not published on crates.io, so you need to use the Git repository.



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

### Overlay

The overlay module is the main module of the library, it is the entry point of the library.

There are two ways to create an overlay:
- With the `init` method, it will create a new overlay with the given window id, and the given mapping.
- With the `init_with_name` method, it will create a new overlay with the given window name, and the given mapping.

The overlay is the main struct of the library, it is used to create the window, and to handle the event loop.

The overlay is composed of two windows:
- The parent window: The window on which the overlay is displayed
- The overlay window: The window that is displayed on top of the parent window

Shapes are added to the overlay window, with the `add_shape` method. (`add_shapes` method is also available to add multiple shapes at once)

Shapes can be drawn manually with the `draw` method, or automatically with the `event_loop` method.

The `event_loop` method is used to handle the event of the overlay window.

> Note: The `event_loop` method take a closure that take two arguments:
> - The overlay itself
> - The event
>
> After the event loop is finished, the overlay is freed. (Currently not crash safe)


### Event

The event module is used to handle the event of the overlay.

The following events are handled:
- ***ParentResize***: Triggered when the parent window is resized
- ***MousePress***: Triggered when a mouse button is pressed (inside the parent window)
- ***MouseMotion***: Triggered when the mouse is moved (inside the parent window)
- ***KeyPress***: Triggered when a key is pressed (while the overlay window is focused)
- ***KeyRelease***: Triggered when a key is released (while the overlay window is focused)
- ***Redraw***: Triggered when the overlay need to be redrawn
- ***StopEventLoop***: Triggered when the event loop need to be stopped
- ***Nothing***: Triggered a previous event is partially handled (ex: MousePress outside the overlay window)
- ***Unknown***: Triggered when a not handled event is received

### Shape

The shape module is used to create the shape of the overlay window.

The following shapes are handled:
- ***Rectangle***: A rectangle shape
- ***Arc***: An arc shape (partial circle)
    - ***Circle***: A circle shape (special case of arc)

### Coord

The coord module is used to handle the coordinate of the shape.

The following structures are implemented:
- ***Pos***: Represent a position in the window (position are f32 between 0.0 and 1.0)
- ***Size***: Represent a size in the window (size are f32 between 0.0 and 1.0)
- ***Anchor***: Represent an anchor point in the window
    - NorthWest (default anchor), North, NorthEast, West, Center, East, SouthWest, South, SouthEast, Custom(x, y)

### Color

The color module is used to handle the color of the shape.

### Key

The key module is used to handle the key event.

> Note: Currently only the arrow keys are handled. (Later I will add more keys)

### Export of x11rb

x11rb crate is re-exported in the library, to allow to use the x11rb crate directly.

x11rb is a low level X11 library, used to interact with the X server.

## Next steps

Those are the next steps for the library (the list is not exhaustive and not in order):

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
- Create github action for CI (build, test, etc.)
- Improve the error handling

## Built With

* [x11rb](https://crates.io/crates/x11rb) - The X11 library used


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
