mod init;
mod user;

use crate::database::Database;
use axum::{http::HeaderMap, Json};
use user::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Service {
    database: Database,
    jwt_secret_key: String,
}

impl Service {
    pub async fn init() -> Self {
        //! Initialize the service with the environment variables and return the service object
        let (database, jwt_secret_key) = init::init().await;
        Self {
            database,
            jwt_secret_key,
        }
    }
}

impl Service {
    pub async fn login_handler(&self, Json(payload): Json<LoginRequest>) -> Json<TokenResponse> {
        user::auth::login_handler(&self, Json(payload)).await
    }

    async fn check_jwt(&self, token: &str) -> (bool, bool, String) {
        //! Return the validity and the admin status and the message
        //!
        //! Check the jwt token and return the validity and the admin status
        user::auth::auth_check(&self, token).await
    }

    pub async fn register_handler(
        &self,
        headers: HeaderMap,
        Json(payload): Json<RegisterRequest>,
    ) -> Json<RegisterResponse> {
        user::register::register_handler(&self, headers, Json(payload)).await
    }
}
