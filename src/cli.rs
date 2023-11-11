use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
// #[clap()]
pub struct Cli {
    pub request_names: Vec<String>,

    #[clap(short = 'c', long = "config")]
    pub config_path: Option<PathBuf>,

    #[clap(short = 'p', long = "parallel")]
    pub parallel: Option<bool>,
}
