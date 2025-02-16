use super::util;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Command {
  amount: f64,
  #[serde(rename(deserialize = "hourly-wage"))]
  hourly_wage: f64,
}

#[derive(Serialize)]
pub struct Time {
  pub hours: isize,
}

pub fn handle(cmd: Command) -> TimeDelta {
  let amount = util::from_major(cmd.amount);
  let hourly_wage = util::from_major(cmd.hourly_wage);
  let hours = amount / hourly_wage;

  TimeDelta::hours(hours)
}
