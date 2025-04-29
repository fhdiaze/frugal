use std::sync::Arc;

use super::config::Config;
use azure_data_cosmos::{clients::DatabaseClient, CosmosClient};
use azure_identity::DefaultAzureCredential;

pub type DynCosmosClient = Arc<DatabaseClient>;

pub fn new(config: &Config) -> DatabaseClient {
  let credential =
    DefaultAzureCredential::new().expect("Failed to create azure credential");
  let client =
    CosmosClient::new(&config.db.connection_string, credential, None)
      .expect("Failed to create CosmosDb client");

  client.database_client(&config.db.db_name)
}
