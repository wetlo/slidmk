use std::path::PathBuf;

use printpdf::{Color as PdfColor, Mm, Point, Pt, Rgb};

use crate::{
    config::{Color, Rectange},
    drawing::DrawError,
};

impl Into<PdfColor> for Color {
    fn into(self) -> PdfColor {
        PdfColor::Rgb(Rgb {
            r: self.0,
            g: self.1,
            b: self.2,
            icc_profile: None,
        })
    }
}

/*
cant make an const function with floating point
arithmetic, yet
*/
macro_rules! px_to_mm {
    ($px:expr) => {
        Mm($px as f64 * (25.4 / 300.0))
    };
}

pub const X_SIZE: Mm = px_to_mm!(1920);
pub const Y_SIZE: Mm = px_to_mm!(1080);

pub fn to_pdf_rect(rect: &Rectange<f64>) -> Vec<(Point, bool)> {
    rect.points()
        .map(|p| to_pdf_coords(p.into()))
        .map(|(x, y)| (Point { x, y }, false))
        .collect()
}

/// changes coordinates from the top left to
/// bottem left pdf Pt coords
pub fn to_pdf_coords((x, y): (f64, f64)) -> (Pt, Pt) {
    (
        Pt(x * Pt::from(X_SIZE).0),
        Pt((1.0 - y) * Pt::from(Y_SIZE).0),
    )
}

pub fn get_font_path(name: &str) -> Result<PathBuf, DrawError> {
    let path = fontconfig::Fontconfig::new()
        .ok_or(DrawError::FontConfigNotLoaded)?
        .find(name, None)
        .ok_or_else(|| DrawError::FontNotFound(name.into()))?
        .path;
    
        Ok(path)
}
