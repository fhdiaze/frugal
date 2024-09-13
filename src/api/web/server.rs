use super::home;
use crate::{
  infra::{config::Config, logger},
  module::{cmd, money, price},
};
use axum::{
  http::{header::CONTENT_TYPE, Method},
  serve, Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
  cors::{Any, CorsLayer},
  services::ServeDir,
};
use tracing::info;

pub async fn start(config: &Config) {
  info!("Starting the web server!");

  let service_builder = ServiceBuilder::new()
    .layer(logger::trace_layer())
    .layer(cors_layer());
  let router = build_router().layer(service_builder);
  let address = SocketAddr::from(([0, 0, 0, 0], config.server.port));

  tracing::info!("Server is going to listen on address={}", address);

  let listener = TcpListener::bind(address)
    .await
    .expect("Error while binding the address!");

  serve(listener, router)
    .await
    .expect("Failed to start the web server");
}

pub fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE])
    .allow_origin(Any)
}

fn build_router() -> Router {
  Router::new()
    .nest("/", home::route())
    .nest("/", cmd::web::route())
    .nest("/", price::web::route())
    .nest("/", money::web::route())
    .nest_service("/assets", ServeDir::new("assets"))
}
