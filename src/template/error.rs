use ramhorns::{Template, Content};
use crate::template::base::{Children, Actions, Base, asset};

#[derive(Content)]
struct Error {
    message: String,
}

pub async fn render(message: String) -> Result<String, String> {
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let error = Template::new(asset("error.html").await?)
        .ok().ok_or("Could not create table template")?;

    let error_data = Error { message };
    let error_rendered = error.render(&error_data);
    let base_data = Base {
        title: "Error".to_owned(),
        content: error_rendered,
        children: None,
        actions:  None,
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
