use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    style: Option<PathBuf>,
    #[structopt(short, long)]
    templates: Vec<PathBuf>,
    #[structopt(name = "FILE")]
    present_file: PathBuf,
}
