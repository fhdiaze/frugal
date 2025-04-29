use crate::util::money::Money;
use azure_data_cosmos::clients::DatabaseClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterCmd {
  amount: Money,
  description: String,
}

pub fn handle_register_cmd(_: RegisterCmd, _: &DatabaseClient) -> u64 {
  1
}
