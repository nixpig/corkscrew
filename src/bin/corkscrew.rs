use clap::Parser;
use corkscrew::{printer, run, Cli, Settings};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Parse settings from CLI args
    let settings: Settings = Cli::parse()
        .try_into()
        .expect("Failed to parse provided arguments. Run 'corkscrew --help' for help.");

    let results = run::go(settings).await?;

    printer::print(results);

    Ok(())
}
