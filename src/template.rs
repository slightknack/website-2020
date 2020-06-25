use ramhorns::{Template, Content};
use std::collections::HashMap;
use crate::logger::log;
use crate::kv;

#[derive(Content)]
struct BaseFiller {
    title: String,
    content: String,
}

pub async fn asset(name: &str) -> Result<String, String> {
    Ok(
        kv::value(kv::StaticNS::get(name, "text"))
            .await.ok_or("Could not load asset")?
            .as_string().ok_or("Could not convert asset to string")?
    )
}

#[derive(Content)]
struct PageFiller {
    title: String,
    #[md]
    content: String,
    branch: String,
    ver_no: usize,
    id: String,
    parent: String,
}

pub async fn page(
    title: String,
    content: String,
    branch: String,
    ver_no: usize,
    id: String,
    parent: String,
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let page = Template::new(asset("page.html").await?)
        .ok().ok_or("Could not create page template")?;

    // flesh them out
    let page_filler = PageFiller { title: title.clone(), content, branch, ver_no, id, parent };
    let page_rendered = page.render(&page_filler);
    let base_filler = BaseFiller { title: title + " — Isaac Clayton", content: page_rendered };
    let base_rendered = base.render(&base_filler);
    return Ok(base_rendered);
}

#[derive(Content)]
struct EditFiller {
    title: String,
    old: String,
}

pub async fn edit(
    title: String,
    old: String,
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let edit = Template::new(asset("edit.html").await?)
        .ok().ok_or("Could not create edit template")?;

    // flesh them out
    let edit_filler = EditFiller { title: title.clone(), old };
    let edit_rendered = edit.render(&edit_filler);
    let base_filler = BaseFiller {
        title: "Editing — ".to_owned() + &title + " — Isaac Clayton",
        content: edit_rendered
    };
    let base_rendered = base.render(&base_filler);
    return Ok(base_rendered);
}

#[derive(Debug, Content)]
struct TableFiller {
    title: String,
    columns: Vec<Item>,
    rows: Vec<RowFiller>,
}

#[derive(Debug, Content)]
struct RowFiller {
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
        .map(|row| RowFiller {
            items: row.into_iter()
                .map(|item| Item { item })
                .collect::<Vec<Item>>()
            }
        )
        .collect::<Vec<RowFiller>>();


    // flesh them out
    let table_filler = TableFiller { title: title.clone(), columns, rows };
    log(&format!("table {:?}", table_filler));
    let table_rendered = table.render(&table_filler);
    let base_filler = BaseFiller {
        title: "Listing ".to_owned() + &title + " — Isaac Clayton",
        content: table_rendered,
    };
    let base_rendered = base.render(&base_filler);
    return Ok(base_rendered);
}

#[derive(Content)]
struct ErrorFiller {
    message: String,
}

pub async fn error(message: String) -> Result<String, String> {
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let error = Template::new(asset("error.html").await?)
        .ok().ok_or("Could not create table template")?;

    let error_filler = ErrorFiller { message };
    let error_rendered = error.render(&error_filler);
    let base_filler = BaseFiller {
        title: "Error".to_owned() + " — Isaac Clayton",
        content: error_rendered,
    };
    let base_rendered = base.render(&base_filler);
    return Ok(base_rendered);

    todo!();
}
