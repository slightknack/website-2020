use ramhorns::{Template, Content};
use std::collections::HashMap;
use crate::kv;

#[derive(Content)]
struct PageFiller {
    title: String,
    #[md]
    content: String,
    branch: String,
    ver_no: usize,
    id: String,
}

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

pub async fn page(
    title: String,
    content: String,
    branch: String,
    ver_no: usize,
    id: String,
) -> Result<String, String> {
    // get the templates
    let base = Template::new(asset("base.html").await?)
        .ok().ok_or("Could not create base template")?;
    let page = Template::new(asset("page.html").await?)
        .ok().ok_or("Could not create page template")?;

    // flesh them out
    let page_filler = PageFiller { title: title.clone(), content, branch, ver_no, id };
    let page_rendered = page.render(&page_filler);
    let base_filler = BaseFiller { title: title + "—Isaac Clayton", content: page_rendered };
    let base_rendered = base.render(&base_filler);
    return Ok(base_rendered);
}
