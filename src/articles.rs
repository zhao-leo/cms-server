use std::io::Write;
#[derive(Debug, Clone)]
pub struct Articles {
    abs_post_dir: String,
}

impl Articles {
    pub fn new(abs_post_dir: String) -> Self {
        Self { abs_post_dir }
    }

    pub fn new_article(&self, content: String) -> Result<String, std::io::Error> {
        //! Create a new article
        //!
        //! Create a new article with the given uuid and content
        let mut uuid = uuid::Uuid::new_v4().to_string();
        while std::path::Path::new(&format!("{}/{}.html", self.abs_post_dir, uuid)).exists() {
            uuid = uuid::Uuid::new_v4().to_string();
        }
        let mut file = std::fs::File::create(format!("{}/{}.html", self.abs_post_dir, uuid))?;
        file.write_all(content.as_bytes())?;
        Ok(uuid)
    }

    pub fn delete_article(&self, uuid: String) -> Result<(), std::io::Error> {
        //! Delete an article by the given uuid
        //!
        //! Delete an article by the given uuid
        std::fs::remove_file(format!("{}/{}.html", self.abs_post_dir, uuid))?;
        Ok(())
    }

    pub fn motify_article(&self, uuid: String, content: String) -> Result<(), std::io::Error> {
        //! Modify an article by the given uuid
        //!
        //! Modify an article by the given uuid
        let mut file = std::fs::File::create(format!("{}/{}.html", self.abs_post_dir, uuid))?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
