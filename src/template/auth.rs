use ramhorns::{Template, Content};
use crate::template::base::{Base, asset};

#[derive(Content)]
struct Auth();

pub async fn render() -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let auth = Template::new(asset("auth.html").await?)
        .ok().ok_or("Could not create auth template")?;

    // flesh them out
    let auth_data = Auth();
    let auth_rendered = auth.render(&auth_data);
    let base_data = Base {
        title: "Authenticating".to_owned(),
        content: auth_rendered,
        children: None,
        actions: None,
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
