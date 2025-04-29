use infra::{config::Config, logger};

mod commands;
mod infra;
mod util;
mod web;

#[tokio::main]
async fn main() {
  let config = Config::new().expect("Failed to load configuration");
  logger::add_logger(&config);
  web::start(&config).await;
}
