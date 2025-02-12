use mysql::{prelude::Queryable, *};
pub async fn init() {
    let db_user: &str = &std::env::var("DB_USER").unwrap();
    let db_pass: &str = &std::env::var("DB_PASS").unwrap();
    let db_name: &str = &std::env::var("DB_NAME").unwrap();
    let db_host: &str = &std::env::var("DB_HOST").unwrap();
    let db_port: &str = &std::env::var("DB_PORT").unwrap();

    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name
    );
    let pool = Pool::new(db_url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();

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
            author TEXT,
            tags TEXT,
            category TEXT NOT NULL,
            origin BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .unwrap();
    println!("Database initialized");
    drop(conn);
}
