use clap::{command, Args, Parser, Subcommand};

use crate::infra::error::AppError;

mod scale;

#[derive(Debug, Args)]
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

#[derive(Debug, Clone, Parser)]
pub struct Price {
  pub items: isize,
  pub size: isize,
  pub cost: f64,
}

pub fn run(cmd: PriceCmd) -> Result<String, AppError> {
  let result = match cmd.subcommand {
    Some(scmd) => match scmd {
      PriceScmd::Scale(_) => "no price".to_string(),
    },
    None => match cmd.value {
      Some(value) => scale::run(value),
      None => scale::index()?,
    },
  };

  Ok(result)
}
