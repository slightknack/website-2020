use web_sys::{Response, Request};
use crate::renderer::form;
use crate::responder;
use crate::template;
use crate::route::Route;
use crate::hrdb::{location::Location, controller};
use crate::logger::log;

pub async fn branch_id(path: Route) -> Result<(String, String), String> {
    return Ok((
        path.iter().nth(1).ok_or("No branch specified")?.to_owned(),
        path.iter().nth(2).ok_or("No id specified")?.to_owned(),
    ));
}

pub async fn locate(path: Route) -> Result<Location, String> {
    let (b, id) = branch_id(path).await?;

    let branch = Location::from_branch(b.to_owned());
    let head = controller::head(branch).await?;
    let location = controller::locate_id(head.to_owned(), id.to_owned()).await?;

    return Ok(location);
}

pub async fn respond(path: Route) -> Result<Response, String> {
    let location = locate(path).await?;
    let (title, content, _) = controller::read(&location).await?;

    let html = template::edit::render(title, content, location.branch(), location.id().await?).await?;
    responder::html(&html, 200)
        .ok_or("Could not load the editor".to_owned())
}

pub async fn form(request: Request, path: Route) -> Result<Response, String> {
    let form = form::parse(request).await?;
    let title = form.get("title").as_string()
        .ok_or("Could not retrieve new title from request")?;
    let edited = form.get("editor").as_string()
        .ok_or("Could not retrieve edited Page from request")?;

    // get the hrdb location of the page
    let location = locate(path).await?;

    // update page with new information
    controller::edit(location.clone(), Some(title), Some(edited), None).await?;

    // redirect to head
    let head = Route::over(vec![
        "perma".to_string(),
        location.branch(),
        "head".to_string(),
        location.id().await?,
    ]);

    responder::redirect(&head.to_string())
        .ok_or("Could not generate redirect to new version".to_owned())
}
