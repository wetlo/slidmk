use printpdf::{Color as PdfColor, Rgb};

use crate::config::Color;

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
