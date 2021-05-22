use super::tokens::Token;
use crate::util::advancer::CreateAdvancer;
use std::iter::{Iterator, Peekable};

/// iterator that iterates over all the tokens
/// from a given char-iterator
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    /// stores misread characters
    _cached: Option<String>,
    sqr_bracks: i8,
    source: Peekable<I>,
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // skip whitespace
        let skipped = self.count_while(is_whitepace);

        // skip comments
        if self.source.peek() == Some(&';') {
            self.collect_until(&['\n'], None, false);
            self.source.next(); // consume the trailing \n
        }

        let token = match self.source.next()? {
            // \n shouldn't be escaped
            '\n' => Token::Linefeed,

            '\\' => self.get_text(String::new(), true),

            '[' => {
                self.update_square_brackets(1);
                Token::SqrBracketLeft
            }
            ']' => {
                self.update_square_brackets(-1);
                Token::SqrBracketRight
            }

            // a list item prefix
            '*' | '-' if self.is_next_whitespace() => Token::ListPre(skipped),

            // identifier (---IDENTIFIER)
            '-' if self.count_while(|c| c == '-') == 2 => {
                // TODO: add ability to readd those --- if
                // there isn't a valid Identifier
                self.count_while(is_whitepace);
                Token::Identifier(self.get_identifier())
            }

            // "some/path/"
            '"' => self.get_path(),
            // just some text
            c => self.get_text(String::from(c), false),
        };

        Some(token)
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    /// creates the TokenIterator with an
    /// iterate over the chars of the source (\n should be included)
    pub fn new(chars: I) -> Self {
        Self {
            _cached: None,
            sqr_bracks: 0,
            source: chars.peekable(),
        }
    }

    /// collects all chars until a certain char appears
    /// the collected chars will be inserted into the collector if
    /// it is Some and otherwise are ignored
    /// the enum type of the return value is the same as the argument collector
    fn collect_until(
        &mut self,
        ends: &[char],
        mut collector: Option<String>,
        mut escaped: bool,
    ) -> Option<String> {
        // TODO: maybe rewrite with advance_while
        let mut coll_fn: Box<dyn FnMut(char)> = match collector.as_mut() {
            Some(s) => Box::new(move |c| s.push(c)),
            None => Box::new(|_: char| ()), // () is a noop
        };

        while self.source.peek().filter(|&c| !ends.contains(c)).is_some()
            || self.source.peek().is_some() && escaped
        {
            match self.source.next().unwrap_or_else(|| unreachable!()) {
                '\\' => escaped = true,
                c => {
                    escaped = false;
                    coll_fn(c)
                }
            }
        }

        // the collect function borrows the collector mutably so
        // to transfer ownership we need to drop it before
        drop(coll_fn);
        collector
    }

    fn update_square_brackets(&mut self, sign: i8) {
        self.sqr_bracks += sign;

        if self.sqr_bracks < 0 {
            self.sqr_bracks = 0;
        }
    }

    /// counts all the occurrences
    fn count_while<P>(&mut self, mut predicate: P) -> u8
    where
        P: FnMut(char) -> bool,
    {
        self.source.advance_while(|&c| predicate(c)).count() as u8
    }

    fn is_next_whitespace(&mut self) -> bool {
        match self.source.peek() {
            Some(c) => c.is_whitespace(),
            _ => false,
        }
    }

    fn get_identifier(&mut self) -> String {
        self.source
            .advance_while(|&c| c.is_alphanumeric() || c == '_')
            .collect()
        // TODO: remove advance_while
    }

    fn get_path(&mut self) -> Token {
        let path = self
            .collect_until(&['"'], Some(String::new()), false)
            .unwrap()
            .into();

        // ignore the "
        if self.source.next().is_none() {
            // if a " doesn't exit it's a not a valid token
            Token::Illegal
        } else {
            Token::Path(path)
        }
    }

    fn get_text(&mut self, collector: String, first_escaped: bool) -> Token {
        let delims: &[char] = if self.sqr_bracks > 0 {
            &['\n', ';', ']']
        } else {
            &['\n', ';']
        };

        Token::Text(
            self.collect_until(&delims, Some(collector), first_escaped)
                .unwrap(),
        )
    }
}

fn is_whitepace(c: char) -> bool {
    c.is_whitespace() && c != '\n'
}
