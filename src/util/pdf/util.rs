use crate::config::Color;
use printpdf::{Color as PdfColor, Rgb};

impl From<Color> for PdfColor {
    fn from(c: Color) -> Self {
        PdfColor::Rgb(Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
            icc_profile: None,
        })
    }
}

const INCHES_PER_POINT: f64 = 72.0;

pub fn pt_to_px(pt: f64, dpi: u16) -> f64 {
    pt * dpi as f64 / INCHES_PER_POINT
}

pub fn px_to_pt(px: f64, dpi: u16) -> f64 {
    px * INCHES_PER_POINT / dpi as f64
}
