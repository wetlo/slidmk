use std::{
    fs::File,
    io::{BufReader, Result},
    path::Path,
};

use utf8_chars::BufReadCharsExt;

mod lexer;
mod tokens;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<()> {
    // open a reader to read every single character inside the file
    let mut reader = BufReader::new(File::open(path)?);
    let chars = reader.chars().map(|c| c.unwrap());

    let _lexer = lexer::Lexer::new(chars);

    Ok(())
}
