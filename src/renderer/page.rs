use web_sys::Response;
use crate::route::Route;
use crate::hrdb::location::Location;
use crate::hrdb::controller;
use crate::hrdb::shorthand::Shorthand;
use crate::{responder, template};

pub async fn shortify(location: Location) -> Result<Option<Response>, String> {
    // if latest version and on master, remap to shorthand
    let master = controller::head(
        Location::from_branch("master".to_owned())
    ).await?;

    if location.branch()  != master.branch()
    || location.version() != master.version() {
        return Ok(None);
    }

    let mut best: (usize, Option<Response>) = (0, None);
    let id = location.id().await?;
    for (key, value) in Shorthand::read().await?.unwrap().iter() {
        if value.1 == id && value.0 > best.0 {
            let redirect = responder::redirect(&Route::over(vec![key.to_owned()]).to_string())
                .ok_or("Could not generate redirect to simplified URL")?;
            best = (value.0, Some(redirect));
        }
    };

    return match best {
        (_, Some(r)) => Ok(Some(r)),
        (_, None) => Err("Could not find any valid Shorthand to reduce to".to_owned()),
    };
}

pub async fn remap(location: Location) -> Result<Location, String> {
    // find and load the page location
    // try to find page on most recent version
    let master = Location::from_branch("master".to_owned());
    let head = controller::head(master).await?;

    match controller::locate_id(head, location.id().await?.clone()).await {
        Ok(l) => Ok(l),
        Err(_) => Ok(location),
    }
}

pub async fn render(location: Location) -> Result<Response, String> {
    let parent = match location.back() {
        Ok(p)  => p.id().await?,
        Err(_) => location.id().await?,
    };

    let mut children = vec![];
    for child in controller::children(location.clone()).await? {
        children.push((controller::title(&child).await?, child.id().await?));
    };

    // redirect to head if on most recent version
    // redirect to root if at page root.
    let is_head = controller::head(location.clone()).await?.version()? == location.version()?;
    let is_root = controller::root(location.clone())?.path()?          == location.path()?;

    let (title, content, _) = controller::read(&location).await?;
    let html = template::page::render(
        title,
        content,
        location.branch(),
        location.ver_no().await?, // have them pass value in?
        location.id().await?,
        is_head,
        is_root,
        parent,
        children,
    ).await?;

    responder::html(&html, 200)
        .ok_or("Could not generate response for location query".to_owned())
}
