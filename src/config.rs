use crate::cli::Cli;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub config_file: PathBuf,
    pub parallel: bool,
    pub request_names: Vec<String>,
    pub hosts: Vec<Host>,
}

impl TryFrom<Cli> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: Cli) -> Result<Self, Box<dyn Error>> {
        let parallel = value.parallel.unwrap_or(false);

        let config_file = value.config_path.unwrap();

        let f = File::open(&config_file)?;
        let hosts: Vec<Host> = serde_yaml::from_reader(f)?;

        Ok(Config {
            config_file,
            request_names: value.request_names,
            parallel,
            hosts,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub host: String,
    pub scheme: String,
    pub port: i32,
    pub requests: Vec<Request>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub path: String,
    pub method: String,
    pub params: HashMap<String, String>,
    pub response: Vec<String>,
}
