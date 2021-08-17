pub mod error;
pub mod pdf_maker;

use std::io::Write;

pub use error::*;

use crate::{config::Config, parser::Slide};

type DResult<T> = Result<T, DrawError>;

pub trait Drawer {
    fn write<W: Write>(self, to: W) -> DResult<()>;
    fn create_slide(&mut self, slides: Slide, config: &Config) -> DResult<()>;
}
