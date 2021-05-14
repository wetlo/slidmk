use super::tokens::Token;
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::{Iterator, Peekable};

lazy_static! {
    static ref SLIDE_REG: Regex = Regex::new(r"^---(\S+)").unwrap();
    static ref LIST_IDENT: Regex = Regex::new(r"^[\s--\n]*[\*\-]\s").unwrap();
    static ref LINE_COMMENT: Regex = Regex::new(r";.*").unwrap();
    // TODO: maybe add multiline comments
    /*static ref BLOCK_COMM_BEGIN: Regex = Regex::new(r";[").unwrap();
    static ref BLOCK_COMM_END: Regex = Regex::new(r"];").unwrap();*/
}

pub struct Lexer<I, S>
where
    I: Iterator<Item = S>,
    S: Into<String>,
{
    last: Option<String>,
    source: Peekable<I>,
}

impl<I, S> Iterator for Lexer<I, S>
where
    I: Iterator<Item = S>,
    S: Into<String>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // don't allocate if you don't need to
        let mut buffer = None::<String>;

        while let Some(line) = self
            .last
            .take()
            .or_else(|| self.source.next().map(Into::into))
        {
            let token = if buffer.is_some() {
                if self.source.peek().filter(|s| s.into().is_empty()).is_some() {
                    Some(Token::Text(buffer.unwrap() /*must be some*/))
                } else {
                    buffer.map(|s| s.push_str(&line));
                    None
                }
            } else if line.is_empty() {
                Some(Token::Linefeed)
            } else if let Some(cap) = SLIDE_REG.captures(&line) {
                Some(Token::Identifier(cap.get(1)?.as_str().into()))
            } else if let Some(m) = LIST_IDENT.find(&line) {
                // TODO: should still be looked at by the text
                // the rest of the line needs to be processed
                self.last = Some(line[m.end()..].into());
                Some(Token::ListPre(m.end() as u8 - 2))
            // nothing matched so start our buffer collector
            } else {
                buffer = Some(line);
                None
            };

            // no token found yet so another round
            if token.is_some() {
                return token;
            }
        }

        None
    }
}

impl<I, S> Lexer<I, S>
where
    I: Iterator<Item = S>,
    S: Into<String>,
{
    pub fn new(strings: I) -> Self {
        Self {
            source: strings.peekable(),
            last: None,
        }
    }
}
