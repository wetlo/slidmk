use crate::drawing::error::DrawError;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops;
pub type StyleMap = HashMap<String, SlideStyle>;

mod de_se;

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

#[derive(Debug, Clone)]
pub enum VertOrientation {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone)]
pub enum HorOrientation {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone)]
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
    pub orientation: Orientation,
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
        let header_orientation = Orientation {
            vertical: VertOrientation::Bottom,
            horizontal: HorOrientation::Middle,
        };

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
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 1.0,y: 0.8} },
                            font_size: 36.0,
                            orientation: header_orientation.clone(),
                        },
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.8},
                                size: Point{x: 1.0,y: 0.2} },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                    ],
                },

                "Head_Cont" => SlideStyle {
                    decorations: vec![],
                    content: vec![
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 1.0,y: 0.3},
                            },
                            font_size: 24.0,
                            orientation: header_orientation.clone(),
                        },
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.3},
                                size: Point{x: 1.0,y: 0.7},
                            },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                    ],
                },

                "Vert_Split" => SlideStyle {
                    decorations: vec![],
                    content: vec![
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 0.5,y: 0.3},
                            },
                            font_size: 24.0,
                            orientation: header_orientation.clone(),
                        },
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.3},
                                size: Point{x: 0.5,y: 0.7},
                            },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.5,y: 0.0},
                                size: Point{x: 0.5,y: 0.3},
                            },
                            font_size: 24.0,
                            orientation: header_orientation,
                        },
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.5,y: 0.3},
                                size: Point{x: 0.5,y: 0.7},
                            },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                    ],
                },
                "Two_Hor" => SlideStyle {
                    decorations: vec![],
                    content: vec![
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 1.0,y: 0.5},
                            },
                            font_size: 20.0,
                            orientation: Orientation::default(),
                        },
                        Content {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.5},
                                size: Point{x: 1.0,y: 0.5},
                            },
                            font_size: 20.0,
                            orientation: Orientation::default(),
                        },
                    ],
                },
            },
            fg_idx: 0,
            bg_idx: 1,
            font: String::from("monospace"),
        }
    }
}
