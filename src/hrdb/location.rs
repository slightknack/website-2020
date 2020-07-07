use serde::{Serialize, Deserialize};
use sha2::Digest;
use crate::hrdb::controller;
use crate::hrdb::page::Page;

/// The location of a specific page at a specific version on a specific branch.
/// branch is the name of the branch,
/// version is the hash of the root,
/// path is a list of checksums.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location((String, Option<(String, Option<Vec<String>>)>));

impl Location {
    pub fn from_branch(branch: String) -> Location {
        Location((branch, None))
    }

    pub fn from_branch_and_version(branch: String, version: String) -> Location {
        Location((branch, Some((version, None))))
    }

    pub fn from_branch_version_and_path(branch: String, version: String, path: Vec<String>) -> Location {
        if path.is_empty() { panic!("Can not create location with empty path") };
        Location((branch, Some((version, Some(path)))))
    }

    pub fn branch(&self) -> String {
        (self.0).0.to_string()
    }

    pub fn version(&self) -> Result<String, String> {
        Ok(((self.0).1).clone().ok_or("Location does not specify a version")?.0)
    }

    pub async fn ver_no(&self) -> Result<usize, String> {
        let version = self.version()?;
        controller::versions(self.clone()).await?
            .iter().position(|l| match l.version() {
                Ok(v)  => v == version,
                Err(e) => false,
            })
            .ok_or("Could not find version number".to_owned())
    }

    pub fn path(&self) -> Result<Vec<String>, String> {
        Ok(
            ((self.0).1).clone()
            .ok_or("Location does not specify a version")?.1.clone()
            .ok_or("Location does not specify a path")?
        )
    }

    pub fn back(&self) -> Result<Location, String> {
        let mut path = self.path()?;
        path.pop().ok_or("Can not have empty path")?;
        if path.is_empty() { return Err("Can not go back past root".to_owned()) };

        Ok(
            Location::from_branch_version_and_path(
                self.branch().clone(),
                self.version()?.clone(),
                path,
            )
        )
    }

    pub fn forward(&self, address: String) -> Result<Location, String> {
        let mut path = self.path()?;
        path.push(address);

        Ok(
            Location::from_branch_version_and_path(
                self.branch().clone(),
                self.version()?.clone(),
                path,
            )
        )
    }

    pub fn end(&self) -> Result<String, String> {
        Ok(
            self.path()?.last()
                .ok_or("Path has no addresses")?
                .to_string()
        )
    }

    pub async fn ids(&self) -> Result<Vec<String>, String> {
        let mut ids = vec![];
        for a in self.path()?.iter() {
            let page = Page::from(a).await?;
            ids.push(page.id());
        }

        return Ok(ids);
    }

    pub async fn id(&self) -> Result<String, String> {
        let a = self.end()?;
        let page = Page::from(&a).await?;
        return Ok(page.id());
    }
}
