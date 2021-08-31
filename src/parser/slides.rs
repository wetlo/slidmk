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

fn construct_slide_parser<'s>() -> impl Parser<Token<'s>, Output = Slide> {
    let text = text
        .many()
        .process(|v| v.into_iter().intersperse(" ").collect());
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
        .or(text.process(Content::Text))
        .suffix(line_feed.or(combinators::eof));
    //.inspect(|c| eprintln!("found Content: {:?}", c));

    identifier
        .suffix(line_feed)
        .and(content.many())
        .process(|(kind, content)| Slide {
            kind: kind.into(),
            contents: content,
        })
}

pub fn lazy_parser(
    tokens: Vec<Token<'_>>,
) -> impl Iterator<Item = Result<Slide, ParseError<'static>>> + '_ {
    let parser = construct_slide_parser();
    let mut offset = 0;

    std::iter::from_fn(move || {
        if offset < tokens.len() {
            match parser.parse(&tokens, offset) {
                Err(e) => Some(Err(e)),

                Ok((off, slide)) => {
                    offset = off;
                    Some(Ok(slide))
                }
            }
        } else {
            None
        }
    })
}
