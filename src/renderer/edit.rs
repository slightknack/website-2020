use web_sys::Response;
use crate::responder;
use crate::template;
use crate::route::Route;
use crate::hrdb::{location::Location, controller};

pub async fn respond(path: Route) -> Result<Response, String> {
    let (b, id) = (
        path.iter().nth(1).ok_or("No branch specified")?,
        path.iter().nth(2).ok_or("No id specified")?,
    );

    let branch = Location::from_branch(b.to_owned());
    let versions = controller::versions(branch).await?;
    let head = versions.last().ok_or("No versions exist on this branch")?;
    let location = controller::locate_id(head.to_owned(), id.to_owned()).await?;
    let (title, content, _) = controller::read(&location).await?;

    let html = template::edit::render(title, content).await?;
    responder::html(&html, 200)
        .ok_or("Could not load the editor".to_owned())
}
