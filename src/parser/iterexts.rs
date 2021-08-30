use super::{slides::Slides, tokens::Token};

pub trait SlideExt<'s>: Iterator {
    fn slides(self) -> Slides<'s>;
}

impl<'a, I> SlideExt<'a> for I
where
    I: Iterator<Item = Token<'a>>,
{
    fn slides(self) -> Slides<'a> {
        let toks = self.collect();
        Slides::new(toks)
    }
}
