use clap::Parser;
use std::error::Error;

use corkscrew::{cli::Cli, config::Config};

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Config = Cli::parse().try_into()?;
    println!("{:?}", cli);

    Ok(())
}
