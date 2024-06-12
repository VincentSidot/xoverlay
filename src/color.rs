fn to_rgba(value: u32) -> u32 {
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = (value) & 0xFF;
    let a = 0xFF;

    a << 24 | r << 16 | g << 8 | b
}

fn to_rgb(value: u32) -> u32 {
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = value & 0xFF;

    (r << 16) | (g << 8) | b
}

fn for_depth(value: u32, depth: u8) -> u32 {
    match depth {
        32 => to_rgba(value),
        24 => to_rgb(value),
        16 => value >> 16,
        8 => value >> 24,
        1 => {
            if value > 0 {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

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

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::RGB(r, g, b)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::RGBA(r, g, b, a)
    }

    pub fn value(&self, depth: u8) -> u32 {
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

    pub fn with_alpha(&self, alpha: u8) -> Self {
        let raw = self.value(24);
        let r = (raw >> 16) & 0xFF;
        let g = (raw >> 8) & 0xFF;
        let b = raw & 0xFF;

        Color::rgba(r as u8, g as u8, b as u8, alpha)
    }
}
