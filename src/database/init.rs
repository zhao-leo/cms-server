use super::Database;
use mysql::prelude::Queryable;

pub async fn init(database: &Database) -> Result<(), mysql::Error> {
    //! Initialize the database
    let mut conn = database.pool.get_conn().unwrap();

    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL,
            admin BOOLEAN DEFAULT FALSE
        )",
    )
    .unwrap();
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS articles (
            id INT AUTO_INCREMENT PRIMARY KEY,
            uuid VARCHAR(255) NOT NULL,
            title VARCHAR(255) NOT NULL,
            source TEXT NOT NULL,
            category TEXT NOT NULL,
            author TEXT,
            tags TEXT,
            origin BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .unwrap();
    println!("Database initialized");
    drop(conn);
    Ok(())
}
