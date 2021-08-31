use super::{
    combinators,
    combinators::Parser,
    parse_error::ParseError,
    slide::{Content, Slide},
    tokens::Token,
};

use std::path::Path;

macro_rules! token_fn {
    ($name:ident, $ret_ty:ty, $pat:pat => $ret:expr) => {
        /// tries to get the token, if the token is
        /// not found it creates a nice error
        fn $name<'s>(input: &[Token<'s>], offset: usize) -> combinators::ParseResult<$ret_ty> {
            match input.get(offset) {
                Some($pat) => combinators::p_ok(offset + 1, $ret),
                Some(t) => Err(ParseError {
                    actual: format!("{:?}", t),
                    expected: stringify!($pat),
                }),
                None => Err(ParseError {
                    actual: String::from("EOF"),
                    expected: stringify!($pat),
                }),
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

/// an iterator over the lazily parsed slides
pub struct Slides<'s> {
    tokens: Vec<Token<'s>>,
    offset: usize,
}

impl<'s> Slides<'s> {
    /// creates the slide parser iterator with the token contents
    pub fn new(tokens: Vec<Token<'s>>) -> Self {
        Self { tokens, offset: 0 }
    }
}

impl<'s> Iterator for Slides<'s> {
    type Item = Result<Slide, ParseError<'static>>;

    /// parses the next slide if available
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.tokens.len() {
            return None;
        }

        let text = text.many().process(|v| v.into_iter().collect::<String>());
        let list = list_pre.and(text.clone()).many().process(Content::List);

        // TODO: fix problem where you can't write ] in normal text
        let image = text
            .clone()
            .prefix(left_bracket)
            .suffix(right_bracket)
            .and(path)
            .process(|(desc, path)| Content::Image(desc, path.into()));

        let content = path
            .process(|p| Content::Config(p.into()))
            .or(image)
            .or(list)
            .or(text.clone().process(Content::Text))
            .suffix(line_feed.or(combinators::eof));
        //.inspect(|c| eprintln!("found Content: {:?}", c));

        let result = identifier
            .suffix(line_feed)
            .and(content.many())
            .process(|(kind, content)| Slide {
                kind: kind.into(),
                contents: content,
            })
            .parse(&self.tokens, self.offset);

        match result {
            Err(e) => Some(Err(e)),

            Ok((offset, slide)) => {
                self.offset = offset;
                Some(Ok(slide))
            }
        }
    }
}
