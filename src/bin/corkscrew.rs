use clap::Parser;
use std::error::Error;

use corkscrew::{cli::Cli, config::Config, corkscrew::run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Cli::parse().try_into()?;

    run(config);

    Ok(())
}
