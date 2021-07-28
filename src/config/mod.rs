use crate::drawing::error::DrawError;
use std::{collections::HashMap, ops::Add};
pub type StyleMap = HashMap<String, SlideStyle>;
pub type Decorations = Vec<(Rectangle<f64>, usize)>;
// TODO: add line spacing
pub type Contents = Vec<(Rectangle<f64>, f32)>;

pub struct Config<'a> {
    pub doc_name: &'a str,
    pub colors: Vec<Color>,
    pub fg_idx: usize,
    pub bg_idx: usize,
    pub slide_styles: StyleMap,
    // TODO: add support for different fonts
    pub font: String,
}

impl<'a> Config<'a> {
    pub fn get_color(&self, idx: usize) -> Result<Color, DrawError> {
        self.colors
            .get(idx)
            .ok_or_else(|| DrawError::NoColor(idx))
            .map(|c| *c)
    }
}

pub struct SlideStyle {
    /// a decoration for the slides
    /// draws a simple rectangle at the given position(item0) with the color from the index
    pub decorations: Decorations,
    /// an area were content can appear
    pub content: Contents,
}

/// color struct with rgba values
/// (red, green, blue, alpha)
#[derive(Clone, Copy)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);

/// a simple 2d point with both coords going from the top-left
#[derive(Clone, Copy)]
pub struct Point<T>(pub T, pub T);

impl<T> From<Point<T>> for (T, T) {
    fn from(p: Point<T>) -> Self {
        (p.0, p.1)
    }
}

/// a simple representation of an Rectange with 2 points
pub struct Rectangle<T> {
    /// original point from the top-left
    pub orig: Point<T>,
    /// the size of the rectangle relative to the orig Point
    pub size: Point<T>,
}

impl<T> Rectangle<T> {
    pub fn points(&'_ self) -> RectPoints<'_, T> {
        RectPoints {
            rect: self,
            point: 0,
        }
    }
}

pub struct RectPoints<'a, T> {
    rect: &'a Rectangle<T>,
    point: u8,
}

impl<'a, T> Iterator for RectPoints<'a, T>
where
    T: Clone + Copy + Add<T, Output = T>,
{
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let orig = &self.rect.orig;
        let size = &self.rect.size;

        self.point += 1;

        match self.point {
            1 => Some(*orig),
            2 => Some(Point(orig.0 + size.0, orig.1)),
            3 => Some(Point(orig.0, orig.1 + size.1)),
            4 => Some(Point(orig.0 + size.0, orig.1 + size.1)),
            _ => None,
        }
    }
}
