use super::*;

use crate::service::Service;

use axum::{http::HeaderMap, Json};

pub async fn delete_article_handler(
    service: &Service,
    headers: HeaderMap,
    Json(uuid): Json<DeleteArticle>,
) -> Json<DeleteResponse> {
    //! Delete an article
    //!
    //! Delete an article by the given uuid
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let (valid, _, message, _) = service.check_jwt(token).await;
    if !valid {
        return Json(DeleteResponse {
            result: false,
            msg: message,
        });
    }
    let database = &service.database;
    let article_archive = &service.article_archive;
    match database.delete_article(&uuid.uuid).await {
        Ok(_) => {
            article_archive.delete_article(uuid.uuid.clone()).unwrap();
            Json(DeleteResponse {
                result: true,
                msg: "Article deleted".to_string(),
            })
        }
        Err(_) => Json(DeleteResponse {
            result: false,
            msg: "Article deletion failed".to_string(),
        }),
    }
}
