use std::{fs::File, io::BufReader, path::Path, process};

use crate::util::buff_container::BuffContainer;
use crate::util::IterExt;

mod iterexts;
mod lexer;
mod parse_error;
mod slide;
mod slides;
mod tokens;

use iterexts::SlideExt;
use parse_error::ParseError;
use slide::Slide;

pub fn parse_file<P: AsRef<Path>>(
    path: P,
) -> impl Iterator<Item = Result<Slide, ParseError<'static>>> {
    let reader = BufReader::new(match File::open(path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Couldn't open file due to: {}", e);
            process::exit(1);
        }
    });

    // used to read every single char of the file
    let chars = BuffContainer::new(reader).map(|c| match c {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Couldn't read another character due to: {}", e);
            process::exit(1);
        }
    });

    lexer::Lexer::new(chars)
        .leave_one(tokens::Token::Linefeed)
        .slides()
        .inspect(|token| println!("{:?}", token))
}
