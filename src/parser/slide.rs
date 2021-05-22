use std::path::PathBuf;

pub struct Slide {
    pub kind: String,
    pub contents: Vec<Content>,
}

pub enum Content {
    Text(String),
    Path(PathBuf),
    Image(String, PathBuf),
    List(Vec<(u8, String)>),
}
