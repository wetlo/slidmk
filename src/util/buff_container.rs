use std::io::{BufRead, Result};
use utf8_chars::BufReadCharsExt;

pub struct BuffContainer<R: BufRead> {
    reader: R,
}

impl<R: BufRead> BuffContainer<R> {
    pub fn new(r: R) -> Self {
        Self { reader: r }
    }
}

impl<R: BufRead> Iterator for BuffContainer<R> {
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader
            .read_char_raw()
            .map_err(|e| e.into_io_error())
            .transpose()
    }
}
