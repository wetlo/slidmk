use serde_derive::Deserialize;
use std::ops;

/// color struct with rgba values
/// (red, green, blue, alpha)
#[derive(Clone, Copy, Debug)]
//pub struct Color(pub f64, pub f64, pub f64, pub f64);
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

/// a simple 2d point with both coords going from the top-left
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn map<F, U>(self, mut f: F) -> Point<U>
    where
        F: FnMut(T) -> U,
    {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(p: Point<T>) -> Self {
        (p.x, p.y)
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<U, T: ops::Add<Output = U>> ops::Add for Point<T> {
    type Output = Point<U>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: ops::AddAssign> ops::AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<U, T: ops::Sub<Output = U>> ops::Sub for Point<T> {
    type Output = Point<U>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
/// a simple representation of an Rectangle with 2 points
pub struct Rectangle<T> {
    /// original point from the top-left
    pub orig: Point<T>,
    /// the size of the rectangle relative to the orig Point
    pub size: Point<T>,
}

impl<T> Rectangle<T>
where
    T: PartialOrd + ops::Add<Output = T> + Copy,
{
    /// checks whether the other rectangle is inside this one
    pub fn is_inside_inclusive(&self, other: Point<T>) -> bool {
        self.orig <= other && self.orig + self.size >= other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VertOrientation {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HorOrientation {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Orientation {
    pub vertical: VertOrientation,
    pub horizontal: HorOrientation,
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            vertical: VertOrientation::Top,
            horizontal: HorOrientation::Left,
        }
    }
}
