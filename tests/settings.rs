#[cfg(test)]
mod test {
    use corkscrew::types::cli::Cli;
    use corkscrew::Settings;
    use std::{error::Error, path::PathBuf};

    #[test]
    fn test_cli_config_defaults() -> Result<(), Box<dyn Error>> {
        let config: Settings = Cli {
            config_path: None,
            parallel: None,
            request_names: vec![],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("requests.yml"));
        assert_eq!(config.parallel, 0);
        assert_eq!(config.request_names, vec![] as Vec<String>);

        Ok(())
    }

    #[test]
    fn test_cli_config_only_path() -> Result<(), Box<dyn Error>> {
        let config: Settings = Cli {
            config_path: Some(PathBuf::from("test.yml")),
            parallel: None,
            request_names: vec![],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("test.yml"));
        assert_eq!(config.parallel, 0);
        assert_eq!(config.request_names, vec![] as Vec<String>);

        Ok(())
    }

    #[test]
    fn test_cli_config_only_parallel() -> Result<(), Box<dyn Error>> {
        let config: Settings = Cli {
            config_path: None,
            parallel: Some(4),
            request_names: vec![],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("requests.yml"));
        assert_eq!(config.parallel, 4);
        assert_eq!(config.request_names, vec![] as Vec<String>);

        Ok(())
    }

    #[test]
    fn test_cli_config_only_request_names() -> Result<(), Box<dyn Error>> {
        let config: Settings = Cli {
            config_path: None,
            parallel: None,
            request_names: vec![
                String::from("test_one"),
                String::from("test_two"),
                String::from("test_three"),
            ],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("requests.yml"));
        assert_eq!(config.parallel, 0);
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
    fn test_cli_config_combination() -> Result<(), Box<dyn Error>> {
        let config: Settings = Cli {
            config_path: Some(PathBuf::from("test.yml")),
            parallel: Some(8),
            request_names: vec![
                String::from("test_one"),
                String::from("test_two"),
                String::from("test_three"),
            ],
        }
        .try_into()?;

        assert_eq!(config.config_path, PathBuf::from("test.yml"));
        assert_eq!(config.parallel, 8);
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
