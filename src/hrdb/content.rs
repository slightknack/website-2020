use crate::hrdb::utils::*;

pub struct Content(pub String);

impl Content {
    pub async fn from(hash: &str) -> Result<Content, String> {
        Ok(Content(read(hash).await?))
    }

    pub fn new(content: String) -> Content {
        Content(content)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
