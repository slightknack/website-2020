use web_sys::Response;
use crate::responder;
use crate::template;
use crate::hrdb::{location::Location, controller, shorthand::Shorthand};

pub async fn respond(short: &str) -> Result<Response, String>  {
    // look up the id-path
    let (ver_no, id, _) = Shorthand::read()
        .await?.unwrap().get(short)
        .ok_or("Shorthand does not map to Page")?
        .to_owned();

    // find and load the page location
    let master = Location::from_branch("master".to_owned());

    // try to find page on most recent version
    let versions = controller::versions(master).await?;
    let head = versions.last()
        .ok_or("No versions found")?
        .to_owned();
    let location = match controller::locate_id(head, id.clone()).await {
        Ok(l) => l,
        Err(_) => {
            let ver_no = versions.len() - 1;
            let specified = versions.iter().nth(ver_no)
                .ok_or("Specified Shorthand version does not exist")?
                .to_owned();
            controller::locate_id(specified, id.clone()).await?
        }
    };

    let parent = match location.back() {
        Ok(p)  => p.id().await?,
        Err(_) => location.id().await?,
    };

    // render the page
    let (title, content, _) = controller::read(&location).await?;
    let html = template::page::render(
        title,
        content,
        location.branch(),
        ver_no.to_string(),
        id,
        parent,
        vec![], // need to add chidren
    ).await?;

    return responder::html(&html, 200)
        .ok_or("Could not load Shorthand Page".to_owned())
}
