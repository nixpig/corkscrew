use crate::cli::Cli;
use std::{error::Error, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub config_path: PathBuf,
    pub parallel: bool,
    pub request_names: Vec<String>,
}

impl TryFrom<Cli> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: Cli) -> Result<Self, Box<dyn Error>> {
        let parallel = value.parallel.unwrap_or(false);
        let config_file = value.config_path.unwrap_or(PathBuf::from("hosts.yml"));
        let request_names = value.request_names;

        Ok(Config {
            config_path: config_file,
            parallel,
            request_names,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::cli::Cli;
    use std::{error::Error, path::PathBuf};

    #[test]
    fn test_cli_config_defaults() -> Result<(), Box<dyn Error>> {
        let config: Config = Cli {
            config_path: None,
            parallel: None,
            request_names: vec![],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("hosts.yml"));
        assert!(!config.parallel);
        assert_eq!(config.request_names, vec![] as Vec<String>);

        Ok(())
    }

    #[test]
    fn test_cli_config_path() -> Result<(), Box<dyn Error>> {
        let config: Config = Cli {
            config_path: Some(PathBuf::from("test.yml")),
            parallel: None,
            request_names: vec![],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("test.yml"));
        assert!(!config.parallel);
        assert_eq!(config.request_names, vec![] as Vec<String>);

        Ok(())
    }

    #[test]
    fn test_cli_config_parallel() -> Result<(), Box<dyn Error>> {
        let config: Config = Cli {
            config_path: None,
            parallel: Some(true),
            request_names: vec![],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("hosts.yml"));
        assert!(config.parallel);
        assert_eq!(config.request_names, vec![] as Vec<String>);

        Ok(())
    }

    #[test]
    fn test_cli_config_request_names() -> Result<(), Box<dyn Error>> {
        let config: Config = Cli {
            config_path: None,
            parallel: None,
            request_names: vec![
                String::from("test_one"),
                String::from("test_two"),
                String::from("test_three"),
            ],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("hosts.yml"));
        assert!(!config.parallel);
        assert_eq!(
            config.request_names,
            vec![
                String::from("test_one"),
                String::from("test_two"),
                String::from("test_three"),
            ],
        );

        Ok(())
    }

    #[test]
    fn test_cli_config_combi() -> Result<(), Box<dyn Error>> {
        let config: Config = Cli {
            config_path: Some(PathBuf::from("test.yml")),
            parallel: Some(true),
            request_names: vec![
                String::from("test_one"),
                String::from("test_two"),
                String::from("test_three"),
            ],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("test.yml"));
        assert!(config.parallel);
        assert_eq!(
            config.request_names,
            vec![
                String::from("test_one"),
                String::from("test_two"),
                String::from("test_three"),
            ],
        );

        Ok(())
    }
}
