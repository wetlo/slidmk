pub enum Token {
    Linefeed,
    Text(String),
    Identifier(String),
    ListPre(u8),
}
