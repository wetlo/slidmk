use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

use utf8_chars::BufReadCharsExt;

mod lexer;
mod tokens;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<tokens::Token>> {
    // open a reader to read every single character inside the file
    let mut reader = BufReader::new(File::open(path)?);

    let chars = reader
        .chars()
        .map(|c| c.expect("couldn't read another char"));

    Ok(lexer::Lexer::new(chars)
        .inspect(|token| println!("{:?}", token))
        .collect())
}
