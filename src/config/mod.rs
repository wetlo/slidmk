use std::collections::HashMap;
pub type StyleMap = HashMap<String, SlideStyle>;
pub type Decorations = Vec<(Rectange, usize)>;

pub struct Config<'a> {
    pub doc_name: &'a str,
    pub colors: Vec<Color>,
    pub fg_idx: usize,
    pub bg_idx: usize,
    pub slide_styles: StyleMap,
}

pub struct SlideStyle {
    /// a decoration for the slides
    /// draws a simple rectangle at the given position(item0) with the color from the index
    decorations: Decorations,
    /// an area were content can appear
    content: Vec<Rectange>,
}

/// color struct with rgba values
/// (red, green, blue, alpha)
pub struct Color(pub f32, pub f32, pub f32, pub f32);

pub struct Rectange {
    /// original point from the top-left
    orig: Point,
    /// the size of the rectangle relative to the orig Point
    size: Point,
}

/// a simple representation of an Rectange with 2 points
pub struct Point(u32, u32);
