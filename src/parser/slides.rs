use super::{
    combinators,
    combinators::Parser,
    //parse_error::ParseError,
    slide::{Content, Slide},
    tokens::Token,
};

use std::path::{Path, PathBuf};

//type SResult<T> = Result<T, ParseError<'static>>;

macro_rules! sokay {
    ($inner:expr) => {
        Some(Ok($inner))
    };
}

/*macro_rules! get_token {
    ($source:expr, $pat:pat, $ret:expr) => {
        match $source {
            Some($pat) => Ok($ret),
            // TODO: add parse errors
            Some(actual) => Err(ParseError {
                expected: stringify!($pat),
                actual: format!("{:?}", actual),
            }),
            None => Err(ParseError {
                expected: stringify!($pat),
                actual: "EOF".into(),
            }),
        };
    };
}*/

macro_rules! token_fn {
    ($name:ident, $ret_ty:ty, $pat:pat => $ret:expr) => {
        fn $name<'s>(input: &[Token<'s>], offset: usize) -> combinators::ParseResult<$ret_ty> {
            match input.get(offset).ok_or(())? {
                $pat => combinators::p_ok(offset + 1, $ret),
                _ => Err(()),
            }
        }
    };
}

token_fn!(identifier, &'s str, Token::Identifier(t) => t);
token_fn!(text, &'s str, Token::Text(t) => t);
token_fn!(path, &'s Path, Token::Path(p) => p);
token_fn!(list_pre, u8, Token::ListPre(i) => *i);
token_fn!(right_bracket, (), Token::SqrBracketRight => ());
token_fn!(left_bracket, (), Token::SqrBracketLeft => ());
token_fn!(line_feed, (), Token::Linefeed => ());
//token_fn!(identifier, &'s str, Token::Identifier(t) => t);

/*macro_rules! ret_err {
    ($result:expr) => {
        match $result {
            Ok(i) => i,
            Err(e) => return Some(Err(e)),
        }
    };
}

pub struct Slides<'s, I>
where
    I: Iterator<Item = Token<'s>>,
{
    tokens: I,
    next_token: Option<Token<'s>>,
}

impl<'s, I> Slides<'s, I>
where
    I: Iterator<Item = Token<'s>>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens,
            next_token: None,
        }
    }

    fn buf_next(&mut self) -> Option<Token<'s>> {
        self.next_token.take().or_else(|| self.tokens.next())
    }

    fn concat_text(&mut self, mut coll: String) -> String {
        loop {
            let next = self.tokens.next();
            if let Some(Token::Text(s)) = next {
                coll.push(' '); // add a space
                coll.push_str(s);
            } else {
                self.next_token = next;
                break;
            }
        }

        coll
    }

    fn get_image(&mut self) -> SResult<(String, PathBuf)> {
        let desc = get_token!(self.buf_next(), Token::Text(d), d)?;
        let _ = get_token!(self.buf_next(), Token::SqrBracketRight, ())?;
        let path = get_token!(self.buf_next(), Token::Path(p), p)?;

        Ok((desc.into(), path.into()))
    }
}

struct ListItems<'a, 's, I: Iterator<Item = Token<'s>>>(&'a mut Slides<'s, I>, Option<u8>);

impl<'a, 's, I> Iterator for ListItems<'a, 's, I>
where
    I: Iterator<Item = Token<'s>>,
{
    type Item = SResult<(u8, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        let ident = self.1.take().or_else(|| match self.0.buf_next()? {
            Token::ListPre(i) => Some(i),
            // TODO: maybe change it to Linefeed
            _ => None,
        })?;

        let text = ret_err!(get_token!(
            self.0.buf_next(),
            Token::Text(s),
            self.0.concat_text(s.into())
        ));

        sokay!((ident, text))
    }
}

struct ContentIter<'a, 's, I: Iterator<Item = Token<'s>>>(&'a mut Slides<'s, I>);

impl<'a, 's, I> Iterator for ContentIter<'a, 's, I>
where
    I: Iterator<Item = Token<'s>>,
{
    type Item = SResult<Content>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        match self.0.buf_next()? {
            Text(s) => sokay!(Content::Text(self.0.concat_text(s.into()))),
            Path(p) => sokay!(Content::Config(p.into())),
            Linefeed => self.next(), // TODO: maybe don't use recursion
            ListPre(i) => sokay!(Content::List(
                ret_err!(ListItems(self.0, Some(i)).collect())
            )),
            SqrBracketLeft => {
                let (desc, path) = ret_err!(self.0.get_image());

                sokay!(Content::Image(desc, path))
            }
            t => {
                self.0.next_token = Some(t);
                None
            }
        }
    }
}

impl<'a, 's, I> From<&'a mut Slides<'s, I>> for ContentIter<'a, 's, I>
where
    I: Iterator<Item = Token<'s>>,
{
    fn from(slides: &'a mut Slides<'s, I>) -> Self {
        Self(slides)
    }
}

impl<'s, I> Iterator for Slides<'s, I>
where
    I: Iterator<Item = Token<'s>>,
{
    type Item = SResult<Slide>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = match get_token!(self.buf_next(), Token::Identifier(i), i) {
            Ok(i) => i,
            Err(ParseError { actual: e, .. }) if e == "EOF" => return None,

            Err(e) => return Some(Err(e)),
        };

        // TODO: change to Result<Content, ParseError>
        let contents = ret_err!(ContentIter::from(self).collect());

        sokay!(Slide {
            kind: kind.into(),
            contents
        })
    }
}*/

pub struct Slides<'s> {
    tokens: Vec<Token<'s>>,
    offset: usize,
}

impl<'s> Slides<'s> {
    pub fn new(tokens: Vec<Token<'s>>) -> Self {
        Self { tokens, offset: 0 }
    }
}

impl<'s> Iterator for Slides<'s> {
    type Item = Result<Slide, combinators::ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = text.many().after(|v| v.into_iter().collect::<String>());
        let list = list_pre
            .and(text.clone())
            .many()
            .after(|v| Content::List(v));

        let _content = path
            .after(|p| Content::Config(p.into()))
            .or(text.clone().after(|s| Content::Text(s)))
            .or(list);

        todo!()
    }
}
