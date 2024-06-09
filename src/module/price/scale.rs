use askama::Template;
use serde::Serialize;

use crate::infra::error::AppError;

use super::Price;

#[derive(Serialize)]
pub struct UnitPrice {
  pub result: String,
}

#[derive(Template)]
#[template(path = "comps/scale.html")]
struct ScaleTemplate;

pub fn index() -> Result<String, AppError> {
  let template = ScaleTemplate {};
  let content = template.render().map_err(AppError::Render)?;

  Ok(content)
}

pub fn run(price: Price) -> String {
  let unit = price.cost / (price.items * price.size) as f64;
  let price = UnitPrice {
    result: unit.to_string(),
  };

  price.result
}
