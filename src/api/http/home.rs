use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn handle_get() -> impl IntoResponse {
    let template = HomeTemplate {};

    HtmlTemplate { content: template }
}

pub fn route() -> Router {
    Router::new().route("/", get(handle_get))
}
