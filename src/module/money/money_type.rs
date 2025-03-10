use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
pub struct Money(Inner);
type Inner = i64;

impl Money {
  const MIN_INNER: Inner = i64::MIN;
  const MAX_INNER: Inner = i64::MAX;

  /// Minimum allowable value for Money
  pub const fn min() -> Money {
    Money(Money::MIN_INNER)
  }

  /// Maximum allowable value for Money
  pub const fn max() -> Money {
    Money(Money::MAX_INNER)
  }

  pub fn from_minor(minor_amount: f64) -> Self {
    Money(minor_amount.round() as Inner)
  }

  const fn minor(&self) -> i64 {
    self.0 % 100
  }

  const fn major(&self) -> i64 {
    self.0 / 100
  }

  pub fn to_major(self) -> f64 {
    let major = self.major();
    let minor = self.minor();

    major as f64 + minor as f64 / 100.0
  }

  pub fn from_major(amount: f64) -> Money {
    Money((amount * 100.0).round() as Inner)
  }

  fn inner(&self) -> Inner {
    self.0
  }
}

impl Mul<f64> for Money {
  type Output = Money;

  fn mul(self, rhs: f64) -> Self::Output {
    Self::from_minor(self.0 as f64 * rhs)
  }
}

impl Mul<f32> for Money {
  type Output = Money;

  fn mul(self, rhs: f32) -> Self::Output {
    Self::from_minor(self.0 as f64 * rhs as f64)
  }
}

impl Mul<Money> for f64 {
  type Output = Money;

  fn mul(self, rhs: Money) -> Self::Output {
    Money::from_minor(self * rhs.0 as f64)
  }
}

impl Mul<Money> for f32 {
  type Output = Money;

  fn mul(self, rhs: Money) -> Self::Output {
    Money::from_major(self as f64 * rhs.to_major())
  }
}

impl Div<f64> for Money {
  type Output = Money;

  fn div(self, rhs: f64) -> Self::Output {
    Money::from_major(self.to_major() / rhs)
  }
}

impl Div<i64> for Money {
  type Output = Money;

  fn div(self, rhs: i64) -> Self::Output {
    Money(self.0 / rhs)
  }
}

impl Div<i32> for Money {
  type Output = Money;

  fn div(self, rhs: i32) -> Self::Output {
    Money(self.0 / rhs as i64)
  }
}

impl Div<f32> for Money {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    let inner = self.inner() as f32 / rhs;
    Money(inner.round() as i64)
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

impl Sub<Money> for Money {
  type Output = Money;

  fn sub(self, rhs: Money) -> Self::Output {
    Money(self.0 - rhs.0)
  }
}

impl Add<f64> for Money {
  type Output = Money;

  fn add(self, rhs: f64) -> Self::Output {
    self + Money::from_major(rhs)
  }
}

#[cfg(test)]
mod tests {
  use super::Money;

  #[test]
  fn test_mul() {
    assert_eq!(
      Money::from_major(3000.5) * 3.5_f64,
      Money::from_major(10501.75)
    );
  }

  #[test]
  fn test_div_f64() {
    assert_eq!(Money(87808) / 11_f64, Money(7983))
  }

  #[test]
  fn test_addition_success() {
    assert_eq!(Money(1) + Money(1), Money(2))
  }

  #[test]
  #[should_panic]
  #[allow(unused_must_use)]
  fn test_addition_failure_overflow_max() {
    Money::max() + Money(1);
  }

  #[test]
  #[should_panic]
  #[allow(unused_must_use)]
  fn test_addition_failure_overflow_min() {
    Money::min() + Money(-1);
  }

  #[test]
  fn test_subtraction_success() {
    assert_eq!(Money(2) - Money(1), Money(1))
  }

  #[test]
  #[should_panic]
  #[allow(unused_must_use)]
  fn test_subtraction_failure_overflow_max() {
    Money::max() - Money(-1);
  }

  #[test]
  #[should_panic]
  #[allow(unused_must_use)]
  fn test_subtraction_failure_overflow_min() {
    Money::min() - Money(1);
  }

  #[test]
  fn test_money_multiply_f64() {
    assert_eq!(Money(12300) * 2_f64, Money(24600))
  }

  #[test]
  fn test_f64_multiply_money() {
    assert_eq!(2_f64 * Money(12300), Money(24600))
  }

  #[test]
  fn test_money_div_f64() {
    assert_eq!(Money(12300) / 2_f64, Money(6150))
  }

  #[test]
  fn test_money_multiply_f32() {
    assert_eq!(Money(12300) * 2_f32, Money(24600))
  }

  #[test]
  fn test_f32_multiply_money() {
    assert_eq!(2_f32 * Money(12300), Money(24600))
  }

  #[test]
  fn test_money_div_f32() {
    assert_eq!(Money(12300) / 2_f32, Money(6150))
  }

  // Comparisons
  #[test]
  fn test_eq() {
    assert_eq!(Money(12300), Money(12300))
  }

  #[test]
  fn test_not_eq() {
    assert_ne!(Money(12300), Money(12400))
  }

  #[test]
  fn test_lt_eq() {
    assert!(Money(12300) <= Money(12300))
  }

  #[test]
  fn test_gt_eq() {
    assert!(Money(12300) >= Money(12300))
  }

  #[test]
  fn test_lt() {
    assert!(Money(12300) < Money(12400))
  }

  #[test]
  fn test_gt() {
    assert!(Money(12300) > Money(12200))
  }

  #[test]
  fn test_eq_inverse() {
    assert!(Money(12300) != Money(12301))
  }

  #[test]
  fn test_not_eq_inverse() {
    assert!(Money(12300) == Money(12300))
  }

  #[test]
  fn test_lt_eq_inverse() {
    assert!(Money(12300) > Money(12299))
  }

  #[test]
  fn test_gt_eq_inverse() {
    assert!(Money(12300) < Money(12301))
  }

  #[test]
  fn test_lt_inverse() {
    assert!(Money(12300) >= Money(12200))
  }

  #[test]
  fn test_gt_inverse() {
    assert!(Money(12300) <= Money(12400))
  }

  // Rounding vs. Truncation in Division
  #[test]
  fn test_rounded_division_f64() {
    assert_eq!(Money(87808) / 11.0_f64, Money(7983))
  }

  #[test]
  fn test_rounded_division_f32() {
    assert_eq!(Money(87808) / 11.0_f32, Money(7983))
  }

  // Precision loss
  #[test]
  fn test_precision_loss_i64() {
    assert_eq!(
      Money(9000000000000009900) / 10_i64,
      Money(900000000000000990)
    )
  }

  #[test]
  fn test_precision_loss_i32() {
    assert_eq!(
      Money(9000000000000009900) / 10_i32,
      Money(900000000000000990)
    )
  }
}
