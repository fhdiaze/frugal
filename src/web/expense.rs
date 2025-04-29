use crate::{
  commands,
  infra::{
    cosmos::DynCosmosClient,
    error::{AppError, AppResult},
  },
};
use askama::Template;
use axum::{
  extract::State,
  response::Html,
  routing::{get, post},
  Form, Router,
};

#[derive(Template)]
#[template(path = "expense/register.html")]
struct ExpenseTemplate {}

async fn handle_register_get() -> AppResult<Html<String>> {
  let template = ExpenseTemplate {};
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

#[derive(Template)]
#[template(path = "expense/register_out.html")]
struct ExpenseOutTemplate {
  expense_id: u64,
}

#[axum::debug_handler]
async fn handle_register_run(
  State(db): State<DynCosmosClient>,
  Form(cmd): Form<commands::RegisterCmd>,
) -> AppResult<Html<String>> {
  let expense_id = commands::handle_register_cmd(cmd, &db);
  let template = ExpenseOutTemplate { expense_id };
  let content = template.render().map_err(AppError::RenderingError)?;

  Ok(Html(content))
}

pub fn route_expense() -> Router<DynCosmosClient> {
  Router::new()
    .route("/expense.register.get", get(handle_register_get))
    .route("/expense.register.run", post(handle_register_run))
}
