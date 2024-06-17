//! Color module
//! 
//! This module is used to define the color for the shapes.
//! It defines the Color enum which contains the predefined colors and the RGB and RGBA colors.
//! 
//! - RGBA may not be handled correctly by the X11 server (Depends off the presence of the Composite extension).
//! The transparent color will use Shape extension to fake transparency.
//! 
//! 

/// Convert a u32 RGB value to a RGBA u32 value
/// 
/// # Arguments
/// 
/// * `value` - The u32 value to convert corresponding to the RGB value
/// 
/// # Returns
/// 
/// The function returns the RGBA u32 value with the alpha channel set to 0xFF.
/// 
fn to_rgba(value: u32) -> u32 {
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = (value) & 0xFF;
    let a = 0xFF;

    a << 24 | r << 16 | g << 8 | b
}

/// Convert a u32 RGBA value to a RGB u32 value
/// 
/// # Arguments
/// 
/// * `value` - The u32 value to convert corresponding to the RGBA value
/// 
/// # Returns
/// 
/// The function returns the RGB u32 value without the alpha channel.
///
fn to_rgb(value: u32) -> u32 {
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = value & 0xFF;

    (r << 16) | (g << 8) | b
}

/// Convert the RGBA value to the corresponding 16 bit color value.
/// 
/// 5 bit for red, 5 bit for green, 5 bit for blue, and 1 unused bit
/// 
/// # Arguments
/// 
/// * `value` - The u32 value to convert
/// 
/// # Returns
/// 
/// The function returns the value converted to 16 bit value.
/// 
fn to_16bit(value: u32) -> u32 {
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = value & 0xFF;

    // Apply the mask to the value 0111 1111 1111 1111
    (r >> 3) << 10 | (g >> 3) << 5 | (b >> 3) & 0x7FFF
}

/// Convert the RGBA value to the corresponding 8 bit color value.
/// 
/// 8bit color are grayscale
/// 
/// # Arguments
/// 
/// * `value` - The u32 value to convert
/// 
/// # Returns
/// 
/// The function returns the value converted to 8 bit value.
fn to_8bit(value: u32) -> u32 {
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = value & 0xFF;

    // Compute the grayscale value
    let gray = (r + g + b) / 3;

    gray & 0xFF
}

/// Convert the RGBA value to the corresponding 1 bit color value.
/// 
/// 1 bit color are black or white (it is used for shapes masking)
/// 
/// # Arguments
/// 
/// * `value` - The u32 value to convert
/// 
/// # Returns
/// 
/// The function returns the value converted to 1 bit value.
/// - 0 if the value is 0
/// - 1 if the value is greater than 0
/// 
fn to_1bit(value: u32) -> u32 {
    if value > 0 {
        1
    } else {
        0
    }
}

/// Get convert the RGBA value to the corresponding depth
/// 
/// # Arguments
/// 
/// * `value` - The u32 value to convert
/// * `depth` - The depth to convert the value to
/// 
/// # Returns
/// 
/// The function returns the value converted to the specified depth.
/// 
fn for_depth(value: u32, depth: &Depth) -> u32 {
    match depth {
        Depth::D32 => to_rgba(value),
        Depth::D24 => to_rgb(value),
        Depth::D16 => to_16bit(value),
        Depth::D8 => to_8bit(value),
        Depth::D1 => to_1bit(value),
    }
}

/// The Depth enum
/// 
/// This enum defines the depth for the color.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Depth{
    D1=1,
    D8=8,
    D16=16,
    D24=24,
    D32=32,
}

impl Depth {
    /// Create a new Depth from the given value
    /// 
    /// # Arguments
    /// 
    /// * `depth` - The depth value
    /// 
    /// # Returns
    /// 
    /// The function returns a new Depth enum with the given value.
    pub fn from<P>(depth: P) -> Self
    where
        P : Into<u8>
    {
        match depth.into() {
            1 => Depth::D1,
            8 => Depth::D8,
            16 => Depth::D16,
            24 => Depth::D24,
            _ => Depth::D32,
        }
    }

    /// Get the depth value
    /// 
    /// This method will return the depth value.
    /// 
    pub fn value<P>(&self) -> P
    where
        P : From<u8>
    {
        match self {
            Depth::D1 => 1.into(),
            Depth::D8 => 8.into(),
            Depth::D16 => 16.into(),
            Depth::D24 => 24.into(),
            Depth::D32 => 32.into(),
        }
    }
}

/// The Color enum
/// 
/// This enum defines the color for the shapes.
/// 
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    // A
    // B
    BLACK,
    BLUE,
    BROWN,
    // C
    CYAN,
    // D
    // E
    // F
    // G
    GRAY,
    GREEN,
    // H
    // I
    INDIGO,
    // J
    // K
    // L
    LIME,
    // M
    MAGENTA,
    // N
    NAVY,
    // O
    ORANGE,
    // P
    PINK,
    PURPLE,
    // Q
    // R
    RED,
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    // S
    SILVER,
    // T
    TRANSPARENT,
    // U
    // V
    // W
    WHITE,
    // X
    // Y
    YELLOW,
    // Z
}

/// Implementation of the Color enum
impl Color {

    /// Create a new color from the RGB values
    /// 
    /// # Arguments
    /// 
    /// * `r` - The red value
    /// * `g` - The green value
    /// * `b` - The blue value
    /// 
    /// # Returns
    /// 
    /// The function returns a new Color::RGB enum with given value.
    /// 
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::RGB(r, g, b)
    }

    /// Create a new color from the RGBA values
    /// 
    /// # Arguments
    /// 
    /// * `r` - The red value
    /// * `g` - The green value
    /// * `b` - The blue value
    /// * `a` - The alpha value
    /// 
    /// # Returns
    /// 
    /// The function returns a new Color::RGBA enum with given value.
    /// 
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::RGBA(r, g, b, a)
    }

    /// Get the value of the color at the specified depth
    /// 
    /// This method will return the value of the color at the specified depth.
    /// 
    /// # Arguments
    /// 
    /// * `depth` - The depth to get the value at
    /// 
    /// # Returns
    /// 
    /// The function returns the value of the color at the specified depth.
    pub fn value(&self, depth: &Depth) -> u32 {
        match self {
            Color::BLACK => for_depth(0x000000, depth),
            Color::BLUE => for_depth(0x0000FF, depth),
            Color::BROWN => for_depth(0xA52A2A, depth),
            Color::CYAN => for_depth(0x00FFFF, depth),
            Color::GRAY => for_depth(0x808080, depth),
            Color::GREEN => for_depth(0x008000, depth),
            Color::INDIGO => for_depth(0x4B0082, depth),
            Color::LIME => for_depth(0x00FF00, depth),
            Color::MAGENTA => for_depth(0xFF00FF, depth),
            Color::NAVY => for_depth(0x000080, depth),
            Color::ORANGE => for_depth(0xFFA500, depth),
            Color::PINK => for_depth(0xFFC0CB, depth),
            Color::PURPLE => for_depth(0x800080, depth),
            Color::RED => for_depth(0xFF0000, depth),
            Color::RGB(r, g, b) => ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32),
            Color::RGBA(r, g, b, a) => {
                ((*a as u32) << 24) | ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32)
            }
            Color::SILVER => for_depth(0xC0C0C0, depth),
            Color::TRANSPARENT => 0,
            Color::WHITE => for_depth(0xFFFFFF, depth),
            Color::YELLOW => for_depth(0xFFFF00, depth),
        }
    }

    /// Get the color with the given alpha value
    /// 
    /// This method will return the color with the given alpha value.
    /// 
    /// # Arguments
    /// 
    /// * `alpha` - The alpha value to set
    /// 
    /// # Returns
    /// 
    /// The function returns the color with the given alpha value.
    /// 
    pub fn with_alpha(&self, alpha: u8) -> Self {
        let raw = self.value(&Depth::D24);
        let r = (raw >> 16) & 0xFF;
        let g = (raw >> 8) & 0xFF;
        let b = raw & 0xFF;

        Color::new_rgba(r as u8, g as u8, b as u8, alpha)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use super::super::*;

        #[test]
        fn test_to_rgba() {
            assert_eq!(to_rgba(0x000000), 0xFF000000);
            assert_eq!(to_rgba(0x0000FF), 0xFF0000FF);
            assert_eq!(to_rgba(0xA52A2A), 0xFFA52A2A);
            assert_eq!(to_rgba(0x00FFFF), 0xFF00FFFF);
            assert_eq!(to_rgba(0x808080), 0xFF808080);
            assert_eq!(to_rgba(0x008000), 0xFF008000);
            assert_eq!(to_rgba(0x4B0082), 0xFF4B0082);
            assert_eq!(to_rgba(0x00FF00), 0xFF00FF00);
            assert_eq!(to_rgba(0xFF00FF), 0xFFFF00FF);
            assert_eq!(to_rgba(0x000080), 0xFF000080);
            assert_eq!(to_rgba(0xFFA500), 0xFFFFA500);
            assert_eq!(to_rgba(0xFFC0CB), 0xFFFFC0CB);
            assert_eq!(to_rgba(0x800080), 0xFF800080);
            assert_eq!(to_rgba(0xFF0000), 0xFFFF0000);
        }

        #[test]
        fn test_to_rgb() {
            assert_eq!(to_rgb(0xFF000000), 0x000000);
            assert_eq!(to_rgb(0xFF0000FF), 0x0000FF);
            assert_eq!(to_rgb(0xFFA52A2A), 0xA52A2A);
            assert_eq!(to_rgb(0xFF00FFFF), 0x00FFFF);
            assert_eq!(to_rgb(0xFF808080), 0x808080);
            assert_eq!(to_rgb(0xFF008000), 0x008000);
            assert_eq!(to_rgb(0xFF4B0082), 0x4B0082);
            assert_eq!(to_rgb(0xFF00FF00), 0x00FF00);
            assert_eq!(to_rgb(0xFFFF00FF), 0xFF00FF);
            assert_eq!(to_rgb(0xFF000080), 0x000080);
            assert_eq!(to_rgb(0xFFFFA500), 0xFFA500);
            assert_eq!(to_rgb(0xFFFFC0CB), 0xFFC0CB);
            assert_eq!(to_rgb(0xFF800080), 0x800080);
            assert_eq!(to_rgb(0xFFFF0000), 0xFF0000);
        }

        #[test]
        fn test_for_depth() {
            assert_eq!(for_depth(0x000000, &Depth::D32), 0xFF000000);
            assert_eq!(for_depth(0x0000FF, &Depth::D24), 0x0000FF);
            // 0xA52A2A = 0xA5 0x2A 0x2A
            // R: 0xA5 >> 3 = 0x14
            // G: 0x2A >> 3 = 0x5
            // B: 0x2A >> 3 = 0x5
            // 0 | R<<10 | G<<5 | B = 0x50A5
            assert_eq!(for_depth(0xA52A2A, &Depth::D16), 0x50A5);
            assert_eq!(for_depth(0x00FFFF, &Depth::D8), 0xAA);
            assert_eq!(for_depth(0x808080, &Depth::D1), 1);
        }

        #[test]
        fn test_depth_from() {
            assert_eq!(Depth::from(1), Depth::D1);
            assert_eq!(Depth::from(8), Depth::D8);
            assert_eq!(Depth::from(16), Depth::D16);
            assert_eq!(Depth::from(24), Depth::D24);
            assert_eq!(Depth::from(32), Depth::D32);
        }

        #[test]
        fn test_depth_value() {
            assert_eq!(Depth::D1.value::<u8>(), 1);
            assert_eq!(Depth::D8.value::<u8>(), 8);
            assert_eq!(Depth::D16.value::<u8>(), 16);
            assert_eq!(Depth::D24.value::<u8>(), 24);
            assert_eq!(Depth::D32.value::<u8>(), 32);
        }

        #[test]
        fn test_color_value() {
            assert_eq!(Color::BLACK.value(&Depth::D32), 0xFF000000);
            assert_eq!(Color::BLUE.value(&Depth::D24), 0x0000FF);
            assert_eq!(Color::BROWN.value(&Depth::D16), 0x50A5);
            assert_eq!(Color::CYAN.value(&Depth::D8), 0xAA);
            assert_eq!(Color::GRAY.value(&Depth::D1), 0x1);
        }

        #[test]
        fn test_color_with_alpha() {
            assert_eq!(Color::BLACK.with_alpha(0xFF), Color::new_rgba(0, 0, 0, 0xFF));
            assert_eq!(Color::BLUE.with_alpha(0x80), Color::new_rgba(0, 0, 0xFF, 0x80));
            assert_eq!(Color::BROWN.with_alpha(0x40), Color::new_rgba(0xA5, 0x2A, 0x2A, 0x40));
            assert_eq!(Color::CYAN.with_alpha(0x20), Color::new_rgba(0, 0xFF, 0xFF, 0x20));
            assert_eq!(Color::GRAY.with_alpha(0x10), Color::new_rgba(0x80, 0x80, 0x80, 0x10));
        }
    }
}
