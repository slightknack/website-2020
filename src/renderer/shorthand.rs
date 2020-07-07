use web_sys::Response;
use crate::responder;
use crate::template;
use crate::renderer::page;
use crate::hrdb::{location::Location, controller, shorthand::Shorthand};

pub async fn respond(short: &str) -> Result<Response, String>  {
    // look up the id-path
    let (ver_no, id, _) = Shorthand::read()
        .await?.unwrap().get(short)
        .ok_or("Shorthand does not map to Page")?
        .to_owned();

    // try to retrieve shorthand on master
    // if that fails, fall back to specified version.
    let master = Location::from_branch("master".to_string());
    let versions = controller::versions(master).await?;
    let version = versions
        .iter().nth(ver_no)
        .ok_or("Shorthand mapped to Page, but Page version is not valid")?
        .to_owned();
    let specified = controller::locate_id(version, id.clone()).await?;
    let location = page::remap(specified).await?;

    return page::render(location).await;
}
