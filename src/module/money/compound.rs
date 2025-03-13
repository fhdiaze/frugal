use serde::Deserialize;

use crate::infra::error::AppResult;

use super::money_type::Money;

#[derive(Debug, Deserialize)]
pub struct Command {
  principal: Money,
  contribution: f64,
  #[serde(rename(deserialize = "interest-rate"))]
  interest_rate: f64,
  years: i32,
  frequency: Frequency,
}

#[derive(Debug, Deserialize)]
pub enum Frequency {
  Daily,
  Monthly,
  Quarterly,
  Semiannually,
  Annually,
}

pub fn handle(cmd: Command) -> AppResult<Money> {
  let frequency_map = [365, 12, 4, 2, 1];
  let compounds_per_year = frequency_map[cmd.frequency as usize];
  let total_compounds = cmd.years * compounds_per_year;
  let rate_per_period = cmd.interest_rate / 100.0 / compounds_per_year as f64;

  let accumulated_amount = cmd.principal
    * (1.0 + rate_per_period).powi(total_compounds)
    + cmd.contribution * (1.0 + rate_per_period).powi(total_compounds);

  Ok(accumulated_amount)
}

#[cfg(test)]
mod tests {
  use crate::module::money::money_type::Money;

  use super::{handle, Command, Frequency};

  #[test]
  fn handle_should_success() {
    // Arrange
    let cmd = Command {
      principal: Money::from_major(1.0),
      contribution: 0.0,
      frequency: Frequency::Monthly,
      interest_rate: 1.0,
      years: 10,
    };

    // Act
    let result = handle(cmd);

    // Assert
    let expected_amount = 1.0 * (1.0 + 0.01_f64 / 12.0).powi(10 * 12);
    assert_eq!(result.unwrap(), Money::from_major(expected_amount));
  }
}
