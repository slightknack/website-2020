use std::usize;
use web_sys::Response;
use crate::responder;
use crate::template;
use crate::route::Route;
use crate::renderer::page;
use crate::hrdb::{location::Location, controller};
use crate::logger::log;

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

    return match page::shortify(location.clone()).await? {
        Some(r) if vn == "head" => Ok(r),
        _ => page::render(location).await,
    };
}
