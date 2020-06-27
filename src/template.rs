use ramhorns::{Template, Content};
use crate::logger::log;
use crate::route::Route;
use crate::kv;

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

#[derive(Content)]
struct Page {
    title:   String,
    #[md]
    content: String,
}

pub async fn page(
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

    let actions = List {
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

    let children = List {
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
    let base_data = Base { title, content: page_rendered, children, actions };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}

#[derive(Content)]
struct Edit {
    title: String,
    old:   String,
}

pub async fn edit(
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
        children: List { items: vec![] },
        actions:  List { items: vec![] },
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}

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

#[derive(Debug, Content)]
struct Table {
    title: String,
    columns: Vec<Item>,
    rows: Vec<Row>,
}

#[derive(Debug, Content)]
struct Row {
    items: Vec<Item>,
}

#[derive(Debug, Content)]
struct Item {
    item: String,
}

pub async fn table(
    title: String,
    c: Vec<String>,
    r: Vec<Vec<String>>,
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let table = Template::new(asset("table.html").await?)
        .ok().ok_or("Could not create table template")?;

    let columns = c.into_iter()
        .map(|item| Item { item })
        .collect::<Vec<Item>>();
    let rows    = r.into_iter()
        .map(|row| Row {
            items: row.into_iter()
                .map(|item| Item { item })
                .collect::<Vec<Item>>()
            }
        )
        .collect::<Vec<Row>>();

    // flesh them out
    let table_data = Table { title: title.clone(), columns, rows };
    log(&format!("table {:?}", table_data));
    let table_rendered = table.render(&table_data);
    let base_data = Base {
        title: "Listing ".to_owned() + &title,
        content: table_rendered,
        children: List { items: vec![] },
        actions:  List { items: vec![] },
    };
    let base_rendered = base.render(&base_data);
    return Ok(base_rendered);
}
