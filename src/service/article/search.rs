use super::*;
use crate::{database::FindType, service::Service};
use axum::{http::HeaderMap, Json};

pub async fn search_article_handler(
    service: &Service,
    headers: HeaderMap,
    Json(search): Json<serde_json::Value>,
) -> Json<SearchResponse> {
    //! Search for articles
    //!
    //! Search for articles with the given title, source, category, author, tags, origin and content
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let (valid, _, message, _) = service.check_jwt(token).await;
    if !valid {
        return Json(SearchResponse {
            result: false,
            search: vec![],
            msg: message,
        });
    }
    let database = &service.database;
    match search["type"].as_str() {
        Some("title") => {
            match database
                .find_article(FindType::Title, search["content"].as_str().unwrap())
                .await
            {
                Ok(article) => Json(SearchResponse {
                    search: article,
                    result: true,
                    msg: "Article found".to_string(),
                }),
                Err(_) => Json(SearchResponse {
                    search: vec![],
                    result: false,
                    msg: "Article not found".to_string(),
                }),
            }
        }
        Some("source") => {
            match database
                .find_article(FindType::Source, search["content"].as_str().unwrap())
                .await
            {
                Ok(article) => Json(SearchResponse {
                    search: article,
                    result: true,
                    msg: "Article found".to_string(),
                }),
                Err(_) => Json(SearchResponse {
                    search: vec![],
                    result: false,
                    msg: "Article not found".to_string(),
                }),
            }
        }
        Some("category") => {
            match database
                .find_article(FindType::Category, search["content"].as_str().unwrap())
                .await
            {
                Ok(article) => Json(SearchResponse {
                    search: article,
                    result: true,
                    msg: "Article found".to_string(),
                }),
                Err(_) => Json(SearchResponse {
                    search: vec![],
                    result: false,
                    msg: "Article not found".to_string(),
                }),
            }
        }
        Some("author") => {
            match database
                .find_article(FindType::Author, search["content"].as_str().unwrap())
                .await
            {
                Ok(article) => Json(SearchResponse {
                    search: article,
                    result: true,
                    msg: "Article found".to_string(),
                }),
                Err(_) => Json(SearchResponse {
                    search: vec![],
                    result: false,
                    msg: "Article not found".to_string(),
                }),
            }
        }
        Some("tags") => {
            match database
                .find_article(FindType::Tags, search["content"].as_str().unwrap())
                .await
            {
                Ok(article) => Json(SearchResponse {
                    search: article,
                    result: true,
                    msg: "Article found".to_string(),
                }),
                Err(_) => Json(SearchResponse {
                    search: vec![],
                    result: false,
                    msg: "Article not found".to_string(),
                }),
            }
        }
        Some("origin") => {
            match database
                .find_article(FindType::Origin, search["content"].as_str().unwrap())
                .await
            {
                Ok(article) => Json(SearchResponse {
                    search: article,
                    result: true,
                    msg: "Article found".to_string(),
                }),
                Err(_) => Json(SearchResponse {
                    search: vec![],
                    result: false,
                    msg: "Article not found".to_string(),
                }),
            }
        }
        _ => Json(SearchResponse {
            search: vec![],
            result: false,
            msg: "Error searching type".to_string(),
        }),
    }
}
