use std::path::PathBuf;

use printpdf::{Color as PdfColor, Mm, PdfLayerReference, Point as PdfPoint, Pt, Rgb};

use crate::{
    config::{Color, Point, Rectangle},
    drawing::DrawError,
};

pub struct DrawingArgs {
    pub area: Rectangle<Pt>,
    pub font_size: f32,
    pub layer: PdfLayerReference,
}

impl From<Color> for PdfColor {
    fn from(c: Color) -> Self {
        PdfColor::Rgb(Rgb {
            r: c.0,
            g: c.1,
            b: c.2,
            icc_profile: None,
        })
    }
}

pub const DPI: u16 = 300;

/*
cant make an const function with floating point
arithmetic, yet
*/
macro_rules! px_to_mm {
    ($px:expr) => {
        Mm($px as f64 * (25.4 / DPI as f64))
    };
}

pub fn pt_to_px(pt: f32, dpi: u16) -> f32 {
    pt * dpi as f32 / 72.0
}

pub const X_SIZE: Mm = px_to_mm!(1920);
pub const Y_SIZE: Mm = px_to_mm!(1080);

pub fn to_pdf_rect(rect: &Rectangle<f64>) -> Vec<(PdfPoint, bool)> {
    make_inner_pt(&to_bottom_left(rect))
        .points()
        .map(|Point(x, y)| (PdfPoint { x, y }, false))
        .collect::<Vec<_>>()
}

pub fn to_bottom_left(Rectangle { mut orig, size }: &Rectangle<f64>) -> Rectangle<f64> {
    orig.1 += size.1;
    Rectangle {
        orig: to_pdf_coords(orig),
        size: *size,
    }
}

pub fn make_inner_pt(Rectangle { orig, size }: &Rectangle<f64>) -> Rectangle<Pt> {
    Rectangle {
        orig: scale_to_pt(*orig),
        size: scale_to_pt(*size),
    }
}

/// changes coordinates from the top left to
/// bottem left pdf Pt coords
fn to_pdf_coords(Point(x, y): Point<f64>) -> Point<f64> {
    Point(x, 1.0 - y)
}

fn scale_to_pt(Point(x, y): Point<f64>) -> Point<Pt> {
    let x_max: Pt = X_SIZE.into();
    let y_max: Pt = Y_SIZE.into();

    Point(x_max * x, y_max * y)
}

pub fn get_font_path(name: &str) -> Result<PathBuf, DrawError> {
    let path = fontconfig::Fontconfig::new()
        .ok_or(DrawError::FontConfigNotLoaded)?
        .find(name, None)
        .ok_or_else(|| DrawError::FontNotFound(name.into()))?
        .path;

    Ok(path)
}
