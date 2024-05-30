use xoverlay::{
    event::Event,
    shape::{
        coord::{Anchor, Coord, Size}, Arc, Rectangle, Shape
    }, Color, Mapping, Overlay,
    x11rb::rust_connection::RustConnection
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
            Anchor::Center,
            Coord::new(0.25, 0.5),
            Size::new(0.25, 0.25),
            Color::BLUE,
        )?,
        Arc::filled_circle(
            Anchor::Center,
            Coord::new(0.25, 0.5),
            0.2,
            Color::TRANSPARENT,
        )?,
        // Rectangle::fill(
        //     Anchor::NorthEast,
        //     Coord::new(1.0, 0.0),
        //     Size::new(0.2, 1.0),
        //     Color::RED,
        // )?,
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

    // loop {
    //     let (event, seq) = overlay.conn.wait_for_event_with_sequence()?;
    //     println!("[{}] Event: {:?}", seq, event);
    // }
    
    Ok(())

}