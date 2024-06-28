//! Contains the Vec2 struct and its implementations
//! 
//! This module contains the Vec2 struct and its implementations.
//! 
//! Vec2 is a generic 2D vector struct that can be used to represent points or directions in a 2D space.
//! 
//! It can be used with any type that implements the basic arithmetic operations (Add, Sub, Mul, Div, Neg).

#![allow(dead_code)]
use std::ops;

/// Represents a 2D vector
/// 
/// Vec2 is a generic 2D vector struct that can be used to represent points or directions in a 2D space.
/// 
/// The struct is generic over the type of the components (x and y).
/// 
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// Type aliases for Vec2
pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;


/// Implements basic operations for Vec2
impl<T> Vec2<T>
{
    /// Returns the x component of the Vec2
    pub fn x(&self) -> T 
    where
        T: Copy,
    {
        self.x
    }

    /// Returns the y component of the Vec2
    pub fn y(&self) -> T
    where
        T: Copy,
    {
        self.y
    }

    /// Returns a new Vec2 with the given x and y components
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    // Convert the Vec2 to another type
    pub fn convert<U>(&self) -> Vec2<U>
    where
        T: Copy,
        U: From<T>,
    
    {
        Vec2 {
            x: U::from(self.x),
            y: U::from(self.y),
        }
    }

    /// Returns a new Vec2 with the x component set to the given value
    pub fn with_x(&self, x: T) -> Self
    where
        T: Copy,
    {
        Vec2 { x, y: self.y }
    }

    /// Returns a new Vec2 with the y component set to the given value
    pub fn with_y(&self, y: T) -> Self
    where
        T: Copy,
    {
        Vec2 { x: self.x, y }
    }


    /// Returns the dot product of two Vec2
    pub fn dot(&self, rhs: Vec2<T>) -> T
    where
        T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy,
    {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Returns the euclidean length of the Vec2
    pub fn length(&self) -> f32
    where
        T: Into<f32> + Copy,
    {
        (self.x.into().powi(2) + self.y.into().powi(2)).sqrt()
    }

    /// Returns the squared euclidean length of the Vec2 (faster than length)
    pub fn fast_length(&self) -> f32
    where
        T: Into<f32> + Copy,
    {
        let x = self.x.into();
        let y = self.y.into();
        x * x + y * y
    }

    /// Returns the normalized Vec2 (length = 1)
    pub fn normalize(&self) -> Vec2<f32>
    where
        T: Into<f32> + Copy,
    {
        let length = self.length();
        Vec2 {
            x: self.x.into() / length,
            y: self.y.into() / length,
        }
    }

    /// Return the hammard product of two Vec2
    pub fn hammard(&self, rhs: Vec2<T>) -> Vec2<T>
    where
        T: ops::Mul<Output = T> + Copy,
    {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }

    /// Return the inverse hammard product of two Vec2
    pub fn inv_hammard(&self, rhs: Vec2<T>) -> Vec2<T>
    where
        T: ops::Div<Output = T> + Copy,
    {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }

}

/// Implements the Display trait for Vec2
impl<T> std::fmt::Display for Vec2<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Implements the Add trait for Vec2
impl<T> ops::Add<Vec2<T>> for Vec2<T>
where
    T: ops::Add<Output = T> + Copy,
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
where
    T: ops::Sub<Output = T> + Copy,
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
where
    T: ops::Mul<Output = T> + Copy,
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
where
    T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy,
{
    type Output = T;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

/// Implements the Scalar Div trait for Vec2
impl<T> ops::Div<T> for Vec2<T>
where
    T: ops::Div<Output = T> + Copy,
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
where
    T: ops::Neg<Output = T> + Copy,
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
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Implements From trait for Vec2
impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vec2 { x, y }
    }
}
impl<T> From<[T; 2]> for Vec2<T>
where
    T: Copy,
{
    fn from(value: [T; 2]) -> Self {
        Vec2 {
            x: value[0],
            y: value[1],
        }
    }
}

// Implement convertion to tuple
impl<T> From<Vec2<T>> for (T, T)
where
    T: Copy,
{
    fn from(vec: Vec2<T>) -> Self {
        (vec.x, vec.y)
    }
}

impl<T> From<Vec2<T>> for [T; 2]
where
    T: Copy,
{
    fn from(vec: Vec2<T>) -> Self {
        [vec.x, vec.y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let vec1 = Vec2::new(1, 2);
        let vec2 = Vec2::new(3, 4);
        let vec3 = Vec2::new(0, 0);
        let result = vec1 + vec2 + vec3;
        assert_eq!(result.x(), 4);
        assert_eq!(result.y(), 6);
    }

    #[test]
    fn test_subtraction() {
        let vec1 = Vec2::new(5, 6);
        let vec2 = Vec2::new(2, 3);
        let result = vec1 - vec2;
        assert_eq!(result.x(), 3);
        assert_eq!(result.y(), 3);
    }

    #[test]
    fn test_scalar_multiplication() {
        let vec = Vec2::new(2, 3);
        let scalar = 2;
        let result = vec * scalar;
        assert_eq!(result.x(), 4);
        assert_eq!(result.y(), 6);
    }

    #[test]
    fn test_dot_product() {
        let vec1 = Vec2::new(1, 2);
        let vec2 = Vec2::new(3, 4);
        let result = vec1 * vec2;
        
        assert_eq!(result, 11);
    }

    #[test]
    fn test_scalar_division() {
        let vec = Vec2::new(4, 6);
        let scalar = 2;
        let result = vec / scalar;
        assert_eq!(result.x(), 2);
        assert_eq!(result.y(), 3);
    }

    #[test]
    fn test_negation() {
        let vec = Vec2::new(2, 3);
        let result = -vec;
        assert_eq!(result.x(), -2);
        assert_eq!(result.y(), -3);
    }

    #[test]
    fn test_equality() {
        let vec1 = Vec2::new(1, 2);
        let vec2 = Vec2::new(1, 2);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_from_tuple() {
        let tuple = (3, 4);
        let vec: Vec2<i32> = tuple.into();
        assert_eq!(vec.x(), 3);
        assert_eq!(vec.y(), 4);
    }

    #[test]
    fn test_from_array() {
        let array = [5, 6];
        let vec: Vec2<i32> = array.into();
        assert_eq!(vec.x(), 5);
        assert_eq!(vec.y(), 6);
    }

    #[test]
    fn test_to_tuple() {
        let vec = Vec2::new(3, 4);
        let tuple: (i32, i32) = vec.into();
        assert_eq!(tuple, (3, 4));
    }

    #[test]
    fn test_to_array() {
        let vec = Vec2::new(5, 6);
        let array: [i32; 2] = vec.into();
        assert_eq!(array, [5, 6]);
    }

    #[test]
    fn test_length() {
        let vec = Vec2f::new(3.0, 4.0);
        let length = vec.length();
        assert_eq!(length, 5.0);
    }

    #[test]
    fn test_fast_length() {
        let vec = Vec2f::new(3.0, 4.0);
        let length = vec.fast_length();
        assert_eq!(length, 25.0);
    }

    #[test]
    fn test_normalize() {
        let vec = Vec2f::new(3.0, 4.0);
        let normalized = vec.normalize();
        assert_eq!(normalized.x(), 0.6);
        assert_eq!(normalized.y(), 0.8);
    }
}