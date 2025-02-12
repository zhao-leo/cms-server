pub mod auth;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
    md5: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    token: Option<String>,
    message: String,
}
