use super::*;

use crate::service::Service;

use axum::{http::HeaderMap, Json};

pub async fn modify_article_handler(
    service: &Service,
    headers: HeaderMap,
    Json(article): Json<ModifyArticle>,
) -> Json<ModifyResponse> {
    //! Modify an article
    //!
    //! Modify an article by the given uuid
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let (valid, _, message, _) = service.check_jwt(token).await;
    if !valid {
        return Json(ModifyResponse {
            result: false,
            msg: message,
        });
    }
    let database = &service.database;
    let article_archive = &service.article_archive;
    match database
        .put_article(
            &article.uuid,
            &article.title,
            &article.source,
            &article.category,
            &article.author,
            &article.tags,
            article.origin,
        )
        .await
    {
        Ok(_) => {
            article_archive
                .motify_article(article.uuid.clone(), article.content.clone())
                .unwrap();
            Json(ModifyResponse {
                result: true,
                msg: "Article modified".to_string(),
            })
        }
        Err(_) => Json(ModifyResponse {
            result: false,
            msg: "Article modification failed".to_string(),
        }),
    }
}
