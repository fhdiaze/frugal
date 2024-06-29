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
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

pub fn route() -> Router {
  Router::new().route("/cmd.run", post(handle_run))
}
