use ramhorns::{Template, Content};
use crate::template::base::{Child, Children, Action, Actions, Base, asset};
use crate::route::Route;

#[derive(Content)]
struct Edit {
    title:  String,
    old:    String,
    branch: String,
    id:     String,
}

pub async fn render(
    title: String,
    old:   String,
    branch: String,
    id: String,
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let edit = Template::new(asset("edit.html").await?)
        .ok().ok_or("Could not create edit template")?;

    let actions = Actions {
        items: vec![
            ("arrow_back", Route::over(vec!["perma".to_string(), branch.clone(), "head".to_string(), id.clone()]), "Back"),
            ("add", Route::over(vec!["create".to_string(), branch.clone(), id.clone()]), "Create a new page"),
            ("delete", Route::over(vec!["delete".to_string(), branch.clone(), id.clone()]), "Delete this page"),
        ].into_iter()
            .map(
            |action| {
                let (icon, route, value) = action;
                Action {
                    icon:  icon.to_owned(),
                    link:  route.to_string(),
                    value: value.to_string(),
                }
            }
            )
            .collect::<Vec<Action>>(),
    };

    // flesh them out
    let edit_data = Edit { title: title.clone(), old, branch, id };
    let edit_rendered = edit.render(&edit_data);
    let base_data = Base {
        title: "Editing — ".to_owned() + &title,
        content: edit_rendered,
        children: None,
        actions:  Some(actions),
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
