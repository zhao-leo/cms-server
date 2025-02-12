pub mod auth;
pub mod init;

use axum::{Router, routing::get};
use tokio;

use auth::login;

#[tokio::main]
pub async fn run() {
    init::init().await;
    let router = Router::new().route("/login", get(login::login_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
