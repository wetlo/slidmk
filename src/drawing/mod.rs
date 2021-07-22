pub mod error;
pub mod pdf_maker;

use std::io::Write;

pub use error::*;

use crate::{config::Config, parser::Slide};

type DResult<T> = Result<T, DrawError>;

pub trait Drawer {
    fn write<W: Write>(self, to: W) -> DResult<()>;
    fn create_slides<I: Iterator<Item = Slide>>(
        &mut self,
        slides: I,
        config: &Config<'_>,
    ) -> DResult<()>;
}
