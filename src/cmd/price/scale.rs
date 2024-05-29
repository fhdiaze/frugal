use serde::Serialize;

use super::Price;

#[derive(Serialize)]
pub struct UnitPrice {
  pub result: String,
}

pub fn run(price: Price) -> String {
  let unit = price.cost / (price.items * price.size) as f64;
  let price = UnitPrice {
    result: unit.to_string(),
  };

  price.result
}
