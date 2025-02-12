use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use md5;
use chrono::{Utc, Duration};

use super::*;
use crate::service::Service;

pub async fn login_handler(
    service: &Service,
    Json(payload): Json<LoginRequest>,
) -> Json<TokenResponse> {
    // check MD5
    let computed_md5 = format!(
        "{:x}",
        md5::compute(payload.username.clone() + &payload.password)
    );
    if computed_md5 != payload.md5 {
        return Json(TokenResponse {
            token: None,
            message: "Md5 not matched".to_string(),
        });
    }

    // check user
    let pwd_md5 = format!("{:x}", md5::compute(payload.password));
    if let (false, msg) = service
        .database
        .check_user(&payload.username, &pwd_md5)
        .await
    {
        return Json(TokenResponse {
            token: None,
            message: msg,
        });
    }

    // get token
    // create expiring time
    let expiration = Utc::now()
    .checked_add_signed(Duration::days(3))
    .expect("valid timestamp")
    .timestamp() as usize;
    // create Claims
    let claims = Claims {
        sub: payload.username.clone(),
        company: "www.example.com".to_string(),
        exp: expiration,
    };
    // generate token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(service.jwt_secret_key.as_ref()),
    )
    .expect("Token creation failed");
    println!("jwt_secret_key: {}", service.jwt_secret_key);
    // return token
    Json(TokenResponse {
        token: Some(token),
        message: "Login successful".to_string(),
    })
}
