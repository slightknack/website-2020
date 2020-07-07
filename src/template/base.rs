use ramhorns::Content;
use crate::kv;

#[derive(Content)]
pub struct Children {
    pub items: Vec<Child>,
}

#[derive(Content)]
pub struct Actions {
    pub items: Vec<Action>,
}

#[derive(Content)]
pub struct Child {
    pub id:     String,
    pub branch: String,
    pub ver_no: String,
    pub value:  String,
}

#[derive(Content)]
pub struct Action {
    pub icon:  String,
    pub link:  String,
    pub value: String,
}

#[derive(Content)]
pub struct Base {
    pub title:    String,
    pub content:  String,
    pub children: Option<Children>,
    pub actions:  Option<Actions>,
}

pub async fn asset(name: &str) -> Result<String, String> {
    Ok(
        kv::value(kv::StaticNS::get(name, "text"))
            .await.ok_or("Could not load asset")?
            .as_string().ok_or("Could not convert asset to string")?
    )
}
