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
  #[arg(required = false)]
  pub items: isize,
  #[arg(required = false)]
  pub size: isize,
  #[arg(required = false)]
  pub cost: f64,
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
