use super::common::{self, Money};
use chrono::TimeDelta;
use serde::Serialize;

pub struct ConvertCmd {
  amount: Money,
  hourly_wage: Money,
}

impl ConvertCmd {
  pub fn new(amount: f64, hourly_rate: f64) -> Self {
    ConvertCmd {
      amount: common::from_major(amount),
      hourly_wage: common::from_major(hourly_rate),
    }
  }
}

#[derive(Serialize)]
pub struct Time {
  pub hours: isize,
}

pub fn run(cmd: ConvertCmd) -> TimeDelta {
  let hours = cmd.amount / cmd.hourly_wage;

  TimeDelta::hours(hours)
}
