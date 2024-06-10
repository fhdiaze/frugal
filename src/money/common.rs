pub type Money = i64;

pub fn from_major(amount: f64) -> Money {
  let rounded = (amount * 100.0).round() / 100.0;
  let cents = rounded * 100.0;

  cents as Money
}
