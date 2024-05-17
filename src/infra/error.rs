use config as conf;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Config(conf::ConfigError),
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::NotFound(msg) => format!("NotFound: {}", msg),
            AppError::Validation(msg) => format!("Validation: {}", msg),
            AppError::Config(err) => format!("Config: {}", err),
        };

        write!(f, "{}", message)
    }
}
