use std::{
    fs::File,
    io::{BufReader, Result},
    path::Path,
};

use utf8_chars::BufReadCharsExt;

mod iterexts;
mod lexer;
mod parse_error;
mod slide;
mod slides;
mod tokens;

use iterexts::SlideExt;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<slide::Slide>> {
    // open a reader to read every single character inside the file
    let mut reader = BufReader::new(File::open(path)?);

    let chars = reader
        .chars()
        .map(|c| c.expect("couldn't read another char"));

    Ok(lexer::Lexer::new(chars)
        .slides()
        .inspect(|token| println!("{:?}", token))
        .collect())
}
