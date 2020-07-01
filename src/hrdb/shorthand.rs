use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use crate::hrdb::utils::*;

/// `Shorthand` maps name to a version, id, and bloom filter which can be used to create a `Location`.
#[derive(Serialize, Deserialize)]
pub struct Shorthand(HashMap<String, (usize, String, String)>);

impl Shorthand {
    pub async fn read() -> Result<Shorthand, String> {
        serde_json::from_str(&read("shorthand").await?)
            .ok().ok_or("Could not deserialize Shorthand".to_owned())
    }

    pub fn wrap(map: HashMap<String, (usize, String, String)>) -> Shorthand {
        Shorthand(map)
    }

    pub fn unwrap(self) -> HashMap<String, (usize, String, String)> {
        self.0
    }

    pub fn to_string(&self) -> Result<String, String> {
        let serialized = serde_json::to_string_pretty(self)
            .ok().ok_or("Could not serialize Shorthand")?;
        return Ok(serialized);
    }

    pub async fn write(&self) -> Result<(), String> {
        mutate("shorthand", &self.to_string()?).await
    }

    pub async fn update(name: String, ver_no: usize, id: String) -> Result<(), String> {
        let mut table = Shorthand::read().await?.unwrap();
        table.insert(name, (ver_no, id, "".to_owned()));
        Shorthand::wrap(table).write().await?;
        return Ok(());
    }
}
