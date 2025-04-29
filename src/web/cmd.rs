use crate::infra::error::{AppError, AppResult};
use askama::Template;
use axum::{response::Html, routing::post, Form, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Run {}

#[derive(Template)]
#[template(path = "cmd/list.html")]
struct ListTemplate;

async fn handle_run(Form(_run): Form<Run>) -> AppResult<Html<String>> {
  let template = ListTemplate {};
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

pub fn route_cmd() -> Router {
  Router::new().route("/cmd.run", post(handle_run))
}
