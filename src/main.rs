use infra::{config::Config, logger};

mod api;
mod infra;
mod module;

#[tokio::main]
async fn main() {
  let config = Config::new().expect("Failed to load configuration");
  logger::add_logger(&config);
  api::web::server::start(&config).await;
}
