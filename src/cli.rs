use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    author = "@nixpig",
    version,
    about = "A simple tool for executing HTTP requests defined in a YAML config. Written in Rust, btw."
)]
pub struct Cli {
    pub request_names: Vec<String>,

    #[clap(short = 'f', long = "file", default_value = "corkscrew.yml")]
    pub config_path: Option<PathBuf>,

    #[clap(short = 'p', long = "parallel")]
    pub parallel: Option<bool>,
}
