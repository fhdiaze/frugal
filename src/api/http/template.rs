use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub struct HtmlTemplate<T> {
    pub content: T,
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.content.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error {}", err),
            )
                .into_response(),
        }
    }
}
