use ramhorns::{Template, Content};
use crate::template::base::{Child, Children, Action, Actions, Base, asset};
use crate::route::Route;

#[derive(Content)]
struct Page {
    title:   String,
    #[md]
    content: String,
}

pub async fn render(
    title:    String,
    content:  String,
    branch:   String,
    ver_no:   usize,
    id:       String,
    parent:   String,
    child_pair: Vec<(String, String)>,
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let page = Template::new(asset("page.html").await?)
        .ok().ok_or("Could not create page template")?;

    let actions = Actions {
        items: vec![
            ("arrow_back", Route::over(vec!["perma".to_string(), branch.clone(), ver_no.to_string(), parent]), "Back"),
            ("push_pin", Route::over(vec!["perma".to_string(), branch.clone(), ver_no.to_string(), id.clone()]), "Permalink for this Version"),
            ("update", Route::over(vec!["perma".to_string(), branch.clone(), "head".to_string(), id.clone()]), "Jump to Present"),
            ("edit", Route::over(vec!["edit".to_string(), branch.clone(), id.clone()]), "Edit this Page"),
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

    let children = Children {
        items: child_pair.into_iter()
            .map(
                |pair| {
                    let (title, id) = pair;
                    Child { id, value: title }
                }
            )
            .collect::<Vec<Child>>(),
    };

    // flesh them out
    let page_data = Page { title: title.clone(), content };
    let page_rendered = page.render(&page_data);
    let base_data = Base {
        title,
        content: page_rendered,
        children: None, // Some(children),
        actions: Some(actions)
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
