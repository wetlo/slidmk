use super::tokens::Token;
use std::iter::Iterator;

pub struct Lexer<I, S>
where
    I: Iterator<Item = S>,
    S: Into<char>,
{
    source: I,
}

impl<I, S> Iterator for Lexer<I, S>
where
    I: Iterator<Item = S>,
    S: Into<char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|c| Token::Text(c.into().into()))
    }
}

impl<I, S> Lexer<I, S>
where
    I: Iterator<Item = S>,
    S: Into<char>,
{
    pub fn new(chars: I) -> Self {
        Self { source: chars }
    }
}
