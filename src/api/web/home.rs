use askama::Template;
use axum::{response::Html, routing::get, Router};

use crate::infra::error::{AppError, AppResult};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn handle_get() -> AppResult<Html<String>> {
  let template = HomeTemplate {};
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

pub fn route() -> Router {
  Router::new().route("/", get(handle_get))
}
