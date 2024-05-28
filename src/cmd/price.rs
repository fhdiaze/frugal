use clap::{command, Args, Parser, Subcommand};

mod compare;

#[derive(Debug, Args)]
pub struct PriceCmd {
  #[clap(flatten)]
  value: Option<Price>,
  #[command(subcommand)]
  subcommand: Option<PriceScmd>,
}

#[derive(Subcommand, Debug)]
enum PriceScmd {
  Unify,
}

#[derive(Debug, Clone, Parser)]
pub struct Price {
  pub qty: isize,
  pub t: isize,
  pub value: isize,
}

pub fn run(cmd: PriceCmd) -> String {
  if let Some(scmd) = cmd.subcommand {
    match scmd {
      PriceScmd::Unify => "no price".to_string(),
    }
  } else if let Some(value) = cmd.value {
    compare::run(value)
  } else {
    "error: a value is required for '[MSISDN]' but none was supplied For more information, try '--help'".to_string()
  }
}
