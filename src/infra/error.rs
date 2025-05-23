use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
  RenderingError(askama::Error),
}

/// Short hand for [`Result`] type
///
/// [`Result`]: std::result::Result
pub type AppResult<T> = Result<T, AppError>;

impl Error for AppError {}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let message = match self {
      AppError::RenderingError(err) => format!("Render: {}", err),
    };

    write!(f, "{}", message)
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
  }
}
