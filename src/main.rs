use std::fs::File;

use crate::{
    config::{Color, Config, StyleMap},
    drawing::{pdf_maker::PdfMaker, DrawError, Drawer},
};

mod config;
mod drawing;
mod parser;
mod util;

fn main() -> Result<(), DrawError> {
    //println!("Hello, world!");
    let file = std::env::args().nth(1).unwrap();
    println!("file read from: {}", file);
    let slides = parser::parse_file(file);
    let config = Config {
        colors: vec![
            Color(0.0, 0.0, 0.0, 1.0),
            Color(1.0, 0.0, 0.0, 1.0),
            Color(0.0, 1.0, 1.0, 1.0),
        ],
        doc_name: "hello world",
        slide_styles: StyleMap::new(),
        fg_idx: 0,
        bg_idx: 0,
        font: String::from("Sans Serif"),
    };

    let mut pdf = PdfMaker::with_config(&config);
    pdf.create_slides(slides, &config).unwrap();
    let file = File::open("output.pdf").expect("couldn't open file");
    pdf.write(file)
}
