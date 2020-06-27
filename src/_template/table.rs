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
