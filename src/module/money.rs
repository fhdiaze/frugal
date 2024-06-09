use crate::infra::error::AppError;
use clap::{command, Args, Subcommand};

mod common;
mod convert;

#[derive(Debug, Args)]
pub struct MoneyCmd {
  /// The amount of money
  pub amount: f64,
  #[command(subcommand)]
  pub subcommand: MoneyScmd,
}

#[derive(Subcommand, Debug, Clone)]
pub enum MoneyScmd {
  Convert { hourly_wage: f64 },
}

pub fn run(cmd: MoneyCmd) -> Result<String, AppError> {
  let result = match cmd.subcommand {
    MoneyScmd::Convert { hourly_wage } => {
      let c = convert::ConvertCmd::new(cmd.amount, hourly_wage);
      convert::run(c)
    }
  };

  Ok(result.num_hours().to_string())
}
