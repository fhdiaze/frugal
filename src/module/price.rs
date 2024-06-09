use clap::{command, Args, Subcommand};

use crate::infra::error::AppError;

mod scale;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PriceCmd {
  #[clap(flatten)]
  value: Option<Price>,
  #[command(subcommand)]
  subcommand: Option<PriceScmd>,
}

#[derive(Subcommand, Debug)]
enum PriceScmd {
  Scale(Price),
}

#[derive(Debug, Clone, Args)]
pub struct Price {
  /// Grams, weight or units per item
  #[arg(required = false)]
  pub size: isize,
  /// Cost of the items
  #[arg(required = false)]
  pub cost: f64,
  /// Number of items included for the price
  #[arg(required = false, default_value_t = 1)]
  pub items: isize,
}

pub fn run(cmd: PriceCmd) -> Result<String, AppError> {
  let result = match cmd.subcommand {
    Some(scmd) => match scmd {
      PriceScmd::Scale(price) => scale::run(price),
    },
    None => match cmd.value {
      Some(value) => scale::run(value),
      None => scale::index()?,
    },
  };

  Ok(result)
}
