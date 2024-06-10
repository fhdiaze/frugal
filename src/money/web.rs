use crate::infra::error::{AppError, AppResult};
use askama::Template;
use axum::{
  response::Html,
  routing::{get, post},
  Form, Router,
};

use super::convert;

#[derive(Template)]
#[template(path = "price/index.html")]
struct IndexTemplate;

async fn handle_index() -> AppResult<Html<String>> {
  let template = IndexTemplate {};
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "money/convert_in.html")]
struct ConvertInTemplate;

async fn handle_convert_get() -> AppResult<Html<String>> {
  let template = ConvertInTemplate {};
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "money/convert_out.html")]
struct ConvertOutTemplate {
  time: String,
}

async fn handle_convert_run(
  Form(cmd): Form<convert::Command>,
) -> AppResult<Html<String>> {
  let time = convert::handle(cmd);
  let template = ConvertOutTemplate {
    time: time.to_string(),
  };
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

pub fn route() -> Router {
  Router::new()
    .route("/money.index", get(handle_index))
    .route("/money.convert.get", get(handle_convert_get))
    .route("/money.convert.run", post(handle_convert_run))
}
