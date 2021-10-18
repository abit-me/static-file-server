use axum::{
    handler::{get, Handler},
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};

pub async fn handler_404() -> impl IntoResponse {

    (StatusCode::NOT_FOUND, "nothing to see here")
}