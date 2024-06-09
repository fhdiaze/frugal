use crate::infra::error::{AppError, AppResult};
use askama::Template;
use axum::{
  response::Html,
  routing::{get, post},
  Form, Router,
};

use super::scale;

#[derive(Template)]
#[template(path = "comps/price_index.html")]
struct IndexTemplate;

async fn handle_index() -> AppResult<Html<String>> {
  let template = IndexTemplate {};
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "comps/scale.html")]
struct ScaleTemplate {
  unit_price: f64,
}

async fn handle_scale(
  Form(cmd): Form<scale::Command>,
) -> AppResult<Html<String>> {
  let unit_price = scale::handle(cmd);
  let template = ScaleTemplate {
    unit_price: unit_price.amount,
  };
  let content = template.render().map_err(AppError::Render)?;

  Ok(Html(content))
}

pub fn route() -> Router {
  Router::new()
    .route("/price.index", get(handle_index))
    .route("/price.scale", post(handle_scale))
}
