use serde::Serialize;

use super::Price;

#[derive(Serialize)]
pub struct UnitPrice {
  pub result: String,
}

#[derive(Template)]
#[template(path = "comps/index.html")]
struct OutputTemplate {
  result: String,
}

pub fn run() -> String {
  let unit = price.cost / (price.items * price.size) as f64;
  let price = UnitPrice {
    result: unit.to_string(),
  };

  price.result
}
