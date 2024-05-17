use super::template::HtmlTemplate;
use crate::module::{self, Command};
use askama::Template;
use axum::{response::IntoResponse, routing::post, Form, Router};
use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize)]
struct Run {
    cmd: String,
}

#[derive(Template)]
#[template(path = "output.html")]
struct OutputTemplate {
    content: String,
}

async fn handle_run(Form(run): Form<Run>) -> impl IntoResponse {
    let content =
        Command::try_parse_from(run.cmd.split(' ')).map_or_else(|x| x.to_string(), module::run);

    let template = OutputTemplate { content };

    HtmlTemplate { content: template }
}

pub fn route() -> Router {
    Router::new().route("/cmd.run", post(handle_run))
}
