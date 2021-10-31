use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    style: Option<PathBuf>,
    #[structopt(short, long)]
    templates: Vec<PathBuf>,
    #[structopt(short, long, default_value = "out.pdf")]
    output: PathBuf,
    #[structopt(name = "FILE")]
    present_file: PathBuf,
    #[structopt(short = "n", default_value = "presentation")]
    docname: String,
}

pub struct CliArgs {
    pub style: PathBuf,
    pub templates: Vec<PathBuf>,
    pub output: PathBuf,
    pub present_file: PathBuf,
    pub doc_name: String,
}

fn get_project_dir() -> directories::ProjectDirs {
    directories::ProjectDirs::from("org", "wetlo", "slidmk")
        .expect("Unknown operating system, couldn't find a good project directory")
}

pub fn get() -> CliArgs {
    let mut opts = Opts::from_args();
    let dir = get_project_dir();
    
    opts.templates.push(dir.config_dir().join("template.hjson"));
    
    CliArgs {
        doc_name: opts.docname,
        output: opts.output,
        present_file: opts.present_file,
        templates: opts.templates,
        style: opts.style.unwrap_or_else(|| dir.config_dir().join("style.hjson")),
    }
}
