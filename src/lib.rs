pub mod database;
pub mod service;

pub use database::Database;
pub use service::Service;

use axum::{http::HeaderMap, routing::get, Json, Router};
use std::sync::Arc;
use tokio;

#[tokio::main]
pub async fn run() {
    let service = Arc::new(Service::init().await);

    let router = Router::new();
    let router = register_login(router, "/login", service.clone());
    let router = register_register(router, "/register", service.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

fn register_login(router: Router, path: &str, service: Arc<Service>) -> Router {
    router.route(
        path,
        get(move |Json(payload)| {
            let service = service.clone();
            async move { service.login_handler(Json(payload)).await }
        }),
    )
}

fn register_register(router: Router, path: &str, service: Arc<Service>) -> Router {
    router.route(
        path,
        get(move |headers: HeaderMap, Json(payload)| {
            let service = service.clone();
            async move { service.register_handler(headers, Json(payload)).await }
        }),
    )
}
