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
    title:   String,
    content: String,
    branch:  String,
    vn:      usize,
    iden:    String,
    is_head: bool,
    is_root: bool,
    parent:  String,
    child_pair: Vec<(String, String)>, // (title, id)
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let page = Template::new(asset("page.html").await?)
        .ok().ok_or("Could not create page template")?;


    let ver_no = if is_head { "head".to_owned() } else { vn.to_string()  };
    let id     = if is_root { "root".to_owned() } else { iden.to_owned() };

    let mut items = vec![];
    if !is_root {
        items.push((
            "arrow_back",
            Route::over(vec!["perma".to_string(), branch.clone(), ver_no.clone(), parent]),
            "Back"
        ))
    }
    items.append(&mut vec![
        ("push_pin", Route::over(vec!["perma".to_string(), branch.clone(), vn.to_string(), id.clone()]), "Permalink this Version"),
        ("edit", Route::over(vec!["edit".to_string(), branch.clone(), iden.clone()]), "Edit this Page"),
    ]);
    if !is_head {
        items.push((
            "update",
            Route::over(vec!["perma".to_string(), branch.clone(), "head".to_string(), id.clone()]),
            "Jump to Present"
        ))
    }

    let actions = Actions {
        items: items.into_iter()
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
                    Child { branch: branch.clone(), id, ver_no: ver_no.clone(), value: title }
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
        children: if children.items.is_empty() { None } else { Some(children) },
        actions:  if actions.items.is_empty()  { None } else { Some(actions)  },
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
