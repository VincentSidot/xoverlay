use x11rb::rust_connection::RustConnection;


use xoverlay::{
    event::Event,
    shape::{
        coord::{Anchor, Coord, Size}, Arc, Rectangle, Shape
    }, Color, Mapping, Overlay
};

use std::{
    env,
    error::Error
};

fn main() -> Result<(), Box<dyn Error>>{
    
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

    println!("Window: {:#x}", window);

    // Initialize the overlay
    let mut overlay = Overlay::init(
        window,
        &Mapping::FullScreen, 
        None
    )?;

    // Create rectangles
    let rec: Vec<Box<dyn Shape<RustConnection>>> = vec![
        Rectangle::fill(
            Anchor::NorthWest,
            Coord::new(0.0, 0.0),
            Size::new(0.2, 1.0),
            Color::BLUE,
        )?,
        Arc::filled_circle(
            Anchor::Center,
            Coord::new(0.5, 0.5),
            0.5,
            Color::WHITE,
        )?,
        Rectangle::fill(
            Anchor::NorthEast,
            Coord::new(1.0, 0.0),
            Size::new(0.2, 1.0),
            Color::RED,
        )?,
    ];

    // Add the rectangles to the overlay
    overlay
        .add_shapes(rec)
        .event_loop(|_, event| {
            if event == Event::Unkown {
                return
            }
            // Print the event
            println!("Event: {:?}", event);
        })?;
    
    Ok(())

}