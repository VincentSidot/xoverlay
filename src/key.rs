//! Key module
//! 
//! This module is used to define the key event

use std::marker::PhantomData;


const ARROW_UP: u8 = 111;
const ARROW_RIGHT: u8 = 114;
const ARROW_DOWN: u8 = 116;
const ARROW_LEFT: u8 = 113;

/// Key reference
/// 
/// This enum is used to define the key reference
/// 
/// Currently only the arrow keys are supported
#[derive(Debug, PartialEq)]
pub enum KeyRef {
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,

    Unkown(PhantomData<()>),
}

/// Implement the conversion from u8 to KeyRef
impl From<u8> for KeyRef {
    fn from(detail: u8) -> Self {
        match detail {
            ARROW_UP => Self::ArrowUp,
            ARROW_RIGHT => Self::ArrowRight,
            ARROW_DOWN => Self::ArrowDown,
            ARROW_LEFT => Self::ArrowLeft,
            _ => Self::Unkown(PhantomData),
        }
    }
}

/// Key object is used to define the key event
/// 
/// Currently key modifier are not supported
#[derive(Debug, PartialEq)]
pub struct Key(pub KeyRef);

/// Implement key object
impl Key {

    /// Create a key object from a raw xorg key value
    /// 
    /// # Arguments
    /// 
    /// * `detail` - The raw xorg key value
    /// 
    /// # Returns
    /// 
    /// The function returns a key object
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use xoverlay::key::Key;
    /// let key = Key::from_xorg_raw(111);
    /// assert_eq!(key.0, xoverlay::key::KeyRef::ArrowUp);
    /// let key = Key::from_xorg_raw(0);
    /// assert_eq!(key.0, xoverlay::key::KeyRef::Unkown(std::marker::PhantomData));
    /// ```
    /// 
    pub fn from_xorg_raw(detail: u8) -> Self {
        // Compute key
        let key = KeyRef::from(detail);
        Self(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from_xorg_raw() {
        // Test valid key values
        let key1 = Key::from_xorg_raw(ARROW_UP);
        assert_eq!(key1.0, KeyRef::ArrowUp);

        let key2 = Key::from_xorg_raw(ARROW_RIGHT);
        assert_eq!(key2.0, KeyRef::ArrowRight);

        let key3 = Key::from_xorg_raw(ARROW_DOWN);
        assert_eq!(key3.0, KeyRef::ArrowDown);

        let key4 = Key::from_xorg_raw(ARROW_LEFT);
        assert_eq!(key4.0, KeyRef::ArrowLeft);

        // Test invalid key value
        let key5 = Key::from_xorg_raw(0);
        assert_eq!(key5.0, KeyRef::Unkown(PhantomData));
    }

    #[test]
    fn test_keyref_from() {
        // Test valid key values
        let key1 = KeyRef::from(ARROW_UP);
        assert_eq!(key1, KeyRef::ArrowUp);

        let key2 = KeyRef::from(ARROW_RIGHT);
        assert_eq!(key2, KeyRef::ArrowRight);

        let key3 = KeyRef::from(ARROW_DOWN);
        assert_eq!(key3, KeyRef::ArrowDown);

        let key4 = KeyRef::from(ARROW_LEFT);
        assert_eq!(key4, KeyRef::ArrowLeft);

        // Test invalid key value
        let key5 = KeyRef::from(0);
        assert_eq!(key5, KeyRef::Unkown(PhantomData));
    }
}