use super::{
    slide::{Content, Slide},
    tokens::Token,
};
use crate::util::RemoveFirst;

//use std::path::PathBuf;

pub struct Slides<I>
where
    I: Iterator<Item = Token>,
{
    tokens: I,
    next_token: Option<Token>,
}

impl<I> Slides<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens,
            next_token: None,
        }
    }

    fn buf_next(&mut self) -> Option<Token> {
        self.next_token.take().or_else(|| self.tokens.next())
    }

    fn get_kind(&mut self) -> Option<String> {
        if let Some(Token::Identifier(s)) = self.buf_next() {
            Some(s)
        } else {
            None
        }
    }

    fn concat_text(&mut self, mut coll: String) -> String {
        loop {
            let next = self.tokens.next();
            if let Some(Token::Text(s)) = next {
                coll.push_str(&s);
            } else {
                self.next_token = next;
                break;
            }
        }

        coll
    }

    fn get_list(&mut self, first_ident: u8) -> Option<Vec<(u8, String)>> {
        todo!()
    }

    fn get_image(&mut self) -> Option<Content> {
        todo!()
    }
}

struct ContentIter<'a, I: Iterator<Item = Token>>(&'a mut Slides<I>);

impl<'a, I> Iterator for ContentIter<'a, I>
where
    I: Iterator<Item = Token>,
{
    type Item = Content;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        match self.0.buf_next()? {
            Text(s) => Some(Content::Text(self.0.concat_text(s))),
            Path(p) => Some(Content::Path(p)),
            t => {
                self.0.next_token = Some(t);
                None
            }
        }
    }
}

impl<'a, I> From<&'a mut Slides<I>> for ContentIter<'a, I>
where
    I: Iterator<Item = Token>,
{
    fn from(slides: &'a mut Slides<I>) -> Self {
        Self(slides)
    }
}

impl<I> Iterator for Slides<I>
where
    I: Iterator<Item = Token>,
{
    type Item = Slide;
    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.get_kind()?;

        // TODO: change to Result<Content, ParseError>
        let contents = ContentIter::from(self).collect();

        Some(Slide { kind, contents })
    }
}