use std::{fmt::Display, io::Error, process::exit};

#[derive(Debug)]
pub enum CorkscrewError {
    Cli(Error),
    FsRead,
    Parse,
    Request,
}

impl Display for CorkscrewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorkscrewError::Cli(e) => {
                write!(f, "Failed to parse CLI arguments. {}", e)
            }
            CorkscrewError::FsRead => {
                write!(f, "unable to read file")
            }
            CorkscrewError::Parse => {
                write!(f, "unable to parse yaml")
            }
            CorkscrewError::Request => {
                write!(f, "unable to execute request")
            }
        }
    }
}

pub fn handle_error(err: CorkscrewError) {
    eprintln!("ERROR: {}", err);

    exit(1);
}
