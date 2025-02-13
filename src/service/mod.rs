mod article;
mod init;
mod user;

use crate::{articles::Articles, database::Database};
use axum::{http::HeaderMap, Json};
use user::*;

#[derive(Clone)]
pub struct Service {
    database: Database,
    jwt_secret_key: String,
    article_archive: Articles,
}

impl Service {
    pub async fn init() -> Self {
        //! Initialize the service with the environment variables and return the service object
        let (database, jwt_secret_key, articles) = init::init().await;
        Self {
            database,
            jwt_secret_key,
            article_archive: articles,
        }
    }
}

impl Service {
    async fn check_jwt(&self, token: &str) -> (bool, bool, String, String) {
        //! Return the validity and the admin status and the message
        //!
        //! Check the jwt token and return the validity and the admin status
        user::auth::auth_check(&self, token).await
    }

    pub async fn login_handler(&self, Json(payload): Json<LoginRequest>) -> Json<TokenResponse> {
        user::auth::login_handler(&self, Json(payload)).await
    }

    pub async fn register_handler(
        &self,
        headers: HeaderMap,
        Json(payload): Json<RegisterRequest>,
    ) -> Json<RegisterResponse> {
        user::register::register_handler(&self, headers, Json(payload)).await
    }

    pub async fn modify_handler(
        &self,
        headers: HeaderMap,
        Json(payload): Json<MotifyRequest>,
    ) -> Json<MotifyResponse> {
        user::modify::modify_handler(&self, headers, Json(payload)).await
    }
}

impl Service {
    pub async fn create_article_handler(
        &self,
        headers: HeaderMap,
        Json(article): Json<article::CreateArticle>,
    ) -> Json<article::CreateResponse> {
        //! Create a new article
        //!
        //! Create a new article with the given title, source, category, author, tags, origin and content
        article::create::create_article_handler(&self, headers, Json(article)).await
    }

    pub async fn delete_article_handler(
        &self,
        headers: HeaderMap,
        Json(uuid): Json<article::DeleteArticle>,
    ) -> Json<article::DeleteResponse> {
        //! Delete an article
        //!
        //! Delete an article by the given uuid
        article::delete::delete_article_handler(&self, headers, Json(uuid)).await
    }

    pub async fn modify_article_handler(
        &self,
        headers: HeaderMap,
        Json(article): Json<article::ModifyArticle>,
    ) -> Json<article::ModifyResponse> {
        //! Modify an article
        //!
        //! Modify an article by the given uuid title, source, category, author, tags, origin and content
        article::modify::modify_article_handler(&self, headers, Json(article)).await
    }

    pub async fn search_article_handler(
        &self,
        headers: HeaderMap,
        Json(search): Json<serde_json::Value>,
    ) -> Json<article::SearchResponse> {
        //! Search for an article
        //!
        //! Search for an article by the given search
        article::search::search_article_handler(&self, headers, Json(search)).await
    }
}
