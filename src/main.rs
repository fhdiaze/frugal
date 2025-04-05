use api::web::server;
use infra::{config::Config, logger};

mod api;
mod core;
mod infra;
mod util;

#[tokio::main]
async fn main() {
  let config = Config::new().expect("Failed to load configuration");
  logger::add_logger(&config);
  server::start(&config).await;
}
