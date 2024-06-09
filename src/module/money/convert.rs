use askama::Template;
use chrono::TimeDelta;
use serde::Serialize;

use super::common::{self, Money};
use crate::infra::error::AppError;

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

#[derive(Template)]
#[template(path = "comps/convert.html")]
struct ConvertTemplate;

pub fn index() -> Result<String, AppError> {
  let template = ConvertTemplate {};
  let content = template.render().map_err(AppError::Render)?;

  Ok(content)
}

pub fn run(cmd: ConvertCmd) -> TimeDelta {
  let hours = cmd.amount / cmd.hourly_wage;

  TimeDelta::hours(hours)
}
