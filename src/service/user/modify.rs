use super::*;
use crate::service::Service;
use axum::{http::HeaderMap, Json};
use md5;

pub async fn modify_handler(
    service: &Service,
    headers: HeaderMap,
    Json(payload): Json<MotifyRequest>,
) -> Json<MotifyResponse> {
    //! Modify a user
    // check jwt
    let jwt_token = headers.get("Authorization").unwrap().to_str().unwrap();
    // check MD5

    let pwd_md5 = format!("{:x}", md5::compute(payload.password));
    if pwd_md5 != payload.md5 {
        return Json(MotifyResponse {
            result: false,
            msg: "Md5 not matched".to_string(),
        });
    }

    let (valid, admin, msg, username) = service.check_jwt(jwt_token).await;
    if valid {
        match service.database.put_user(&username, &pwd_md5, admin).await {
            Ok(_) => {
                return Json(MotifyResponse {
                    result: true,
                    msg: "User Updated".to_string(),
                });
            }
            Err(e) => {
                return Json(MotifyResponse {
                    result: false,
                    msg: e.to_string(),
                });
            }
        }
    } else {
        Json(MotifyResponse { result: false, msg })
    }
}
