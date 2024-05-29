use askama::Template;
use axum::{response::Html, routing::get, Router};

use crate::infra::error::AppError;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn handle_get() -> Result<Html<String>, AppError> {
  let template = HomeTemplate {};
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

pub fn route() -> Router {
  Router::new().route("/", get(handle_get))
}
