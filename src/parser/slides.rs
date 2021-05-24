use super::{
    slide::{Content, Slide},
    tokens::Token,
};

use std::path::PathBuf;

macro_rules! get_token {
    ($source:expr, $pat:pat, $ret:expr) => {
        match $source {
            Some($pat) => Some($ret),
            // TODO: add parse errors
            Some(_) => todo!(),
            None => None,
        }
    };
}

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

    fn get_image(&mut self) -> Option<(String, PathBuf)> {
        let desc = get_token!(self.buf_next(), Token::Text(d), d)?;
        let _ = get_token!(self.buf_next(), Token::SqrBracketRight, ());
        let path = get_token!(self.buf_next(), Token::Path(p), p)?;

        Some((desc, path))
    }
}

struct ListItems<'a, I: Iterator<Item = Token>>(&'a mut Slides<I>, Option<u8>);

impl<'a, I> Iterator for ListItems<'a, I>
where
    I: Iterator<Item = Token>,
{
    type Item = (u8, String);

    fn next(&mut self) -> Option<Self::Item> {
        let ident = self.1.take().or_else(|| match self.0.buf_next()? {
            Token::ListPre(i) => Some(i),
            // TODO: maybe change it to Linefeed
            _ => None,
        })?;

        let text = get_token!(self.0.buf_next(), Token::Text(s), self.0.concat_text(s))?;

        Some((ident, text))
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
            Linefeed => self.next(), // TODO: maybe don't use recursion
            ListPre(i) => Some(Content::List(ListItems(self.0, Some(i)).collect())),
            SqrBracketLeft => {
                let (desc, path) = self.0.get_image()?;

                Some(Content::Image(desc, path))
            }
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
        let kind = get_token!(self.buf_next(), Token::Identifier(i), i)?;

        // TODO: change to Result<Content, ParseError>
        let contents = ContentIter::from(self).collect();

        Some(Slide { kind, contents })
    }
}
