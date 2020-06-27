use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use sha2::Digest;
use crate::hrdb::utils::*;

#[derive(Serialize, Deserialize)]
pub struct Page {
    id:           String,
    pub fields:   HashMap<String, String>,
    pub title:    String,
    pub content:  String, // Content
    pub children: HashMap<String, String>, // id -> page checksum
}

impl Page {
    pub async fn from(hash: &str) -> Result<Page, String> {
        serde_json::from_str(&read(&hash).await?)
            .ok().ok_or("Could not deserialize Page".to_owned())
    }

    pub fn new(title: String, content: String, fields: HashMap<String, String>) -> Page {
        Page {
            id: stamp().unwrap(),
            fields,
            title,
            content,
            children: HashMap::new(),
        }
    }

    pub fn same(&self, other: Page) -> bool {
        self.id == other.id
    }

    pub fn to_string(&self) -> Result<String, String> {
        let serialized = serde_json::to_string_pretty(self)
            .ok().ok_or("Could not serialize Page")?;
        return Ok(serialized);
    }

    pub fn short(&self) -> String {
        self.title
            .split_whitespace()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join("-")
            .chars()
            .filter(|x| x.is_ascii_alphanumeric() || x == &'-')
            .collect::<String>()
            .to_lowercase()
    }

    pub fn id(&self) -> String { self.id.to_owned() }
}
