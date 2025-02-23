use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Money(i64);

impl Money {
  pub fn from_minor(minor_amount: f64) -> Self {
    Money(minor_amount.round() as i64)
  }

  pub fn to_major(self) -> f64 {
    self.0 as f64 / 100.0
  }

  pub fn from_major(amount: f64) -> Money {
    let rounded = (amount * 100.0).round() / 100.0;

    Money((rounded * 100.0) as i64)
  }
}

impl Mul<f64> for Money {
  type Output = Money;

  fn mul(self, rhs: f64) -> Self::Output {
    Money((self.0 as f64 * rhs).round() as i64)
  }
}

impl Div<f64> for Money {
  type Output = Money;

  fn div(self, rhs: f64) -> Self::Output {
    Money((self.0 as f64 / rhs).round() as i64)
  }
}

impl Div<Money> for Money {
  type Output = f64;

  fn div(self, rhs: Money) -> Self::Output {
    self.0 as f64 / rhs.0 as f64
  }
}

impl Add<Money> for Money {
  type Output = Money;

  fn add(self, rhs: Money) -> Self::Output {
    Money(self.0 + rhs.0)
  }
}

impl Add<f64> for Money {
  type Output = Money;

  fn add(self, rhs: f64) -> Self::Output {
    self + Money::from_major(rhs)
  }
}
