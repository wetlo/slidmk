use crate::drawing::error::DrawError;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};
pub type StyleMap = HashMap<String, SlideTemplate>;

mod de_se;
mod primitives;

pub use primitives::*;

#[derive(Debug)]
pub struct Decoration {
    pub area: Rectangle<f64>,
    pub color_idx: usize,
}

#[derive(Debug)]
pub struct ContentTemplate {
    pub area: Rectangle<f64>,
    // TODO: add line spacing
    pub font_size: f32,
    pub orientation: Orientation,
}

#[derive(Debug)]
pub struct SlideTemplate {
    /// a decoration for the slides
    /// draws a simple rectangle at the given position(item0) with the color from the index
    pub decorations: Vec<Decoration>,
    /// an area were content can appear
    pub content: Vec<ContentTemplate>,
}

#[derive(Debug)]
pub struct PresentStyle {
    pub colors: Vec<Color>,
    pub font: String,
    line_spacing: f64,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub style: PresentStyle,
    pub margin: Rectangle<f64>,
    pub slide_styles: StyleMap,
    pub doc_name: &'a str,
}

fn get_reader<P: AsRef<Path>>(path: P) -> io::Result<io::BufReader<fs::File>> {
    let file = fs::File::open(path)?;

    Ok(io::BufReader::new(file))
}

impl<'a> Config<'a> {
    /// creates a config from a style and n template files
    pub fn from_files<P, Q>(templates: &[P], style: Q) -> Option<Self>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let style = {
            let r = get_reader(style).ok()?;
            let json: de_se::StyleJson = serde_hjson::from_reader(r).ok()?;
            PresentStyle::from(json)
        };

        let mut margin = None;
        let mut slide_templates = StyleMap::new();

        for path in templates {
            let r = get_reader(path).ok()?;
            let template: de_se::TemplateJson = serde_hjson::from_reader(r).ok()?;

            // use the first margin
            if margin.is_none() {
                margin = Some(template.margin);
            }

            let slides = template.slides.into_iter().map(|(k, v)| (k, v.into()));
            slide_templates.extend(slides);
        }

        Some(Self {
            style,
            doc_name: "todo",
            margin: margin.unwrap(),
            slide_styles: slide_templates,
        })
    }

    /// change the style to the one specified inside the path
    pub fn change_style<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let json: de_se::StyleJson = serde_hjson::from_reader(get_reader(path)?).unwrap();
        self.style = json.into();
        Ok(())
    }

    pub fn get_color(&self, idx: usize) -> Result<Color, DrawError> {
        self.style
            .colors
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
            doc_name: "default",
            style: PresentStyle {
                colors: vec![
                    Color::new(0.0, 0.0, 0.0),
                    Color::new(1.0, 0.0, 0.0),
                    Color::new(0.0, 1.0, 1.0),
                ],
                font: String::from("monospace"),
                line_spacing: 1.0,
            },
            margin: Rectangle {
                orig: Point { x: 0.05, y: 0.05 },
                size: Point { x: 0.9, y: 0.9 },
            },
            slide_styles: crate::map! {
                "Title" => SlideTemplate {
                    decorations: vec![],
                    content: vec![
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 1.0,y: 0.8} },
                            font_size: 36.0,
                            orientation: header_orientation.clone(),
                        },
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.8},
                                size: Point{x: 1.0,y: 0.2} },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                    ],
                },

                "Head_Cont" => SlideTemplate {
                    decorations: vec![],
                    content: vec![
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 1.0,y: 0.3},
                            },
                            font_size: 24.0,
                            orientation: header_orientation.clone(),
                        },
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.3},
                                size: Point{x: 1.0,y: 0.7},
                            },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                    ],
                },

                "Vert_Split" => SlideTemplate {
                    decorations: vec![],
                    content: vec![
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 0.5,y: 0.3},
                            },
                            font_size: 24.0,
                            orientation: header_orientation.clone(),
                        },
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.3},
                                size: Point{x: 0.5,y: 0.7},
                            },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.5,y: 0.0},
                                size: Point{x: 0.5,y: 0.3},
                            },
                            font_size: 24.0,
                            orientation: header_orientation,
                        },
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.5,y: 0.3},
                                size: Point{x: 0.5,y: 0.7},
                            },
                            font_size: 18.0,
                            orientation: Orientation::default(),
                        },
                    ],
                },
                "Two_Hor" => SlideTemplate {
                    decorations: vec![],
                    content: vec![
                        ContentTemplate {
                            area: Rectangle {
                                orig: Point{x: 0.0,y: 0.0},
                                size: Point{x: 1.0,y: 0.5},
                            },
                            font_size: 20.0,
                            orientation: Orientation::default(),
                        },
                        ContentTemplate {
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
        }
    }
}
