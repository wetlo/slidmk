use crate::util::IterExt;
use tokens::Token;

mod iterexts;
mod lexer;
mod parse_error;
mod slide;
mod slides;
mod tokens;

use iterexts::SlideExt;
pub use slide::*;

pub fn parse(source: &'_ str) -> impl Iterator<Item = Slide> + '_ {
    lexer::Lexer {
        source: &source,
        no_captures: tokens::NON_CAPTURES.as_ref(),
        captures: tokens::CAPTURES.as_ref(),
        comment: &tokens::COMMENT,
        whitespace: &tokens::WHITESPACE,
        invalid: Token::Illegal,
    }
    .leave_one(tokens::Token::Linefeed)
    .slides()
    .map(|s| match s {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    })
    //.inspect(|token| println!("{:?}", token))
}
