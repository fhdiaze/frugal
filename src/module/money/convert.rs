use super::common;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Command {
  amount: f64,
  hourly_wage: f64,
}

#[derive(Serialize)]
pub struct Time {
  pub hours: isize,
}

pub fn handle(cmd: Command) -> TimeDelta {
  let amount = common::from_major(cmd.amount);
  let hourly_wage = common::from_major(cmd.hourly_wage);
  let hours = amount / hourly_wage;

  TimeDelta::hours(hours)
}
