use x11rb::protocol::xproto::KeyButMask;

#[derive(Debug, PartialEq)]
pub enum KeyRef {
    ArrowUp = 111,
    ArrowRight = 114,
    ArrowDown = 116,
    ArrowLeft = 113,

    Unkown = 0,
}

#[derive(Debug, PartialEq)]
pub struct Key {
    pub key: KeyRef,
    pub modifiers: KeyButMask,
}

impl Key {
    pub fn from_xorg_raw(detail: u8, modifiers: KeyButMask) -> Self {
        // Compute key
        let key = match detail {
            111 => KeyRef::ArrowUp,
            114 => KeyRef::ArrowRight,
            116 => KeyRef::ArrowDown,
            113 => KeyRef::ArrowLeft,
            _ => KeyRef::Unkown,
        };
        Self { key, modifiers }
    }
}
