use super::lexer;
use regex::{Captures, Regex};
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Illegal,
    Linefeed,
    SqrBracketLeft,
    SqrBracketRight,
    Path(&'a Path),
    Text(&'a str),
    Identifier(&'a str),
    ListPre(u8),
}

fn regex(re: &str) -> Regex {
    Regex::new(re).unwrap()
}

lazy_static::lazy_static! {
    pub static ref COMMENT: Regex = regex(r";.*\n");
    pub static ref WHITESPACE: Regex = regex(r"[^\S\n]*");

    pub static ref NON_CAPTURES: [(Regex, Token<'static>); 3] = [
        (regex(r"\["), Token::SqrBracketLeft),
        (regex(r"\]"), Token::SqrBracketRight),
        (regex("\n"), Token::Linefeed),
    ];

    pub static ref CAPTURES: [(Regex, &'static lexer::TokenCreator); 4] = [
        (regex(r"---\s*([^\s\d]+)"), &identifier),
        (regex(r"-|\*"), &list_item),
        (regex(r#""(.*)""#), &path),
        (regex(r"(.*)\n?"), &text),
    ];
}

fn text(_: usize, capture: Captures) -> Token {
    Token::Text(capture.get(1).unwrap().as_str())
}

fn path(_: usize, capture: Captures) -> Token {
    Token::Path(capture.get(1).unwrap().as_str().as_ref())
}

fn identifier(_: usize, capture: Captures) -> Token {
    Token::Identifier(capture.get(1).unwrap().as_str())
}

fn list_item(ident: usize, _: Captures) -> Token {
    Token::ListPre(ident as u8)
}
