use clap::Parser;

use crate::infra::error::AppResult;

use self::price::PriceCmd;

mod price;

#[derive(Debug, Parser)]
#[command(no_binary_name = true)]
pub enum Command {
  Price(PriceCmd),
}

pub fn run(cmd: Command) -> AppResult<String> {
  match cmd {
    Command::Price(cmd) => price::run(cmd),
  }
}
