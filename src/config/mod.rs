use crate::drawing::error::DrawError;
use std::collections::HashMap;
pub type StyleMap = HashMap<String, SlideStyle>;
pub type Decorations = Vec<(Rectange<f64>, usize)>;

pub struct Config<'a> {
    pub doc_name: &'a str,
    pub colors: Vec<Color>,
    pub fg_idx: usize,
    pub bg_idx: usize,
    pub slide_styles: StyleMap,
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
    pub content: Vec<Rectange<f64>>,
}

/// color struct with rgba values
/// (red, green, blue, alpha)
#[derive(Clone, Copy)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);

pub struct Rectange<T> {
    /// original point from the top-left
    pub orig: Point<T>,
    /// the size of the rectangle relative to the orig Point
    pub size: Point<T>,
}

/// a simple representation of an Rectange with 2 points
pub struct Point<T>(pub T, pub T);
