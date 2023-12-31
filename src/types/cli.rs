use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "@nixpig",
    version,
    about = "Configure HTTP requests in YAML and execute from the command line."
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

    #[clap(
        short = 'p',
        long = "parallel",
        name = "num",
        help = "Specify number of parallel requests"
    )]
    pub parallel: Option<usize>,
}
