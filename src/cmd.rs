use clap::{command, Parser, Subcommand};

use crate::infra::error::AppResult;

use self::price::PriceCmd;

mod price;

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true, multicall = true)]
pub struct Frugal {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  Price(PriceCmd),
}

pub fn run(cmd: Frugal) -> AppResult<String> {
  match cmd.command {
    Command::Price(cmd) => price::run(cmd),
  }
}
