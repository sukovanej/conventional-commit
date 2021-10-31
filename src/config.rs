use std::fmt;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub emoji: bool,
}

pub enum Error {
    ConfigFileNotFound,
    ParsingError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = match self {
            Error::ConfigFileNotFound => "config file not found",
            Error::ParsingError => "parsing error",
        };
        write!(f, "Config error: {}", error_message)
    }
}

impl Config {
    pub fn load_local(filename: &str) -> Result<Config, Error> {
        let config_content = fs::read_to_string(filename).map_err(|_| Error::ConfigFileNotFound)?;
        toml::from_str(config_content.as_str()).map_err(|_| Error::ParsingError)
    }

    pub fn default() -> Config {
        Config { emoji: false }
    }
}
