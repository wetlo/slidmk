use crate::drawing::error::DrawError;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};
pub type TemplateMap = HashMap<String, SlideTemplate>;

mod de_se;
mod default;
mod primitives;

pub use primitives::*;

use self::default::default_slide_templates;

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

    fn get_style(&self) -> PresentStyle {
        let style: Result<_, Cow<_>> = self
            .style
            .as_ref()
            .ok_or("no style given".into())
            .map(|s| {
                let r = get_reader(s).unwrap();
                let json: de_se::StyleJson = serde_hjson::from_reader(r).map_err(|e| {
                    Cow::Owned(format!(
                        "invalid style format: {}, due to\n{}",
                        s.to_string_lossy(),
                        e
                    ))
                })?;
                Ok(PresentStyle::from(json))
            })
            .flatten();

        if let Err(e) = &style {
            eprint!("{}\n\tusing default instead", e);
        }

        style.unwrap_or_default()
    }

    fn parse_templates<'a, I: Iterator<Item = &'a PathBuf> + 'a>(
        paths: I,
    ) -> impl Iterator<Item = Result<impl Iterator<Item = (String, SlideTemplate)>, String>> + 'a
    {
        paths.map(|p| {
            let r = get_reader(p).unwrap();
            let json: de_se::TemplateJson = serde_hjson::from_reader(r).map_err(|e| {
                format!(
                    "invalid template at: {} due to:\n{}",
                    p.to_string_lossy(),
                    e
                )
            })?;
            Ok(json.into_iter().map(|(k, t)| (k, SlideTemplate::from(t))))
        })
    }

    fn get_templates(&self) -> TemplateMap {
        let map: Result<_, Cow<str>> = try {
            let paths = self
                .templates
                .as_ref()
                .ok_or(Cow::Borrowed("no templates given"))?;

            Self::parse_templates(paths.iter()).try_fold(TemplateMap::new(), |mut map, t| {
                map.extend(t?);
                Ok::<_, String>(map)
            })?
        };

        map.unwrap_or_else(|e| -> HashMap<String, SlideTemplate> {
            eprintln!("{}\n\tusing default template", e);
            default_slide_templates()
        })
    }

    pub fn build(self, doc_name: &'_ str) -> Config<'_> {
        Config {
            style: self.get_style(),
            slide_templates: self.get_templates(),
            doc_name,
        }
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    pub style: PresentStyle,
    pub slide_templates: TemplateMap,
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
