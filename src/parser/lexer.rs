use super::tokens::Token;
use crate::util::{CreateAdvancer, IterExt, PeekN};
use std::iter::Iterator;

/// iterator that iterates over all the tokens
/// from a given char-iterator
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    sqr_bracks: i8,
    source: PeekN<I>,
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
            '-' if self.look_for('-', 3) => {
                // skip those two characters
                // and unwrap is safe here because we know 2 '-' follow the first
                let _ = self.source.by_ref().skip(3);
                self.count_while(is_whitepace); // skip white space

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
            sqr_bracks: 0,
            source: chars.peekable_n(),
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
        loop {
            match self.source.next_if(|c| !ends.contains(c) || escaped) {
                None => return collector,
                Some('\\') => escaped = true,
                Some(c) => {
                    escaped = false;
                    collector.as_mut().map(|s| s.push(c));
                }
            }
        }
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

    fn look_for(&mut self, searched: char, n: usize) -> bool {
        for i in 0..n {
            if Some(&searched) != self.source.peek_nth(i) {
                return false;
            }
        }

        true
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
