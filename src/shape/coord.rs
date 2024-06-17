//! Module for handling coordinates and sizes
//! 
//! This module defines the `Coord` and `Size` types, which represent coordinates and sizes as a percentage of the drawable's size.
//! 
//! The `CoordExt` and `SizeExt` traits are implemented for `Coord` and `Size` respectively, providing additional methods for working with coordinates and sizes.
//! 
//! The `Anchor` enum is used to define the anchor point of a shape. It is also implemented for `Coord` and `Size` to calculate the position of a shape.
//! 
//! # Usage
//! 
//! This module can be used to handle coordinates and sizes in a graphical application. It provides a convenient way to represent and manipulate positions and dimensions as a percentage of the drawable's size.
//! 
//! The `Coord` and `Size` types can be used to store and manipulate coordinates and sizes, while the `CoordExt` and `SizeExt` traits provide additional methods for working with coordinates and sizes.
//! 
//! The `Anchor` enum can be used to define the anchor point of a shape, and it is also implemented for `Coord` and `Size` to calculate the position of a shape based on the anchor point.
//! 
//! # Examples
//! 
//! ```
//! use crate::xoverlay::shape::coord::{Coord, CoordExt, Size, SizeExt, Anchor};
//! 
//! let size = Size::new(100.0, 100.0);
//! let anchor = Anchor::Center;
//! let coord = Coord::from_anchor(&anchor, &size);
//! 
//! assert_eq!(coord, Coord::new(50.0, 50.0));
//! ```
//! 
//! # Notes
//! 
//! - The `Coord` and `Size` types are represented as a percentage of the drawable's size, where (0.0, 0.0) represents the top left corner and (100.0, 100.0) represents the bottom right corner.
//! - The `Anchor` enum provides predefined anchor points for positioning shapes, such as `Center`, `North`, `SouthEast`, etc. It also allows for custom anchor points using the `Custom` variant.
//! - The `CoordExt` and `SizeExt` traits provide methods for converting between percentage-based coordinates/sizes and real coordinates/sizes based on the drawable's size.
//! - The `CoordExt` trait provides methods for calculating the top left coordinate of a shape based on an anchor point, as well as converting a percentage-based coordinate to a real coordinate.
//! - The `SizeExt` trait provides a method for converting a percentage-based size to a real size based on the drawable's size.
//! - The `Anchor` enum provides a method to calculate the delta for an anchor point, which represents the offset from the top left corner of a shape.
//! - The `Anchor` enum is implemented for `Coord` and `Size` to calculate the position of a shape based on the anchor point.
//! 
//! # Safety
//! 
//! This module does not contain any unsafe code.


use crate::math::vec::{Vec2, Vec2f};

/// Represents a coordinate as a percentage of the drawable's size
pub type Coord = Vec2f;
pub type Size = Vec2f;

/// Extension trait for `Coord`
pub trait CoordExt {
    fn from_anchor(anchor: &Anchor, size: &Size) -> Self;
    fn top_left(&self, anchor: &Anchor, size: &Size) -> Self;
    fn to_real_coord<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self;
}

impl CoordExt for Coord {
    /// Returns a new `Coord` based on the given `Anchor` and `Size`
    /// 
    /// # Arguments
    /// 
    /// * `anchor` - The anchor point
    /// * `size` - The size of the shape
    /// 
    /// # Returns
    /// 
    /// A new `Coord` representing the position of the anchor point
    /// 
    fn from_anchor(anchor: &Anchor, size: &Size) -> Self {
        let (dx, dy) = anchor.delta(size.x(), size.y());
        Coord::new(dx, dy)
    }

    /// Returns the top left `Coord` based on the given `Anchor` and `Size`
    /// 
    /// # Arguments
    /// 
    /// * `anchor` - The anchor point
    /// * `size` - The size of the shape
    /// 
    /// # Returns
    /// 
    /// A new `Coord` representing the top left corner of the shape
    /// 
    fn top_left(&self, anchor: &Anchor, size: &Size) -> Self {
        let delta = Self::from_anchor(anchor, size);
        *self - delta
    }

    /// Converts the `Coord` to a real coordinate based on the given `Size`
    /// 
    /// # Arguments
    /// 
    /// * `size` - The size of the drawable
    /// 
    /// # Returns
    /// 
    /// A new `Coord` representing the real coordinate based on the drawable size
    /// 
    fn to_real_coord<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self {
        Coord::new(self.x * size.x.into(), self.y * size.y.into())
    }
}

/// Extension trait for `Size`
pub trait SizeExt {
    fn to_real_size<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self;
}

impl SizeExt for Size {
    /// Converts the `Size` to a real size based on the given window `Size`
    /// 
    /// # Arguments
    /// 
    /// * `size` - The window size
    /// 
    /// # Returns
    /// 
    /// A new `Size` representing the real size based on the window size
    /// 
    fn to_real_size<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self {
        Size::new(self.x * size.x.into(), self.y * size.y.into())
    }
}

#[derive(Debug, PartialEq)]
pub enum Anchor {
    Center,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    /// Custom anchor, position is relative to the top left corner
    Custom(f32, f32),
}

impl Anchor {
    /// Returns the delta for the anchor
    /// regarding the top left corner of the shape
    /// (top left is equivalent to NorthWest anchor)
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the shape
    /// * `height` - The height of the shape
    ///
    /// # Returns
    ///
    /// A tuple containing the delta values for the anchor.
    /// The first value represents the horizontal delta,
    /// and the second value represents the vertical delta.
    ///
    fn delta(&self, width: f32, height: f32) -> (f32, f32) {
        match self {
            Self::NorthWest => (0.0, 0.0),
            Self::North => (width / 2.0, 0.0),
            Self::NorthEast => (width, 0.0),
            Self::East => (width, height / 2.0),
            Self::SouthEast => (width, height),
            Self::South => (width / 2.0, height),
            Self::SouthWest => (0.0, height),
            Self::West => (0.0, height / 2.0),
            Self::Center => (width / 2.0, height / 2.0),
            Self::Custom(x, y) => (*x, *y),
        }
    }
}

/// Default anchor is NorthWest (top left corner)
impl Default for Anchor {
    fn default() -> Self {
        Self::NorthWest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_from_anchor() {
        let size = Size::new(1.0, 1.0);
        let anchor = Anchor::Center;
        let coord = Coord::from_anchor(&anchor, &size);

        assert_eq!(coord, Coord::new(0.5, 0.5));
    }

    #[test]
    fn test_coord_top_left() {
        let size = Size::new(1.0, 1.0);
        let anchor = Anchor::Center;
        let coord = Coord::new(0.5, 0.5);
        let top_left = coord.top_left(&anchor, &size);

        assert_eq!(top_left, Coord::new(0.0, 0.0));
    }

    #[test]
    fn test_coord_to_real_coord() {
        let size = Size::new(1.0, 1.0);
        let coord = Coord::new(0.5, 0.5);
        let real_coord = coord.to_real_coord(size);

        assert_eq!(real_coord, Coord::new(0.5, 0.5));
    }

    #[test]
    fn test_size_to_real_size() {
        let size = Size::new(1.0, 1.0);
        let window_size = Vec2::<u16>::new(800, 600);
        let real_size = size.to_real_size(window_size);

        assert_eq!(real_size, Size::new(800.0, 600.0));
    }

    #[test]
    fn test_anchor_delta() {
        let anchor = Anchor::NorthWest;
        let width = 1.0;
        let height = 0.5;

        let delta = anchor.delta(width, height);

        assert_eq!(delta, (0.0, 0.0));
    }

    #[test]
    fn test_default_anchor() {
        let default_anchor = Anchor::default();

        assert_eq!(default_anchor, Anchor::NorthWest);
    }
}