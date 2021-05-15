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

pub struct Lexer<I>
where
    I: Iterator<Item = String>,
{
    last: Option<String>,
    source: Peekable<I>,
}
macro_rules! parse_helper {
    ($self:ident, $line:ident => $([$regex:expr, $m:ident => $return:expr $(, $last:expr)?]),+ => $end:tt) => {
        $(
            if let Some($m) = $regex.captures(&$line) {
                $($self.last = Some($last);)?
                Some($return)
            }
        )else+
            else {
                $end
            }
    };
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = String>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // don't allocate if you don't need to
        let mut buffer = None::<String>;

        while let Some(line) = self.next_source() {
            /*let token = if let Some(mut b) = buffer {
                if self.is_next_empty() {
                    //Some(Token::Text(buffer.unwrap() /*must be some*/))
                    return Some(Token::Text(b)); // return to please the borrow checker
                } else {
                    //buffer.map(|s| s.push_str(&line));
                    b.push_str(&line);
                    buffer = Some(b);
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
            };*/

            let token = parse_helper! (self, line =>
                [SLIDE_REG, m => Token::Identifier(m.get(1)?.as_str().into())],
                [LIST_IDENT, m => Token::ListPre(m.get(0)?.end() as u8 - 2), line[m.get(0)?.end()..].into()]
            => {
                buffer = Some(line);
                None
            });

            // no token found yet so another round
            if token.is_some() {
                return token;
            }
        }

        None
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = String>,
{
    pub fn new(strings: I) -> Self {
        Self {
            source: strings.peekable(),
            last: None,
        }
    }

    fn next_source(&mut self) -> Option<String> {
        self.last.take().or_else(|| self.source.next())
    }

    fn is_next_empty(&mut self) -> bool {
        self.source.peek().filter(|s| s.is_empty()).is_some()
    }
}
