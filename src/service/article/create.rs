use super::*;
use crate::service::Service;
use axum::{http::HeaderMap, Json};

pub async fn create_article_handler(
    service: &Service,
    headers: HeaderMap,
    Json(article): Json<CreateArticle>,
) -> Json<CreateResponse> {
    //! Create a new article
    //!
    //! Create a new article with the given title, source, category, author, tags, origin and content
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let (valid, _, message, _) = service.check_jwt(token).await;
    if !valid {
        return Json(CreateResponse {
            result: false,
            uuid: "".to_string(),
            msg: message,
        });
    }
    let database = &service.database;
    let article_archive = &service.article_archive;
    let uuid = article_archive
        .new_article(article.content.clone())
        .unwrap();

    match database
        .create_article(
            &uuid,
            &article.title,
            &article.source,
            &article.category,
            &article.author,
            &article.tags,
            article.origin,
        )
        .await
    {
        Ok(_) => Json(CreateResponse {
            result: true,
            uuid,
            msg: "Article created".to_string(),
        }),
        Err(_) => Json(CreateResponse {
            result: false,
            uuid: "".to_string(),
            msg: "Article creation failed".to_string(),
        }),
    }
}
