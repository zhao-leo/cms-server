mod init;
mod user;

use crate::database::Database;
use axum::Json;
use user::{LoginRequest, TokenResponse};

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
}
