use std::path::PathBuf;

use printpdf::{Color as PdfColor, Mm, PdfLayerReference, Point as PdfPoint, Pt, Rgb};

use crate::{
    config::{Color, Point, Rectangle},
    drawing::DrawError,
};

pub struct DrawingArgs {
    pub area: Rectangle<f64>,
    pub font_size: f32,
    pub layer: PdfLayerReference,
}

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

pub fn to_pdf_rect(rect: &Rectangle<f64>) -> Vec<(PdfPoint, bool)> {
    pdfify_rect(rect)
        .points()
        .map(to_pdf_point)
        .map(|p| (p, false))
        .collect::<Vec<_>>()
}

pub fn pdfify_rect(Rectangle { mut orig, size }: &Rectangle<f64>) -> Rectangle<f64> {
    orig.1 += size.1;
    Rectangle {
        orig: to_pdf_coords(orig),
        size: *size,
    }
}

/// changes coordinates from the top left to
/// bottem left pdf Pt coords
fn to_pdf_coords(Point(x, y): Point<f64>) -> Point<f64> {
    Point(x, 1.0 - y)
}

fn to_pdf_point(Point(x, y): Point<f64>) -> PdfPoint {
    let x_max: Pt = X_SIZE.into();
    let y_max: Pt = Y_SIZE.into();

    PdfPoint {
        x: Pt(x * x_max.0),
        y: Pt(y * y_max.0),
    }
}

pub fn get_font_path(name: &str) -> Result<PathBuf, DrawError> {
    let path = fontconfig::Fontconfig::new()
        .ok_or(DrawError::FontConfigNotLoaded)?
        .find(name, None)
        .ok_or_else(|| DrawError::FontNotFound(name.into()))?
        .path;

    Ok(path)
}
