use crate::database::Database;

pub async fn init() -> (Database, String) {
    // environment variables check
    let db_user = std::env::var("DB_USER").unwrap();
    let db_pass = std::env::var("DB_PASS").unwrap();
    let db_name = std::env::var("DB_NAME").unwrap();
    let db_host = std::env::var("DB_HOST").unwrap();
    let db_port = std::env::var("DB_PORT").unwrap();

    if let Err(_) = &std::env::var("JWT_SECRET_KEY") {
        println!("JWT_SECRET_KEY not found, using default");
        std::env::set_var("JWT_SECRET_KEY", "1a2b3c4d5e6f7g8h9ijklmnopqrstuvwxyz");
    }
    let jwt_key = std::env::var("JWT_SECRET_KEY").unwrap();

    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name
    );
    let db = Database::new(&db_url);

    db.init().await.unwrap();
    (db, jwt_key)
}
