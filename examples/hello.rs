use x11rb::rust_connection::RustConnection;
use xoverlay::{
    Color,
    Mapping,
    Overlay,
    shape::{
        Rectangle,
        Arc,
        Shape
    }
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
        Rectangle::percent(
            0.0, 0.0,
            0.2, 1.0, 
            Color::RED,
            &overlay
        )?,
        Rectangle::percent(
            0.8, 0.0,
            0.2, 1.0, 
            Color::BLUE,
            &overlay
        )?,
        Arc::circle_percent(
            0.45, 0.45,
            0.1,
            Color::GREEN,
            &overlay
        )?,
        Arc::circle_percent(
            0.45, 0.45,
            0.09,
            Color::TRANSPARENT,
            &overlay
        )?,
    ];

    // Add the rectangles to the overlay
    overlay
        .add_shapes(rec)
        .draw()?;
    
    // Draw the overlay
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}