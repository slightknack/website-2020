struct List<T> where T: Content {
    items: Vec<T>,
}

impl<T> Content for List<T> where T: Content {}

#[derive(Content)]
struct Child {
    id:    String,
    value: String,
}

#[derive(Content)]
struct Action {
    icon:  String,
    link:  String,
    value: String,
}

#[derive(Content)]
struct Base {
    title:    String,
    content:  String,
    children: List<Child>,
    actions:  List<Action>,
}

pub async fn asset(name: &str) -> Result<String, String> {
    Ok(
        kv::value(kv::StaticNS::get(name, "text"))
            .await.ok_or("Could not load asset")?
            .as_string().ok_or("Could not convert asset to string")?
    )
}
