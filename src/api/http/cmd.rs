use crate::cmd::{self, Command};
use askama::Template;
use axum::{response::IntoResponse, routing::post, Form, Router};
use clap::Parser;
use serde::Deserialize;

use super::template::HtmlTemplate;

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
        Command::try_parse_from(run.cmd.split(' ')).map_or_else(|x| x.to_string(), cmd::run);

    let template = OutputTemplate { content };

    HtmlTemplate { content: template }
}

pub fn route() -> Router {
    Router::new().route("/cmd.run", post(handle_run))
}
