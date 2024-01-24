use serde::{Deserialize, Serialize};

use crate::types::cli::Cli;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub config_path: PathBuf,
    pub parallel: usize,
    pub request_names: Vec<String>,
}

impl TryFrom<Cli> for Settings {
    type Error = Box<dyn Error>;

    fn try_from(value: Cli) -> Result<Self, Box<dyn Error>> {
        let parallel = value.parallel.unwrap_or(0);
        let config_path = value.config_path.unwrap_or(PathBuf::from("requests.yml"));
        let request_names = value.request_names;

        Ok(Settings {
            config_path,
            parallel,
            request_names,
        })
    }
}
