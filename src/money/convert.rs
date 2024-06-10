use super::common::{self, Money};
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Command {
  amount: Money,
  hourly_wage: Money,
}

impl Command {
  pub fn new(amount: f64, hourly_rate: f64) -> Self {
    Command {
      amount: common::from_major(amount),
      hourly_wage: common::from_major(hourly_rate),
    }
  }
}

#[derive(Serialize)]
pub struct Time {
  pub hours: isize,
}

pub fn handle(cmd: Command) -> TimeDelta {
  let hours = cmd.amount / cmd.hourly_wage;

  TimeDelta::hours(hours)
}
