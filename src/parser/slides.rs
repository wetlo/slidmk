use super::{
    combinators,
    combinators::Parser,
    //parse_error::ParseError,
    slide::{Content, Slide},
    tokens::Token,
};

use std::path::Path;

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
        if self.offset >= self.tokens.len() {
            return None;
        }

        let text = text.many().process(|v| v.into_iter().collect::<String>());
        let list = list_pre
            .and(text.clone())
            .many()
            .process(|v| Content::List(v));

        let image = text
            .clone()
            .prefix(left_bracket)
            .suffix(right_bracket)
            .and(path)
            .process(|(desc, path)| Content::Image(desc, path.into()));

        let content = path
            .process(|p| Content::Config(p.into()))
            .or(text.clone().process(|s| Content::Text(s)))
            .or(list)
            .or(image)
            .suffix(line_feed);

        let result = identifier
            .and(content.many())
            .parse(&self.tokens, self.offset);

        match result {
            Ok((offset, (kind, content))) => {
                self.offset = offset;

                Some(Ok(Slide {
                    kind: kind.into(),
                    contents: content,
                }))
            }

            Err(e) => Some(Err(e)),
        }
    }
}
