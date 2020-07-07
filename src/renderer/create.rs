use std::collections::HashMap;
use web_sys::Response;
use crate::responder;
use crate::route::Route;
use crate::hrdb::controller;
use crate::renderer::edit::locate;

pub async fn respond(path: Route) -> Result<Response, String> {
    let parent = locate(path).await?;
    let child = controller::create(
        parent,
        "A New Mysterious Untitled Page".to_owned(),
        "".to_owned(),
        HashMap::new()
    ).await?;

    responder::redirect(&format!("/edit/{}/{}", child.branch(), child.id().await?))
        .ok_or("Could not generate redirect to newly created Page".to_owned())
}
