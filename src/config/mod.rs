use crate::drawing::error::DrawError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};
pub type StyleMap = HashMap<String, SlideTemplate>;

mod de_se;
mod default;
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
    pub margin: Rectangle<f64>,
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
            margin: Rectangle {
                orig: Point { x: 0.05, y: 0.05 },
                size: Point { x: 0.9, y: 0.9 },
            },
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
    pub fn with_style(&mut self, path: PathBuf) {
        self.style = Some(path);
    }

    pub fn with_templates(&mut self, paths: Vec<PathBuf>) {
        self.templates = Some(paths);
    }

    pub fn build(self) -> Result<Config<'static>, String> {
        let style = self.style.map_or_else(PresentStyle::default, |s| {
            let r = get_reader(s).unwrap();
            let json: de_se::StyleJson = serde_hjson::from_reader(r).unwrap();
            PresentStyle::from(json)
        });

        let template_paths = self.templates.ok_or("no templates given".to_string())?;

        let temps = template_paths.iter().map(|p| {
            let r = get_reader(p).unwrap();
            let json: de_se::TemplateJson = serde_hjson::from_reader(r).map_err(|e| {
                format!(
                    "invalid template at: {} due to:\n{}",
                    p.to_string_lossy(),
                    e
                )
            })?;
            Ok::<_, String>(
                json.into_iter()
                    .map(|(k, t)| (k, SlideTemplate::from(t)))
                    .collect::<Vec<_>>(),
            )
        });

        let mut temp_map = StyleMap::new();
        // TODO: change later so style takes the margin

        for s in temps {
            temp_map.extend(s?);
        }

        Ok(Config {
            style,
            doc_name: "default",
            slide_templates: temp_map,
        })
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    pub style: PresentStyle,
    pub slide_templates: StyleMap,
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
