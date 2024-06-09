use clap::{command, Parser, Subcommand};
use money::MoneyCmd;

use crate::infra::error::AppResult;

use self::price::PriceCmd;

mod money;
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
  Money(MoneyCmd),
}

pub fn run(cmd: Frugal) -> AppResult<String> {
  match cmd.command {
    Command::Price(cmd) => price::run(cmd),
    Command::Money(cmd) => money::run(cmd),
  }
}
