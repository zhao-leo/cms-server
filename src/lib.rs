pub mod database;
pub mod service;

pub use database::Database;
pub use service::Service;

use axum::{
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio;

#[tokio::main]
pub async fn run() {
    //! Initialize the service
    //!
    //! Initialize the service and start the server
    let service = Arc::new(Service::init().await);

    let router = Router::new();
    let router = register_login(router, "/login", service.clone());
    let router = register_register(router, "/register", service.clone());
    let router = register_modify(router, "/modify-user", service.clone());

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
        post(move |headers: HeaderMap, Json(payload)| {
            let service = service.clone();
            async move { service.register_handler(headers, Json(payload)).await }
        }),
    )
}

fn register_modify(router: Router, path: &str, service: Arc<Service>) -> Router {
    router.route(
        path,
        post(move |headers: HeaderMap, Json(payload)| {
            let service = service.clone();
            async move { service.modify_handler(headers, Json(payload)).await }
        }),
    )
}
