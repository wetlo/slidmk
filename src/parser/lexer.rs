use regex::Regex;
use std::iter::Iterator;

type TokenCreator<T> = dyn Fn(usize, regex::Captures) -> T;

/// iterator that iterates over all the tokens
/// from a given char-iterator
pub struct Lexer<'a, 's, T, const N: usize, const C: usize> {
    pub no_captures: [(Regex, T); N],
    pub captures: [(Regex, &'a TokenCreator<T>); C],
    pub comment: Regex,
    pub whitespace: Regex,
    pub invalid: T,
    pub source: &'s str,
}

impl<'a, 's, T: Clone, const N: usize, const C: usize> Iterator for Lexer<'a, 's, T, N, C> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // nothing more to tokenize
        if self.source.is_empty() {
            return None;
        }

        // skip the comments
        loop {
            match self.comment.find(self.source) {
                Some(m) if m.start() == 0 => self.update_pos(m.end()),
                _ => break,
            }
        }

        // skip whitespace
        let indent = match self.whitespace.find(self.source) {
            Some(m) if m.start() == 0 => m.end(),
            _ => 0,
        };
        self.update_pos(indent);

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

impl<'a, 's, T, const N: usize, const C: usize> Lexer<'a, 's, T, N, C> {
    fn update_pos(&mut self, pos: usize) {
        self.source = &self.source[pos..];
    }
}
