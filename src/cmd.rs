use clap::Parser;

use self::price::PriceCmd;

mod price;

#[derive(Debug, Parser)]
#[command(no_binary_name = true, arg_required_else_help = true)]
pub enum Command {
  Price(PriceCmd),
}

pub fn run(cmd: Command) -> String {
  match cmd {
    Command::Price(cmd) => price::run(cmd),
  }
}
