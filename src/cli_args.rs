use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opts {
    #[structopt(short, long)]
    pub style: Option<PathBuf>,
    #[structopt(short, long)]
    pub templates: Vec<PathBuf>,
    #[structopt(short, long, default_value = "out.pdf")]
    pub output: PathBuf,
    #[structopt(name = "FILE")]
    pub present_file: PathBuf,
}
