use std::path::PathBuf;

#[derive(Debug)]
pub struct Slide {
    pub kind: String,
    pub contents: Vec<Content>,
}

#[derive(Debug)]
pub enum Content {
    Text(String),
    Config(PathBuf),
    Image(String, PathBuf),
    List(Vec<(u8, String)>),
}
