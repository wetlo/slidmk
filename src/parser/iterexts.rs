use super::{slides::Slides, tokens::Token};

pub trait SlideExt<'a, I>: Iterator
where
    I: Iterator<Item = Token<'a>>,
{
    fn slides(self) -> Slides<'a, I>;
}

impl<'a, I> SlideExt<'a, I> for I
where
    I: Iterator<Item = Token<'a>>,
{
    fn slides(self) -> Slides<'a, I> {
        Slides::new(self)
    }
}
