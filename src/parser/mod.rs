use crate::util::IterExt;
use std::{fs::File, io::BufReader, io::Read, path::Path, process};
use tokens::Token;

mod iterexts;
mod lexer;
mod parse_error;
mod slide;
mod slides;
mod tokens;

use iterexts::SlideExt;
pub use slide::*;

pub fn parse_file<P: AsRef<Path>>(path: P) -> impl Iterator<Item = Slide> {
    let mut reader = BufReader::new(match File::open(path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Couldn't open file due to: {}", e);
            process::exit(1);
        }
    });

    let mut source = String::new();
    reader.read_to_string(&mut source).unwrap();

    lexer::Lexer {
        source: &source,
        no_captures: [],
        captures: [],
        comment: todo!(),
        whitespace: todo!(),
        invalid: Token::Illegal,
    }
    .leave_one(tokens::Token::Linefeed)
    .slides()
    .map(|s| match s {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    })
    //.inspect(|token| println!("{:?}", token))
}
