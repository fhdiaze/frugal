use crate::{
  core::price::scale,
  infra::error::{AppError, AppResult},
};
use askama::Template;
use axum::{
  response::Html,
  routing::{get, post},
  Form, Router,
};

#[derive(Template)]
#[template(path = "price/index.html")]
struct IndexTemplate;

async fn handle_index() -> AppResult<Html<String>> {
  let template = IndexTemplate {};
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "price/scale.html")]
struct ScaleTemplate;

async fn handle_scale_get() -> AppResult<Html<String>> {
  let template = ScaleTemplate {};
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "price/scale_out.html")]
struct ScaleOutTemplate {
  unit_price: f64,
}

async fn handle_scale_run(
  Form(cmd): Form<scale::Command>,
) -> AppResult<Html<String>> {
  let unit_price = scale::handle(&cmd);
  let template = ScaleOutTemplate {
    unit_price: unit_price.amount,
  };
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

pub fn route() -> Router {
  Router::new()
    .route("/price.index", get(handle_index))
    .route("/price.scale.get", get(handle_scale_get))
    .route("/price.scale.run", post(handle_scale_run))
}
