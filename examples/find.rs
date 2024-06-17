
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

    if args.len() != 2 {
        eprintln!("Usage: {} <window name>", args[0]);
        return Err("No window provided")?;
    }

    let window_name = &args[1];

    // Initialize the overlay
    let mut overlay = Overlay::init_with_name(window_name, &Mapping::FullScreen, None)?;

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
        Anchor::default(),
        Coord::new(0.5, 0.7),
        Size::new(0.5, 1.0),
        color_tab[current_color],
    )?;

    // Add the rectangles to the overlay
    overlay.add_shape(rec.clone()).event_loop(|_, event| {
        match event {
            Event::MouseMotion { coord } => {
                let mut rec = rec.borrow_mut();
                rec.set_position(coord);
                Some(Event::Redraw)
            }
            Event::KeyPress(Key{
                key: KeyRef::ArrowUp
            }) => {
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