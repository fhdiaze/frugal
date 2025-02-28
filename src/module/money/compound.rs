use serde::Deserialize;

use crate::infra::error::AppResult;

use super::util::Money;

#[derive(Debug, Deserialize)]
pub struct Command {
  amount: f64,
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
  let mut amount = Money::from_major(cmd.amount);
  let compounds = cmd.years * 12;

  for _ in 0..compounds {
    let interest_rate = cmd.interest_rate / 100.0;
    let monthly_interest_rate = interest_rate / 12.0;
    let interest = amount * monthly_interest_rate;
    amount = amount + interest + cmd.contribution;
  }

  Ok(amount)
}
