use std::usize;
use web_sys::Response;
use crate::responder;
use crate::template;
use crate::route::Route;
use crate::hrdb::{location::Location, controller};

pub async fn respond(path: Route) -> Result<Response, String> {
    let (b, vn, id) = (
        path.iter().nth(1).ok_or("No branch specified")?,
        path.iter().nth(2).ok_or("No version number specified")?,
        path.iter().nth(3).ok_or("No id specified")?,
    );

    let branch = Location::from_branch(b.to_owned());
    let versions = controller::versions(branch).await?;


    let version = if vn == "head" {
        // redirect to shorthand page
        versions.iter().last().ok_or("No versions exist on this branch yet")?
    } else {
        let ver_no = vn.parse::<usize>()
            .ok().ok_or("Version number was not a number")?;
        versions.iter()
            .nth(ver_no).ok_or("Version with that number does not exist")?
    };

    let location = if id == "root" {
        controller::root(version.clone())?
    } else {
        controller::locate_id(version.to_owned(), id.to_owned()).await?
    };

    let parent = match location.back() {
        Ok(p)  => p.id().await?,
        Err(_) => location.id().await?,
    };

    let (title, content, _) = controller::read(&location).await?;
    let html = template::page::render(
        title,
        content,
        b.to_owned(),
        vn.to_owned(),
        id.to_owned(),
        parent,
        vec![], // need to get children
    ).await?;
    responder::html(&html, 200)
        .ok_or("Could not generate response for location query".to_owned())
}
