use crate::{
    cmd::{self, Command},
    infra::error::AppError,
};
use askama::Template;
use axum::{response::Html, routing::post, Form, Router};
use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize)]
struct Run {
    cmd: String,
}

#[derive(Template)]
#[template(path = "output.html")]
struct OutputTemplate {
    result: String,
}

async fn handle_run(Form(run): Form<Run>) -> Result<Html<String>, AppError> {
    let result =
        Command::try_parse_from(run.cmd.split(' ')).map_or_else(|x| x.to_string(), cmd::run);
    let template = OutputTemplate { result };
    let content = template.render().map_err(AppError::Render)?;

    Ok(Html(content))
}

pub fn route() -> Router {
    Router::new().route("/cmd.run", post(handle_run))
}
