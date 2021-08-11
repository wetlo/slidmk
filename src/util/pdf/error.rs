#[derive(Debug)]
pub enum PdfError {
    NoFontConfig,
    FontNotLoaded(String),
    FontNotFound(String),
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
        }
    }
}
