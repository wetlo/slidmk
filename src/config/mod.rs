use crate::drawing::error::DrawError;
use std::{collections::HashMap, ops::Add};
pub type StyleMap = HashMap<String, SlideStyle>;

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
#[derive(Clone, Copy, Debug)]
pub struct Point<T>(pub T, pub T);

impl<T> From<Point<T>> for (T, T) {
    fn from(p: Point<T>) -> Self {
        (p.0, p.1)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RectPoints<'a, T> {
    rect: &'a Rectangle<T>,
    point: u8,
}

#[derive(Debug)]
pub struct Decoration {
    pub area: Rectangle<f64>,
    pub color_idx: usize,
}

#[derive(Debug)]
pub struct Content {
    pub area: Rectangle<f64>,
    // TODO: add line spacing
    pub font_size: f32,
}

#[derive(Debug)]
pub struct SlideStyle {
    /// a decoration for the slides
    /// draws a simple rectangle at the given position(item0) with the color from the index
    pub decorations: Vec<Decoration>,
    /// an area were content can appear
    pub content: Vec<Content>,
}

#[derive(Debug)]
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
            .ok_or(DrawError::NoColor(idx))
            .map(|c| *c)
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            colors: vec![
                Color::new(0.0, 0.0, 0.0),
                Color::new(1.0, 0.0, 0.0),
                Color::new(0.0, 1.0, 1.0),
            ],
            doc_name: "hello world",
            slide_styles: crate::map! {
                "Title" => SlideStyle {
                    decorations: vec![],
                    content: vec![
                        Content {
                            area: Rectangle {
                                orig: Point(0.0, 0.0),
                                size: Point(1.0, 0.8) },
                                font_size: 36.0,
                        },
                        Content {
                            area: Rectangle {
                                orig: Point(0.0, 0.8),
                                size: Point(1.0, 0.2) },
                                font_size: 18.0,
                        },
                    ],
                },

                "Head_Cont" => SlideStyle {
                    decorations: vec![],
                    content: vec![
                        Content {
                            area: Rectangle {
                            orig: Point(0.0, 0.0),
                            size: Point(1.0, 0.3),
                        },
                            font_size: 24.0,
                        },
                        Content {
                            area: Rectangle {
                            orig: Point(0.0, 0.3),
                            size: Point(1.0, 0.7),
                        },
                            font_size: 18.0,
                        },
                    ],
                },

                "Vert_Split" => SlideStyle {
                    decorations: vec![],
                    content: vec![
                        Content {
                            area: Rectangle {
                            orig: Point(0.0, 0.0),
                            size: Point(0.5, 0.3),
                        },
                            font_size: 24.0,
                        },
                        Content {
                            area: Rectangle {
                            orig: Point(0.0, 0.3),
                            size: Point(0.5, 0.7),
                        },
                            font_size: 18.0,
                        },
                        Content {
                            area: Rectangle {
                            orig: Point(0.5, 0.0),
                            size: Point(0.5, 0.3),
                        },
                            font_size: 24.0,
                        },
                        Content {
                            area: Rectangle {
                            orig: Point(0.0, 0.3),
                            size: Point(0.5, 0.7),
                        },
                            font_size: 18.0,
                        },
                    ],
                },
                "Two_Hor" => SlideStyle {
                    decorations: vec![],
                    content: vec![
                        Content {
                            area: Rectangle {
                            orig: Point(0.0, 0.0),
                            size: Point(1.0, 0.5),
                        },
                            font_size: 20.0,
                        },
                        Content {
                            area: Rectangle {
                            orig: Point(0.0, 0.5),
                            size: Point(1.0, 0.5),
                        },
                            font_size: 20.0,
                        },
                    ],
                },
            },
            fg_idx: 0,
            bg_idx: 0,
            font: String::from("monospace"),
        }
    }
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
