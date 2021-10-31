#![feature(result_flattening, try_blocks)]
use std::fs::File;

use crate::{
    config::Config,
    drawing::{pdf_maker::PdfMaker, DrawError, Drawer},
    parser::Content,
};

mod cli_args;
mod config;
mod drawing;
mod parser;
mod util;


fn main() -> Result<(), DrawError> {
    let args = cli_args::get();

    let mut config = Config::builder();

    if args.style.exists() {
        config.with_style(args.style);
    }

    if args.templates.iter().all(|p| p.exists()) {
        config.with_templates(args.templates);
    }

    let mut config = config.build(&args.doc_name);
    dbg!(&config);

    let source = std::fs::read_to_string(args.present_file).unwrap();
    let slides = parser::parse(&source);
    let mut pdf = PdfMaker::with_config(&config).expect("couldn't get the pdfmaker");

    for slide in slides {
        match slide.kind.as_str() {
            "Style" => {
                let path = slide
                    .contents
                    .into_iter()
                    .next()
                    .map(|c| match c {
                        Content::Config(p) => Some(p),
                        _ => None,
                    })
                    .flatten()
                    .expect("expected path to a style sheet");

                config
                    .change_style(path)
                    .expect("Couldn't load style sheet");
            }
            _ => pdf
                .create_slide(slide, &config)
                .expect("Could not create the slides due to"),
        }
    }

    let file = File::create(args.output).expect("couldn't open file");
    pdf.write(file)
}
