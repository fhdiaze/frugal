use crate::{
  commands::{self, Frequency},
  infra::error::{AppError, AppResult},
  util::money::Money,
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
#[template(path = "money/convert.html")]
struct ConvertTemplate;

async fn handle_convert_get() -> AppResult<Html<String>> {
  let template = ConvertTemplate {};
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "money/convert_out.html")]
struct ConvertOutTemplate {
  time: String,
}

async fn handle_convert_run(
  Form(cmd): Form<commands::ConvertCmd>,
) -> AppResult<Html<String>> {
  let time = commands::handle_convert_cmd(cmd);
  let template = ConvertOutTemplate {
    time: time.num_hours().to_string(),
  };
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "money/compound.html")]
struct CompoundTemplate {
  frequencies: Vec<String>,
}

async fn handle_compound_get() -> AppResult<Html<String>> {
  let frequencies = vec![
    Frequency::Daily,
    Frequency::Monthly,
    Frequency::Quarterly,
    Frequency::Semiannually,
    Frequency::Annually,
  ];
  let template = CompoundTemplate {
    frequencies: frequencies
      .into_iter()
      .map(|x| format!("{:?}", x))
      .collect(),
  };
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "money/compound_out.html")]
struct CompoundOutTemplate {
  amount: f64,
}

async fn handle_compound_run(
  Form(cmd): Form<commands::CompoundCmd>,
) -> AppResult<Html<String>> {
  let amount = commands::handle_compound_cmd(cmd)?;
  let template = CompoundOutTemplate {
    amount: Money::to_major(amount),
  };
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

pub fn route_money() -> Router {
  Router::new()
    .route("/money.index", get(handle_index))
    .route("/money.convert.get", get(handle_convert_get))
    .route("/money.convert.run", post(handle_convert_run))
    .route("/money.compound.get", get(handle_compound_get))
    .route("/money.compound.run", post(handle_compound_run))
}
