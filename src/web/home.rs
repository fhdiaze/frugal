use crate::infra::error::{AppError, AppResult};
use askama::Template;
use axum::{response::Html, routing::get, Router};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn handle_get() -> AppResult<Html<String>> {
  let template = HomeTemplate {};
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

pub fn route_home() -> Router {
  Router::new().route("/", get(handle_get))
}
