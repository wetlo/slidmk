use crate::util::pdf;
use printpdf::image::error::ImageError;
use printpdf::Error as PdfError;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::Error as IoError,
};

#[derive(Debug)]
pub enum DrawError {
    IoError(PdfError),
    ImageNotLoaded(ImageError),
    KindNotFound(String),
    NoColor(usize),
    Pdf(pdf::PdfError),
}

impl Error for DrawError {}

impl Display for DrawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        use DrawError::*;
        match self {
            IoError(e) => e.fmt(f),
            KindNotFound(actual) => write!(f, "couldn't find pdf kind {}", actual),
            NoColor(idx) => write!(f, "no color found at index {}.", idx),
            ImageNotLoaded(e) => write!(f, "couldn't decode the image due to: {}", e),
            Pdf(e) => write!(f, "an pdf error occurred: {}", e),
        }
    }
}

impl From<pdf::PdfError> for DrawError {
    fn from(e: pdf::PdfError) -> Self {
        Self::Pdf(e)
    }
}

impl From<ImageError> for DrawError {
    fn from(e: ImageError) -> Self {
        Self::ImageNotLoaded(e)
    }
}

impl From<IoError> for DrawError {
    fn from(e: IoError) -> Self {
        Self::IoError(e.into())
    }
}

impl From<PdfError> for DrawError {
    fn from(e: PdfError) -> Self {
        Self::IoError(e)
    }
}
