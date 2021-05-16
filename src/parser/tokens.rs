use std::path::PathBuf;

#[derive(Debug)]
pub enum Token {
    Linefeed,
    Exclamation,
    SqrBracketLeft,
    SqrBracketRight,
    RoundBracketLeft,
    RoundBracketRight,
    Path(PathBuf),
    Text(String),
    Identifier(String),
    ListPre(u8),
}
