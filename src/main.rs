mod upload;
use tokio;
use axum::{
    handler::{get},
    Router,
};

use std::{net::SocketAddr};
mod path;
mod show;
mod icon;
mod download;
mod bad;


#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_multipart_form=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    path::create();

    // build our application with some routes
    let app = Router::new()
        .route("/", get(show::show_files))
        .route("/favicon.ico", get(icon::favicon))
        //.route("/download/:filename", get(download::down))
        .route("/:filename", get(download::down))
        .route("/upload", get(upload::show_form).post(upload::accept_form))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
    tracing::debug!("listening on {}", addr);
    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}