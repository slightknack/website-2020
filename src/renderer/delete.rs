use web_sys::Response;
use crate::route::Route;
use crate::responder;
use crate::renderer::edit::locate;
use crate::hrdb::controller;

pub async fn respond(path: Route) -> Result<Response, String> {
    let location = locate(path).await?;
    let parent   = location.back()?;
    controller::delete(location).await?;
    responder::redirect(
        &Route::over(vec![
            "perma".to_owned(),
            "head".to_owned(),
            parent.id().await?,
        ]).to_string()
    ).ok_or("Could not generate redirect to parent page.".to_owned())
}
