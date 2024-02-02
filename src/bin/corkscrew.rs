use clap::Parser;
use corkscrew::{run, Cli, Settings};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Parse settings from CLI args
    let settings: Settings = Cli::parse()
        .try_into()
        .expect("Failed to parse provided arguments. Run 'corkscrew --help' for help.");

    let results = run::go(settings).await?;

    for (name, result) in results {
        println!("{}\t{}\t{}", name, result.status(), result.url());
    }

    Ok(())
}
