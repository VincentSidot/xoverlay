use crate::math::vec::{
    Vec2f,
    Vec2
};

/// Represents a coordinate as a percentage of the drawable's size
pub type Coord = Vec2f;
pub type Size = Vec2f;

pub trait CoordExt {
    fn from_anchor(anchor: &Anchor, size: &Size) -> Self;
    fn top_left(&self, anchor: &Anchor, size: &Size) -> Self;
    fn to_real_coord<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self;
}

impl CoordExt for Coord {
    fn from_anchor(anchor: &Anchor, size: &Size) -> Self {
        let (dx, dy) = anchor.delta(size.x(), size.y());
        Coord::new(dx, dy)
    }
    
    fn top_left(&self, anchor: &Anchor, size: &Size) -> Self  {
        let delta = Self::from_anchor(anchor, size);
        *self - delta
    }
    
    fn to_real_coord<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self {
        Coord::new(
            self.x * size.x.into(),
            self.y * size.y.into()
        )
    }
}

pub trait SizeExt {

    fn to_real_size<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self;

}

impl SizeExt for Size {
    fn to_real_size<C: Into<f32> + Copy>(&self, size: Vec2<C>) -> Self {
        Size::new(
            self.x * size.x.into(),
            self.y * size.y.into()
        )
    }
}

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

impl Default for Anchor {
    fn default() -> Self {
        Self::NorthWest
    }
}