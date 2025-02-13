use mysql::prelude::Queryable;
use mysql::*;

use crate::database::*;

pub async fn create_user(
    database: &Database,
    username: &str,
    password_md5: &str,
    admin: bool,
) -> Result<(), mysql::Error> {
    //! Create a new user
    let mut conn = database.pool.get_conn().unwrap();
    let chech_result = get_user(database, username).await;
    match chech_result {
        Ok(result) => {
            if result.len() > 0 {
                return Err(mysql::Error::MySqlError(MySqlError {
                    state: "23000".to_string(),
                    message: "Key already exists".to_string(),
                    code: 1062,
                }));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    // Insert the new user
    let query = format!(
        "INSERT INTO users (username, password, admin) VALUES ('{}', '{}', {})",
        username, password_md5, admin
    );
    conn.query_drop(query).unwrap();
    drop(conn);
    Ok(())
}

pub async fn check_user(
    database: &Database,
    username: &str,
    password_md5: &str,
) -> (bool, String, bool) {
    let mut conn = database.pool.get_conn().unwrap();

    let query = format!(
        "SELECT * FROM users WHERE username='{}' AND password='{}'",
        username, password_md5
    );
    match conn.query_map(
        query,
        |(id, username, password, admin): (u32, String, String, bool)| {
            (id, username, password, admin)
        },
    ) {
        Ok(result) => {
            if result.len() > 0 {
                return (true, "User found".to_string(), result[0].3);
            }
        }
        Err(e) => {
            return (false, e.to_string(), false);
        }
    }

    drop(conn);
    (false, "Uncorrect username or password".to_string(), false)
}

pub async fn get_user(
    database: &Database,
    username: &str,
) -> Result<Vec<(u32, String, String, bool)>, mysql::Error> {
    let mut conn = database.pool.get_conn().unwrap();

    let query = format!("SELECT * FROM users WHERE username='{}'", username);
    match conn.query_map(
        query,
        |(id, username, password_md5, admin): (u32, String, String, bool)| {
            (id, username, password_md5, admin)
        },
    ) {
        Ok(result) => {
            drop(conn);
            return Ok(result);
        }
        Err(e) => {
            drop(conn);
            return Err(e);
        }
    }
}

pub async fn put_user(
    database: &Database,
    username: &str,
    password_md5: &str,
    admin: bool,
) -> Result<(), mysql::Error> {
    //! Create a new user
    let mut conn = database.pool.get_conn().unwrap();
    let chech_result = get_user(database, username).await;
    match chech_result {
        Ok(result) => {
            if result.len() == 0 {
                return Err(mysql::Error::MySqlError(MySqlError {
                    state: "23000".to_string(),
                    message: "Key not exists".to_string(),
                    code: 1062,
                }));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    let query = format!(
        "UPDATE users SET password='{}', admin={} WHERE username='{}'",
        password_md5, admin, username
    );
    conn.query_drop(query).unwrap();
    drop(conn);
    Ok(())
}
