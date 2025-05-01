use crate::{infra::cosmos::DynDbClient, util::money::Money};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterCmd {
  amount: Money,
  description: String,
}

pub fn handle_register_cmd(_: RegisterCmd, _: DynDbClient) -> u64 {
  1
}
