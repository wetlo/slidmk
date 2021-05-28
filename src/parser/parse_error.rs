use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
};

#[derive(Debug)]
pub struct ParseError<'a> {
    pub expected: &'a str,
    pub actual: String,
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(
            f,
            "Expected token: {}, token found {}",
            self.expected, self.actual
        )
    }
}
impl<'a> Error for ParseError<'a> {}
