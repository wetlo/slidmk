use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Linefeed,
    SqrBracketLeft,
    SqrBracketRight,
    Path(PathBuf),
    Text(String),
    Identifier(String),
    ListPre(u8),
}
