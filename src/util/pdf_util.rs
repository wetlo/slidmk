use printpdf::{Color as PdfColor, Point, Rgb, Mm};

use crate::config::{Color, Rectange};

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
    todo!()
}

pub fn to_pdf_coords((x, y): (f64, f64)) -> (f64, f64) {
    todo!()
}