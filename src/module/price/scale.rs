pub struct UnitPrice {
  pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct Command {
  /// Grams, weight or units per item
  pub size: isize,
  /// Cost of the items
  pub cost: f64,
  /// Number of items included for the price
  pub items: isize,
}

pub fn handle(cmd: Command) -> UnitPrice {
  let unit_price = cmd.cost / (cmd.items * cmd.size) as f64;

  UnitPrice { amount: unit_price }
}
