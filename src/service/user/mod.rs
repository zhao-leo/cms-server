pub mod auth;
pub mod register;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
    admin: bool,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    admin: bool,
    md5: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    result: bool,
    msg: String,
}
