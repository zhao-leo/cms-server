use axum::{response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use md5;
use serde::{Deserialize, Serialize};
use std::env;
use mysql::{prelude::Queryable, *};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
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
struct TokenResponse {
    token: Option<String>,
    message: String,
}

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // check MD5
    let computed_md5 = format!(
        "{:x}",
        md5::compute(payload.username.clone()+&payload.password)
    );
    if computed_md5 != payload.md5 {
        return Json(TokenResponse {
            token: None,
            message: "Md5 not matched".to_string(),
        });
    }

    // check user
    let pwd_md5 = format!("{:x}", md5::compute(payload.password));
    if let (false,msg) = check_user(&payload.username, &pwd_md5) {
        return Json(TokenResponse {
            token: None,
            message: msg,
        });
    }

    // get token
    // get secret key
    let secret_key = env::var("JWT_SECRET_KEY").unwrap_or("1a2b3c4d5e6f7g8h9ijklmnopqrstuvwxyz".to_string());
    // create Claims
    let claims = Claims {
        sub: payload.username.clone(),
        company: "www.dingkeji.com".to_string(),
        exp: 10000000000, // Set to expire
    };
    // generate token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("Token creation failed");

    // return token
    Json(TokenResponse {
        token: Some(token),
        message: "Login successful".to_string(),
    })
}

fn check_user(username: &str, password_md5: &str) -> (bool, String) {
    let db_user: &str = &std::env::var("DB_USER").unwrap();
    let db_pass: &str = &std::env::var("DB_PASS").unwrap();
    let db_name: &str = &std::env::var("DB_NAME").unwrap();
    let db_host: &str = &std::env::var("DB_HOST").unwrap();
    let db_port: &str = &std::env::var("DB_PORT").unwrap();
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name
    );
    let pool = Pool::new(db_url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let query = format!(
        "SELECT * FROM users WHERE username='{}' AND password='{}'",
        username, password_md5
    );
    match conn.query_map(query, |(id, username, password, admin): (u32, String, String, bool)| {
        (id, username, password, admin)
    }) {
        Ok(result) => {
            if result.len() > 0 {
                return (true, "User found".to_string());
            }
        }
        Err(e) => {
            return (false, e.to_string());
        }
    }

    drop(conn);
    (false, "Uncorrect username or password".to_string())
}