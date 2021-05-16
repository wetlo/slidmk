use super::tokens::Token;
use std::iter::{Iterator, Peekable};

pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    /// stores misread characters
    cached: Option<String>,
    source: Peekable<I>,
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let is_whitepace = |c: char| c.is_whitespace() && c != '\n';

        // skip whitespace
        let skipped = self.count_while(is_whitepace);

        // skip comments
        if self.source.peek() == Some(&';') {
            self.collect_until(&['\n'], None);
        }

        match self.source.next()? {
            '\n' => Some(Token::Linefeed),
            // a list item prefix
            '*' | '-' if self.is_next_whitespace() => Some(Token::ListPre(skipped)),
            '-' if self.count_while(|c| c == '-') == 2 => {
                // TODO: add ability to readd those --- if
                // there isn't a valid Identifier
                self.count_while(is_whitepace);
                Some(Token::Identifier(self.get_identifier()))
            }
            // TODO: add picture support
            '!' => unimplemented!(),
            c => Some(Token::Text(
                self.collect_until(&['\n', ';'], Some(String::from(c)))
                    .unwrap(),
            )),
        }
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(chars: I) -> Self {
        Self {
            cached: None,
            source: chars.peekable(),
        }
    }

    /// collects all chars until a certain char appears
    /// the collected chars will be inserted into the collector if
    /// it is Some and otherwise are ignored
    /// the enum type of the return value is the same as the argument collector
    fn collect_until(&mut self, ends: &[char], mut collector: Option<String>) -> Option<String> {
        // TODO: maybe rewrite with advance_while
        let mut coll_fn: Box<dyn FnMut(char)> = match collector.as_mut() {
            Some(s) => Box::new(move |c| s.push(c)),
            None => Box::new(|_: char| ()), // () is a noop
        };

        while self.source.peek().filter(|&c| !ends.contains(c)).is_some() {
            coll_fn(self.source.next().unwrap())
        }

        // the collect function borrows the collector mutably so
        // to transfer ownership we need to drop it before
        drop(coll_fn);
        collector
    }

    fn count_while<P>(&mut self, mut predicate: P) -> u8
    where
        P: FnMut(char) -> bool,
    {
        //self.source.advance_while(|&c| predicate(c)).count() as u8;
        self.source.by_ref().take_while(|&c| predicate(c)).count() as u8
    }

    fn is_next_whitespace(&mut self) -> bool {
        match self.source.peek() {
            Some(c) => c.is_whitespace(),
            _ => false,
        }
    }

    fn get_identifier(&mut self) -> String {
        self.source
            .by_ref()
            .take_while(|&c| c.is_alphanumeric() || c == '_')
            .collect()
        /*self.source
        // TODO: remove this unnecessary construct (.b
        .advance_while(|&c| c.is_alphabetic() || c == '_')
        .collect();*/
    }
}
