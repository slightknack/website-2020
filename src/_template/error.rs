#[derive(Content)]
struct Error {
    message: String,
}

pub async fn error(message: String) -> Result<String, String> {
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let error = Template::new(asset("error.html").await?)
        .ok().ok_or("Could not create table template")?;

    let error_data = Error { message };
    let error_rendered = error.render(&error_data);
    let base_data = Base {
        title: "Error".to_owned(),
        content: error_rendered,
        children: List { items: vec![] },
        actions:  List { items: vec![] },
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
