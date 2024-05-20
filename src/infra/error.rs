use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use config as conf;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Config(conf::ConfigError),
    Render(askama::Error),
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::NotFound(msg) => format!("NotFound: {}", msg),
            AppError::Validation(msg) => format!("Validation: {}", msg),
            AppError::Config(err) => format!("Config: {}", err),
            AppError::Render(err) => format!("Render: {}", err),
        };

        write!(f, "{}", message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl From<askama::Error> for AppError {
    fn from(cause: askama::Error) -> Self {
        AppError::Render(cause)
    }
}
