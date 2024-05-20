use serde::Serialize;

use super::Price;

#[derive(Serialize)]
pub struct UnitPrice {
    pub result: String,
}

pub fn run(cmi: Price) -> String {
    let unit = cmi.value as f64 / cmi.qty as f64;
    let price = UnitPrice {
        result: unit.to_string(),
    };

    price.result
}
