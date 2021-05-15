use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

mod lexer;
mod tokens;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<()> {
    // open a reader to read every single character inside the file
    let strings = BufReader::new(File::open(path)?)
        .lines()
        .map(|s| s.expect("Couldn't read the file"));

    let _lexer = lexer::Lexer::new(strings);

    Ok(())
}
