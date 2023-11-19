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

    // 3. Parse string to structs
    let hosts = Hosts::from_str(&hosts_str);
    println!("hosts: {:?}", hosts);

    // 4. Validate and build requests from structs
    // let requests: Requests = Requests::build(hosts);
    // println!("requests: {:?}", requests);

    // 5. Execute requests and print to stdout
    // Requests::exec(requests)?;

    // request::exec(&config).await?;

    Ok(())
}
