use ramhorns::{Template, Content};
use crate::template::base::{Children, Actions, Base, asset};

#[derive(Content)]
struct Edit {
    title: String,
    old:   String,
}

pub async fn render(
    title: String,
    old:   String,
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let edit = Template::new(asset("edit.html").await?)
        .ok().ok_or("Could not create edit template")?;

    // flesh them out
    let edit_data = Edit { title: title.clone(), old };
    let edit_rendered = edit.render(&edit_data);
    let base_data = Base {
        title: "Editing — ".to_owned() + &title,
        content: edit_rendered,
        children: None,
        actions:  None,
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
