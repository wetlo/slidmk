use printpdf::Error as PdfError;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::Error as IoError,
};

#[derive(Debug)]
pub enum DrawError {
    PdfError(PdfError),
    KindNotFound(String),
    NoColor(usize),
    FontConfigNotLoaded,
    FontNotFound(String),
    FontNotLoaded(String),
}

impl Error for DrawError {}

impl Display for DrawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::PdfError(e) => e.fmt(f),
            Self::KindNotFound(actual) => write!(f, "couldn't find pdf kind {}", actual),
            Self::NoColor(idx) => write!(f, "no color found at index {}.", idx),
            Self::FontNotFound(font) => write!(f, "couldn't find the font {}", font),
            Self::FontConfigNotLoaded => write!(f, "couldn't load the font config"),
            Self::FontNotLoaded(font) => write!(f, "couldn't find the font {}", font),
        }
    }
}

impl From<PdfError> for DrawError {
    fn from(e: PdfError) -> Self {
        Self::PdfError(e)
    }
}

impl From<IoError> for DrawError {
    fn from(e: IoError) -> Self {
        Self::PdfError(e.into())
    }
}
