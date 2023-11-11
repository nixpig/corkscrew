use crate::cli::Cli;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, error::Error, fs::File, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub config_file: PathBuf,
    pub parallel: bool,
    pub hosts: Hosts,
}

impl TryFrom<Cli> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: Cli) -> Result<Self, Box<dyn Error>> {
        let parallel = value.parallel.unwrap_or(false);

        let config_file = value
            .config_path
            .unwrap_or_else(|| PathBuf::from("corkscrew.yml"));

        let f = File::open(&config_file)?;
        let hosts: Hosts = serde_yaml::from_reader(f)?;

        Ok(Config {
            config_file,
            parallel,
            hosts,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hosts(Vec<Host>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    host: String,
    scheme: String,
    port: i32,
    requests: Vec<Request>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    name: String,
    path: String,
    method: String,
    params: HashMap<String, String>,
    response: Vec<String>,
}
