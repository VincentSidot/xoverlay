#![allow(dead_code)]
use std::ops;

/// Represents a 2D vector
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;

/// Implements the Add trait for Vec2
impl<T> ops::Add<Vec2<T>> for Vec2<T> 
where T:
    ops::Add<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Implements the Sub trait for Vec2
impl<T> ops::Sub<Vec2<T>> for Vec2<T> 
where T:
    ops::Sub<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Implements the Scalar Mul trait for Vec2
impl<T> ops::Mul<T> for Vec2<T> 
where T:
    ops::Mul<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Implements the Dot product for Vec2 (Vec2 * Vec2 -> Scalar)
impl<T> ops::Mul<Vec2<T>> for Vec2<T>
where T:
    ops::Mul<Output = T> + ops::Add<Output = T> + Copy
{
    type Output = T;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }

}

/// Implements the Scalar Div trait for Vec2
impl<T> ops::Div<T> for Vec2<T> 
where T:
    ops::Div<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// Implements the Neg trait for Vec2
impl<T> ops::Neg for Vec2<T> 
where T:
    ops::Neg<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Implements the PartialEq trait for Vec2
impl<T> PartialEq for Vec2<T> 
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Implements From trait for Vec2
impl<T> From<(T, T)> for Vec2<T>
{
    fn from((x, y): (T, T)) -> Self {
        Vec2 { x, y }
    }
}

impl<T> From<[T; 2]> for Vec2<T>
where
    T: Copy
{
    fn from(value: [T; 2]) -> Self {
        Vec2 { x: value[0], y: value[1] }
    }
}

// Implement convertion to tuple
impl<T> From<Vec2<T>> for (T, T)
where
    T: Copy
{
    fn from(vec: Vec2<T>) -> Self {
        (vec.x, vec.y)
    }
}

impl<T> From<Vec2<T>> for [T; 2]
where
    T: Copy
{
    fn from(vec: Vec2<T>) -> Self {
        [vec.x, vec.y]
    }
}

/// Implements basic operations for Vec2
impl<T> Vec2<T> 
where T: Copy
{
    /// Returns the x component of the Vec2
    pub fn x(&self) -> T {
        self.x
    }

    /// Returns the y component of the Vec2
    pub fn y(&self) -> T {
        self.y
    }

    /// Returns a new Vec2 with the given x and y components
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    /// Returns a new Vec2 with the x component set to the given value
    pub fn with_x(&self, x: T) -> Self {
        Vec2 { x, y: self.y }
    }

    /// Returns a new Vec2 with the y component set to the given value
    pub fn with_y(&self, y: T) -> Self {
        Vec2 { x: self.x, y }
    }
}

/// Implements the euclidean space operations for Vec2
impl<T> Vec2<T> 
where T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy
{
    /// Returns the dot product of two Vec2
    pub fn dot(&self, rhs: Vec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Returns the euclidean length of the Vec2
    pub fn length(&self) -> f32
    where T: Into<f32>
    {
        (self.x.into().powi(2) + self.y.into().powi(2)).sqrt()
    }

    /// Returns the squared euclidean length of the Vec2 (faster than length)
    pub fn fast_length(&self) -> f32
    where T: Into<f32>
    {
        let x = self.x.into();
        let y = self.y.into();
        x * x + y * y
    }

    /// Returns the normalized Vec2 (length = 1)
    pub fn normalize(&self) -> Vec2<f32>
    where T: Into<f32>
    {
        let length = self.length();
        Vec2 {
            x: self.x.into() / length,
            y: self.y.into() / length,
        }
    }
}
