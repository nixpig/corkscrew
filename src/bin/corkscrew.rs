use clap::Parser;
use cli_table::{print_stdout, WithTitle};
use corkscrew::{cli::Cli, config::Config, requests::Requests};
use std::{error::Error, fs, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Cli::parse().try_into()?;

    let requests = Requests::from_str(&fs::read_to_string(&config.config_path)?)?;

    let output = requests.exec(&config).await?;

    let _ = print_stdout(output.with_title());

    Ok(())
}
