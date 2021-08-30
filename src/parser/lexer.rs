use regex::Regex;
use std::iter::Iterator;

pub type TokenCreator<T> = dyn for<'s> Fn(usize, regex::Captures<'s>) -> T + Sync;

/// iterator that iterates over all the tokens
/// from a given char-iterator
pub struct Lexer<'a, 's, T> {
    pub no_captures: &'a [(Regex, T)],
    pub captures: &'a [(Regex, &'a TokenCreator<T>)],
    pub comment: &'a Regex,
    pub whitespace: &'a Regex,
    pub invalid: T,
    pub source: &'s str,
}

impl<'a, 's, T: Clone> Iterator for Lexer<'a, 's, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // nothing more to tokenize
        if self.source.is_empty() {
            return None;
        }

        let mut indent;

        // skip the comments
        loop {
            // skip whitespace
            indent = match self.whitespace.find(self.source) {
                Some(m) if m.start() == 0 => m.end(),
                _ => 0,
            };
            self.update_pos(indent);

            match self.comment.find(self.source) {
                Some(m) if m.start() == 0 => self.update_pos(m.end()),
                _ => break,
            }
        }

        // look for a simple token like a linefeed ('\n')
        for (re, tok) in self.no_captures.iter() {
            match re.find(self.source) {
                Some(m) if m.start() == 0 => {
                    let tok = tok.clone();
                    self.update_pos(m.end());
                    return Some(tok);
                }
                _ => (),
            }
        }

        for (re, tok_fn) in self.captures.iter() {
            if let Some(c) = re.captures(self.source) {
                let full = c.get(0).unwrap();

                if full.start() != 0 {
                    continue;
                }

                let tok = tok_fn(indent, c);
                self.update_pos(full.end());
                return Some(tok);
            }
        }

        self.source = "";
        Some(self.invalid.clone())
    }
}

impl<'a, 's, T> Lexer<'a, 's, T> {
    fn update_pos(&mut self, pos: usize) {
        self.source = &self.source[pos..];
    }
}
