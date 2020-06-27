use sha2::Digest;
use crate::hrdb::utils::*;

pub struct Branch {
    pub versions: Vec<String> // Page
}

impl Branch {
    pub async fn from(name: &str) -> Result<Branch, String> {
        Ok(Branch { versions: list(name).await? })
    }

    pub async fn fork(&self, version: &str, name: String) -> Result<Branch, String> {
        if let Ok(_) = read(&name).await {
            return Err(format!("A Branch named {} already exists", name));
        }

        let position = self.versions.iter().position(|x| x == version)
            .ok_or("Could not find the specified version to fork at on this Branch")?;

        return Ok(Branch { versions: self.versions[..=position].to_vec() });
    }

    pub fn merge(&mut self, from: &Branch) -> Result<String, String> {
        // find last common ancestor, favoring self
        // enumerate all page ids
        // ...
        todo!()
    }

    pub fn head(&self) -> Result<String, String> {
        let h = self.versions.last().ok_or("No versions on this Branch")?;
        return Ok(h.to_string());
    }
}
