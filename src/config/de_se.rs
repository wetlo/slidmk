use super::{Point, Rectangle};
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct StyleJson {
    pub colors: Vec<String>,
    pub font: String,
    pub margin: Rectangle<f64>,
    #[serde(rename = "lineSpace")]
    pub line_spacing: f64,
}

/*#[derive(Deserialize)]
pub struct TemplateJson {
    pub slides: HashMap<String, SlideTemplate>,
}*/
pub type TemplateJson = HashMap<String, SlideTemplate>;

#[derive(Deserialize)]
pub struct SlideTemplate {
    pub decoration: Vec<DecorationJson>,
    pub template: Vec<ContentTemplate>,
}

#[derive(Deserialize)]
pub struct ContentTemplate {
    pub orig: Point<f64>,
    pub size: Point<f64>,
    pub orientation: String,
    #[serde(rename = "fontSize")]
    pub font_size: f32,
}

#[derive(Deserialize)]
pub struct DecorationJson {
    pub orig: Point<f64>,
    pub size: Point<f64>,
    pub color: usize,
}

impl From<StyleJson> for super::PresentStyle {
    fn from(json: StyleJson) -> Self {
        Self {
            font: json.font,
            line_spacing: json.line_spacing,
            margin: json.margin,
            colors: json
                .colors
                .iter()
                // skip the # at the beginning
                .map(|hex| hex_string_to_color(&hex[1..]))
                .collect(),
        }
    }
}

impl From<SlideTemplate> for super::SlideTemplate {
    fn from(json: SlideTemplate) -> Self {
        Self {
            decorations: json.decoration.into_iter().map(|d| d.into()).collect(),
            content: json.template.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl From<DecorationJson> for super::Decoration {
    fn from(json: DecorationJson) -> Self {
        let DecorationJson { orig, size, color } = json;
        Self {
            area: super::Rectangle { orig, size },
            color_idx: color,
        }
    }
}

impl From<ContentTemplate> for super::ContentTemplate {
    fn from(json: ContentTemplate) -> Self {
        Self {
            area: super::Rectangle {
                orig: json.orig,
                size: json.size,
            },
            font_size: json.font_size,
            orientation: str_to_orientation(&json.orientation),
        }
    }
}

const ORIENT_ERR: &str = "orientation not in the right format";

/// converts a string in the format of "<vert> <hor>"
/// ex. "top left" into an orientation
fn str_to_orientation(s: &str) -> super::Orientation {
    use super::{HorOrientation, VertOrientation};
    let lower = s.to_lowercase();
    let mut words = lower.split_whitespace();

    let vert = match words.next().expect(ORIENT_ERR) {
        "bottom" => VertOrientation::Bottom,
        "middle" => VertOrientation::Middle,
        "top" => VertOrientation::Top,
        _ => panic!("{}", ORIENT_ERR),
    };

    let hort = match words.next().expect(ORIENT_ERR) {
        "left" => HorOrientation::Left,
        "middle" => HorOrientation::Middle,
        "right" => HorOrientation::Right,
        _ => panic!("{}", ORIENT_ERR),
    };

    // nothing else behind it
    assert_eq!(words.next(), None, "{}", ORIENT_ERR);

    super::Orientation {
        vertical: vert,
        horizontal: hort,
    }
}

fn hex_string_to_color(hex: &str) -> super::Color {
    let to_color = |c| c as f64 / 256.0;
    let bytes = u32::from_str_radix(hex, 16)
        .expect("wrong color format expected RRGGBB in hex")
        .to_le_bytes();

    super::Color {
        b: to_color(bytes[0]),
        g: to_color(bytes[1]),
        r: to_color(bytes[2]),
    }
}
