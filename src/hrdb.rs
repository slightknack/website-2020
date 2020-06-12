use std::time;
use std::mem;
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crate::kv;

// helper functions
// crypto

pub fn hash(string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(string);
    let hex = hasher.result_str();
    return hex;
}

pub fn stamp() -> String {
    hash(
        &time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string()
    )
}

// key-value

pub async fn write(value: &str) -> Result<String, String> {
    let key = hash(value);
    kv::value(kv::AddressedNS::put(&key, value))
        .await.ok_or("Could not write to kv")?;
    return Ok(key);
}

pub async fn read(key: &str) -> Result<String, String> {
    kv::value(kv::AddressedNS::get(key, "text"))
        .await.ok_or("Could not read from kv")?
        .as_string().ok_or("Could not convert to String".to_owned())
}

pub async fn append(key: &str, value: &str) -> Result<(), String> {
    let contents = read(key).await? + "\n" + value;
    kv::value(kv::AddressedNS::put(&key, &contents))
        .await.ok_or("Could not append to kv")?;
    return Ok(());
}

pub async fn list(key: &str) -> Result<Vec<String>, String> {
    Ok(
        read(key)
            .await?
            .split("\n")
            .filter(|x| x != &"")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
    )
}

pub async fn ensure(key: &str) -> Result<(), String> {
    if let Ok(_) = read(key).await {
        return Ok(());
    }
    kv::value(kv::AddressedNS::put(&key, ""))
        .await.ok_or("Could not create new empty list")?;
    return Ok(());
}

// structs

pub struct HRDB; // Branch

impl HRDB {
    pub async fn init() -> Result<(), String> {
        let name = "hrdb";

        if let Ok(_) = read(name).await {
            return Ok(())
        }

        let content = Content::new("My friend ( ͡° ͜ʖ ͡°) would like you to check back soon.".to_owned());
        let address = write(&content.to_string()).await?;

        let root = Page::new(
            "Currently a Work in Progress...".to_owned(),
            address,
            HashMap::new(),
        );
        let version = write(&root.to_string()?).await?;

        ensure("master").await?;
        append("master", &version);

        ensure("hrdb").await?;
        append("hrdb", "master").await?;

        return Ok(());
    }

    // pub async fn fork(location, location)
    // pub async fn merge(location, location)

    /// Updates page at location, iterating backwards through path chain.
    async fn commit(location: &Location, updated: &Page) -> Result<(), String> {
        // commits can only be applied to the head version of the branch
        // check that this commit is being applied to the head
        let head = Branch::from(&location.branch()).await?.head()?;
        let version = location.version()?;

        if version != head {
            return Err("Can only create Page on latest version of Branch".to_owned());
        }

        // get the new address of the page that has been updated
        let mut address = write(&updated.to_string()?).await?;

        // next, build an iterator going to the version root through the page tree
        let path = location.path()?;
        let mut addresses = path.iter().rev();
        let mut child = addresses.next()
            .ok_or("Can not commit to a path without a Page")?;

        // work back through the path in parent child pairs
        // replacing the parent's reference to the child with the updated child's address
        for parent in addresses {
            let mut page = Page::from(parent).await?;
            let index = page.children.iter().position(|x| x == child)
                .ok_or("Path is broken, parent Page does not have specified child")?;
            mem::replace(&mut page.children[index], address);
            address = write(&page.to_string()?).await?;
            child = parent;
        }

        // push the updated root version onto the branch tree
        append(&location.branch(), &address).await?;
        return Ok(());
    }

    /// Creates a new page on the head of a branch,
    /// returning the new page if successful
    pub async fn create(
        location: &Location,
        title: String,
        content: String,
        fields: HashMap<String, String>
    ) -> Result<(), String> {
        let new = Page::new(title, content, fields);
        let address = write(&new.to_string()?).await?;
        let mut parent = Page::from(
            location.path()?.last()
                .ok_or("Can not create Page beside root")?
        ).await?;
        parent.children.push(address);

        HRDB::commit(&location.back()?, &parent).await?;
        return Ok(());
    }

    // pub async fn edit(location, ...)
    // pub async fn move(location, ...)
    // pub async fn duplicate(location)
    // pub async fn delete(location)
}

struct Branch {
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
            .ok_or("Could not find version to fork at")?;

        return Ok(Branch { versions: self.versions[..=position].to_vec() });
    }

    pub fn merge(&mut self, from: &Branch) -> Result<String, String> {
        // find last common ancestor, favoring self
        // enumerate all page ids
        // ...
        todo!()
    }

    pub fn head(&self) -> Result<String, String> {
        let h = self.versions.last().ok_or("No versions on Branch")?;
        return Ok(h.to_string());
    }
}

#[derive(Serialize, Deserialize)]
struct Page {
    id:           String,
    pub fields:   HashMap<String, String>,
    pub title:    String,
    pub content:  String, // Content
    pub children: Vec<String>, // page checksum
}

impl Page {
    pub async fn from(hash: &str) -> Result<Page, String> {
        serde_json::from_str(&read(&hash).await?)
            .ok().ok_or("Could not deserialize Page".to_owned())
    }

    pub fn new(title: String, content: String, fields: HashMap<String, String>) -> Page {
        Page {
            id: stamp(),
            fields,
            title,
            content,
            children: vec![],
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
}

struct Content(pub String);

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

/// The location of a specific page at a specific version on a specific branch.
/// branch is the name of the branch,
/// version is the hash of the root,
/// path is a list of checksums.
#[derive(Clone)]
pub struct Location((String, Option<(String, Option<Vec<String>>)>));

impl Location {
    pub fn from_branch(branch: String) -> Location {
        Location((branch, None))
    }

    pub fn from_branch_and_version(branch: String, version: String) -> Location {
        Location((branch, Some((version, None))))
    }

    pub fn from_branch_version_and_path(branch: String, version: String, path: Vec<String>) -> Location {
        Location((branch, Some((version, Some(path)))))
    }

    pub fn branch(&self) -> String {
        (self.0).0.to_string()
    }

    pub fn version(&self) -> Result<String, String> {
        Ok(((self.0).1).clone().ok_or("Location does not specify version")?.0)
    }

    pub fn path(&self) -> Result<Vec<String>, String> {
        Ok(
            ((self.0).1).clone()
            .ok_or("Location does not specify version")?.1.clone()
            .ok_or("Location does not specify path")?
        )
    }

    pub fn back(&self) -> Result<Location, String> {
        let mut path = self.path()?;
        path.pop().ok_or("Can not go back past root");

        Ok(
            Location::from_branch_version_and_path(
                self.branch().clone(),
                self.version()?.clone(),
                path,
            )
        )
    }
}
