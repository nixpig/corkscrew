use clap::Parser;
use std::{error::Error, fs, str::FromStr};

use corkscrew::{cli::Cli, config::Config, hosts::Hosts, requests::Requests};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Cli::parse().try_into()?;

    let hosts_str: String = fs::read_to_string(&config.config_path)?;

    let hosts = Hosts::from_str(&hosts_str)
        .expect("Configured hosts should be valid. See error(s) above for details.");
    // println!("hosts: {:#?}", hosts);

    // 4. Validate and build requests from structs
    // 5. Execute requests and print to stdout
    Requests::send_from_hosts(hosts, &config).await?;

    // request::exec(&config).await?;

    Ok(())
}
