use mysql::prelude::Queryable;
use mysql::*;

use crate::database::*;
pub enum FindType {
    Title,
    Source,
    Category,
    Author,
    Tags,
    Origin,
}

pub async fn create_article(
    database: &Database,
    uuid: &str,
    title: &str,
    source: &str,
    category: &str,
    author: &str,
    tags: &str,
    origin: bool,
) -> Result<(), mysql::Error> {
    // Create a new article
    let mut conn = database.pool.get_conn().unwrap();
    conn.exec_drop(
        r"INSERT INTO articles (uuid, title, source, category, author, tags, origin)
        VALUES (:uuid, :title, :source, :category, :author, :tags, :origin)",
        params! {
            "uuid" => uuid,
            "title" => title,
            "source" => source,
            "category" => category,
            "author" => author,
            "tags" => tags,
            "origin" => origin,
        },
    )
    .unwrap();
    drop(conn);
    Ok(())
}

pub async fn delete_article(database: &Database, uuid: &str) -> Result<(), mysql::Error> {
    //! Delete an article by the given uuid
    let mut conn = database.pool.get_conn().unwrap();
    conn.exec_drop(
        r"DELETE FROM articles WHERE uuid = :uuid",
        params! {
            "uuid" => uuid,
        },
    )
    .unwrap();
    drop(conn);
    Ok(())
}

pub async fn find_article(
    database: &Database,
    find_type: FindType,
    find: &str,
) -> Result<Vec<String>, mysql::Error> {
    //! Find an article by the given type
    let mut conn = database.pool.get_conn().unwrap();
    let query = match find_type {
        FindType::Title => format!("SELECT * FROM articles WHERE title = '{}'", find),
        FindType::Source => format!("SELECT * FROM articles WHERE source = '{}'", find),
        FindType::Category => format!("SELECT * FROM articles WHERE category = '{}'", find),
        FindType::Author => format!("SELECT * FROM articles WHERE author = '{}'", find),
        FindType::Tags => format!("SELECT * FROM articles WHERE tags = '{}'", find),
        FindType::Origin => format!("SELECT * FROM articles WHERE origin = '{}'", find),
    };
    let articles: Vec<String> = conn.query_map(
        query,
        |(_, uuid, _, _, _, _, _, _, _): (
            u64,
            String,
            String,
            String,
            String,
            String,
            String,
            bool,
            String,
        )| { format!("{}", uuid) },
    )?;
    drop(conn);
    Ok(articles)
}

pub async fn put_article(
    database: &Database,
    uuid: &str,
    title: &str,
    source: &str,
    category: &str,
    author: &str,
    tags: &str,
    origin: bool,
) -> Result<(), mysql::Error> {
    //! Update an article by the given uuid
    let mut conn = database.pool.get_conn().unwrap();
    conn.exec_drop(
        r"UPDATE articles SET title = :title, source = :source, category = :category, author = :author, tags = :tags, origin = :origin WHERE uuid = :uuid",
        params! {
            "uuid" => uuid,
            "title" => title,
            "source" => source,
            "category" => category,
            "author" => author,
            "tags" => tags,
            "origin" => origin,
        },
    )
    .unwrap();
    drop(conn);
    Ok(())
}
