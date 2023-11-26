use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    author = "@nixpig",
    version,
    about = "A simple tool for executing HTTP requests defined in a YAML config. Written in Rust, btw."
)]
pub struct Cli {
    pub request_names: Vec<String>,

    #[clap(
        short = 'f',
        long = "file",
        default_value = "requests.yml",
        name = "file_path",
        help = "Path to file containing requests"
    )]
    pub config_path: Option<PathBuf>,

    #[clap(short = 'p', long = "parallel", action=ArgAction::SetTrue, help="Run requests in parallel")]
    pub parallel: Option<bool>,
}
