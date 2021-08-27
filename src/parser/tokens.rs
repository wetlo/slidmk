use super::lexer;
use regex::{Captures, Regex};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Linefeed,
    SqrBracketLeft,
    SqrBracketRight,
    Path(PathBuf),
    Text(String),
    Identifier(String),
    ListPre(u8),
}

fn regex(re: &str) -> Regex {
    Regex::new(re).unwrap()
}

lazy_static::lazy_static! {
    pub static ref COMMENT: Regex = regex(r"\s*;.*");
    pub static ref WHITESPACE: Regex = regex(r"\s*");

    pub static ref NON_CAPTURES: [(Regex, Token); 3] = [
        (regex("["), Token::SqrBracketLeft),
        (regex("]"), Token::SqrBracketRight),
        (regex("\n"), Token::Linefeed),
    ];

    pub static ref CAPTURES: [(Regex, &'static lexer::TokenCreator<Token>); 4] = [
        (regex(".*"), &text),
        (regex(r"---\s*(![\s\d]+)"), &identifier),
        (regex(r"-|\*"), &list_item),
        (regex(r#""(.*)""#), &path),
    ];
}

fn text(_: usize, capture: Captures) -> Token {
    Token::Text(capture.get(0).unwrap().as_str().to_string())
}

fn path(_: usize, capture: Captures) -> Token {
    Token::Path(capture.get(1).unwrap().as_str().into())
}

fn identifier(_: usize, capture: Captures) -> Token {
    Token::Identifier(capture.get(1).unwrap().as_str().to_string())
}

fn list_item(ident: usize, _: Captures) -> Token {
    Token::ListPre(ident as u8)
}
