use crate::drawing::error::DrawError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
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

impl Default for PresentStyle {
    fn default() -> Self {
        PresentStyle {
            colors: vec![
                Color::new(0.0, 0.0, 0.0),
                Color::new(1.0, 0.0, 0.0),
                Color::new(0.0, 1.0, 1.0),
            ],
            font: String::from("Noto Sans"),
            line_spacing: 1.0,
        }
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    pub style: Option<PathBuf>,
    pub templates: Option<Vec<PathBuf>>,
}

impl ConfigBuilder {
    pub fn with_style(mut self, path: PathBuf) -> Self {
        self.style = Some(path);
        self
    }

    pub fn with_templates(mut self, paths: Vec<PathBuf>) -> Self {
        self.templates = Some(paths);
        self
    }

    pub fn build(self) -> Config<'static> {
        let style = self.style.map_or_else(PresentStyle::default, |s| {
            let r = get_reader(s).unwrap();
            let json: de_se::StyleJson = serde_hjson::from_reader(r).unwrap();
            PresentStyle::from(json)
        });

        let mut jsons = self.templates.map(|v| {
            v.into_iter().map(|p| {
                let r = get_reader(p).unwrap();
                let json: de_se::TemplateJson = serde_hjson::from_reader(r).unwrap();
                (
                    json.margin,
                    json.slides
                        .into_iter()
                        .map(|(k, t)| (k, SlideTemplate::from(t)))
                        .collect::<Vec<_>>(),
                )
            })
        });

        let first_slides;
        let margin = if let Some(s) = jsons.as_mut().map(|j| j.next()).flatten() {
            first_slides = Some(s.1);
            s.0
        } else {
            first_slides = None;
            Rectangle {
                orig: Point { x: 0.05, y: 0.05 },
                size: Point { x: 0.9, y: 0.9 },
            }
        };

        let slide_temps = if let Some(temps) = jsons {
            let mut map = StyleMap::new();
            let iter = std::iter::once(first_slides.unwrap());

            for s in temps.map(|t| t.1).chain(iter) {
                map.extend(s);
            }

            map
        } else {
            default_slide_templates()
        };

        Config {
            style,
            doc_name: "default",
            margin,
            slide_styles: slide_temps,
        }
    }
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
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
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

fn default_slide_templates() -> StyleMap {
    let header_orientation = Orientation {
        vertical: VertOrientation::Bottom,
        horizontal: HorOrientation::Middle,
    };
    crate::map! {
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
    }
}
