use clap::Parser;
use std::error::Error;

use corkscrew::{cli::Cli, config::Config, request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. parser - parse input file (e.g. corkscrew.yml) to structs
    // 2. builder - build requests from structs
    // 3. writer - write output to stdout/file

    let config: Config = Cli::parse().try_into()?;

    request::exec(&config).await?;

    Ok(())
}
