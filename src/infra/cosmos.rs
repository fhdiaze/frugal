use std::sync::Arc;

use super::config::Config;
use azure_data_cosmos::{clients::DatabaseClient, CosmosClient};
use azure_identity::DefaultAzureCredential;

pub type DynDbClient = Arc<dyn DbClient + Send + Sync>;

pub trait DbClient {
  fn database_client(&self) -> &DatabaseClient;
}

pub struct AzDbClient {
  db_client: DatabaseClient,
}

impl AzDbClient {
  pub fn new(config: &Config) -> AzDbClient {
    let credential =
      DefaultAzureCredential::new().expect("Failed to create azure credential");
    let client =
      CosmosClient::new(&config.db.connection_string, credential, None)
        .expect("Failed to create CosmosDb client");

    AzDbClient {
      db_client: client.database_client(&config.db.db_name),
    }
  }
}

impl DbClient for AzDbClient {
  fn database_client(&self) -> &DatabaseClient {
    &self.db_client
  }
}
