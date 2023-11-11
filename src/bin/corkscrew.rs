use clap::Parser;
use std::error::Error;

use corkscrew::{cli::Cli, config::Config, request};

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Cli::parse().try_into()?;

    request::exec(&config);

    Ok(())
}
