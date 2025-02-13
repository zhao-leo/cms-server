pub mod create;
pub mod delete;
pub mod modify;
pub mod search;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticle {
    // id: i32,
    // uuid: String,
    title: String,
    source: String,
    category: String,
    author: String,
    tags: String,
    origin: bool,
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    result: bool,
    uuid: String,
    msg: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteArticle {
    uuid: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    result: bool,
    msg: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyResponse {
    result: bool,
    msg: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyArticle {
    // id: i32,
    uuid: String,
    title: String,
    source: String,
    category: String,
    author: String,
    tags: String,
    origin: bool,
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    search: Vec<String>,
    result: bool,
    msg: String,
}
