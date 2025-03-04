mod articles;
mod init;
mod user;

pub use articles::FindType;

use mysql::Pool;
#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
    // pub db_url: String,
}

impl Database {
    //! Database struct to hold the database connection pool and the database url
    pub fn new(db_url: &str) -> Self {
        //! Create a new Database struct
        Database {
            pool: Pool::new(db_url).unwrap(),
            // db_url: db_url.to_string(),
        }
    }

    pub async fn init(&self) -> Result<(), mysql::Error> {
        //! Initialize the database
        init::init(&self).await
    }
}
impl Database {
    //! User functions
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        admin: bool,
    ) -> Result<(), mysql::Error> {
        //! Create a new user
        user::create_user(&self, username, password, admin).await
    }

    pub async fn check_user(&self, username: &str, password: &str) -> (bool, String, bool) {
        //! Check if the user exists
        user::check_user(&self, username, password).await
    }

    // async fn get_user(
    //     &self,
    //     username: &str,
    // ) -> Result<Vec<(u32, String, String, bool)>, mysql::Error> {
    //     //! Get the user
    //     user::get_user(&self, username).await
    // }

    pub async fn put_user(
        &self,
        username: &str,
        password: &str,
        admin: bool,
    ) -> Result<(), mysql::Error> {
        //! Put the user
        user::put_user(&self, username, password, admin).await
    }
}

impl Database {
    //! Article functions
    pub async fn create_article(
        &self,
        uuid: &str,
        title: &str,
        source: &str,
        category: &str,
        author: &str,
        tags: &str,
        origin: bool,
    ) -> Result<(), mysql::Error> {
        //! Create a new article
        articles::create_article(self, uuid, title, source, category, author, tags, origin).await
    }

    pub async fn delete_article(&self, uuid: &str) -> Result<(), mysql::Error> {
        //! Delete an article by the given uuid
        articles::delete_article(self, uuid).await
    }

    pub async fn find_article(
        &self,
        find_type: articles::FindType,
        find: &str,
    ) -> Result<Vec<String>, mysql::Error> {
        //! Find an article by the given type
        articles::find_article(self, find_type, find).await
    }

    pub async fn put_article(
        &self,
        uuid: &str,
        title: &str,
        source: &str,
        category: &str,
        author: &str,
        tags: &str,
        origin: bool,
    ) -> Result<(), mysql::Error> {
        //! Put an article by the given uuid
        articles::put_article(self, uuid, title, source, category, author, tags, origin).await
    }
}
