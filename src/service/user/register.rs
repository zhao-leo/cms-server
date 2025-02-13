use super::*;
use crate::service::Service;
use axum::{http::HeaderMap, Json};
use md5;

pub async fn register_handler(
    service: &Service,
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    //! Register a new user
    // check jwt
    let jwt_token = headers.get("Authorization").unwrap().to_str().unwrap();
    // check MD5
    let computed_md5 = format!(
        "{:x}",
        md5::compute(payload.username.clone() + &payload.password)
    );
    if computed_md5 != payload.md5 {
        return Json(RegisterResponse {
            result: false,
            msg: "Md5 not matched".to_string(),
        });
    }

    // check user
    let pwd_md5 = format!("{:x}", md5::compute(payload.password));
    let (valid, admin, msg) = service.check_jwt(jwt_token).await;
    if valid && admin {
        match service
            .database
            .create_user(&payload.username, &pwd_md5, payload.admin.clone())
            .await
        {
            Ok(_) => {
                return Json(RegisterResponse {
                    result: true,
                    msg: "User created".to_string(),
                });
            }
            Err(e) => {
                return Json(RegisterResponse {
                    result: false,
                    msg: e.to_string(),
                });
            }
        }
    } else {
        Json(RegisterResponse {
            result: false,
            msg: msg,
        })
    }
}
