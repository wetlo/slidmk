use tokens::Token;

mod combinators;
mod lexer;
mod parse_error;
mod slide;
mod slides;
mod tokens;

pub use slide::*;

/// takes a reference to some source and returns the lazily
/// parsed slides
pub fn parse(source: &'_ str) -> impl Iterator<Item = Slide> + '_ {
    let tokens = lexer::Lexer {
        source,
        no_captures: tokens::NON_CAPTURES.as_ref(),
        captures: tokens::CAPTURES.as_ref(),
        comment: &tokens::COMMENT,
        whitespace: &tokens::WHITESPACE,
        invalid: Token::Illegal,
    }
    .filter({
        let mut next = false;
        let mut last = false;
        move |t| {
            last = next;
            next = t == &Token::Linefeed;
            !(last && next)
        }
    })
    .inspect(|t| eprintln!("{:?}", t))
    .collect();

    slides::lazy_parser(tokens)
        .map(|s| match s {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
        })
        .inspect(|s| println!("slide: {:?}", s))
}
