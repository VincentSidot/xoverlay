use x11rb::protocol::xproto::KeyButMask;
use xoverlay::{
    event::Event,
    key::{Key, KeyRef},
    shape::{
        coord::{Anchor, Coord, Size},
        Rectangle,
    },
    Color, Drawable, Mapping, Overlay,
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

    // Create rectangles
    let rec = Rectangle::fill(
        Anchor::Center,
        Coord::new(0.5, 0.5),
        Size::new(0.1, 0.1),
        Color::BLUE,
    )?;

    // let to_draw : Vec<&dyn Shape<RustConnection>> = vec![
    //     &rec,
    //     // Rectangle::fill(
    //     //     Anchor::NorthEast,
    //     //     Coord::new(1.0, 0.0),
    //     //     Size::new(0.2, 1.0),
    //     //     Color::RED,
    //     // )?,
    // ];

    // Add the rectangles to the overlay
    overlay.add_shape(rec.clone()).event_loop(|_, event| {
        match event {
            Event::MouseMotion { coord } => {
                let mut rec = rec.borrow_mut();
                rec.set_position(coord);
                Some(Event::Redraw)
            }
            _ => {
                // Print the event
                // println!("Event: {:?}", event);
                None
            }
        }
    })?;

    // loop {
    //     let (event, seq) = overlay.conn.wait_for_event_with_sequence()?;
    //     println!("[{}] Event: {:?}", seq, event);
    // }

    Ok(())
}
