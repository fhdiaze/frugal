use std::str::FromStr;

use tower_http::{
  classify::{ServerErrorsAsFailures, SharedClassifier},
  trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use super::config::Config;

pub fn add_logger(config: &Config) {
  let filter_layer = EnvFilter::default()
    .add_directive(Level::from_str(&config.log.level).unwrap().into());
  tracing_subscriber::fmt()
    .with_env_filter(filter_layer)
    .with_target(false)
    .compact()
    .init();
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
  TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
    .on_request(DefaultOnRequest::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO))
}
