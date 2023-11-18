use clap::Parser;
use std::{error::Error, fs, str::FromStr};

use corkscrew::{cli::Cli, config::Config, hosts::Hosts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Read config from cli
    let config: Config = Cli::parse().try_into()?;

    // 2a. Read file contents to string
    // 2b. in future - read stdin to string
    let hosts_str: String = fs::read_to_string(config.config_path)?;

    // Parse string to structs
    let hosts = Hosts::from_str(&hosts_str);

    // Validate and build requests from structs
    // Execute requests
    // format output
    // write output to stdout
    // 1. parser - parse input file (e.g. corkscrew.yml) to structs
    // 2. builder - build requests from structs
    // 3. writer - write output to stdout/file

    println!("{:?}", hosts);

    // request::exec(&config).await?;

    Ok(())
}
