use super::{slides::Slides, tokens::Token};

pub trait SlideExt<I>: Iterator
where
    I: Iterator<Item = Token>,
{
    fn slides(self) -> Slides<I>;
}

impl<I> SlideExt<I> for I
where
    I: Iterator<Item = Token>,
{
    fn slides(self) -> Slides<I> {
        Slides::new(self)
    }
}
