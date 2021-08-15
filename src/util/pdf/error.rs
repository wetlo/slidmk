use printpdf::image::error as image;
use std::io;

#[derive(Debug)]
pub enum PdfError {
    NoFontConfig,
    FontNotLoaded(String),
    FontNotFound(String),
    File(io::Error),
    Image(image::ImageError),
}

impl std::error::Error for PdfError {}

use std::fmt;
impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PdfError::*;
        match self {
            NoFontConfig => write!(f, "Couldn't open font config"),
            FontNotLoaded(p) => write!(f, "The font at {} couldn't be loaded", p),
            FontNotFound(s) => write!(f, "Couldn't find font {}", s),
            File(e) => write!(f, "Couldn't read file due to {}", e),
            Image(e) => write!(f, "Couldn't load image due to {}", e),
        }
    }
}

impl From<io::Error> for PdfError {
    fn from(e: io::Error) -> Self {
        Self::File(e)
    }
}

impl From<image::ImageError> for PdfError {
    fn from(e: image::ImageError) -> Self {
        Self::Image(e)
    }
}
