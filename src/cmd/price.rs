use clap::{command, Args, Parser, Subcommand};

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

pub fn run(cmd: PriceCmd) -> String {
  if let Some(scmd) = cmd.subcommand {
    match scmd {
      PriceScmd::Scale(_) => "no price".to_string(),
    }
  } else if let Some(value) = cmd.value {
    scale::run(value)
  } else {
    "error: a value is required for '[MSISDN]' but none was supplied For more information, try '--help'".to_string()
  }
}
