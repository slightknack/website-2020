use web_sys::Response;
use crate::route::Route;
use crate::responder;
use crate::template;

pub async fn respond() -> Result<Response, String> {
    let html = template::auth::render().await?;
    responder::html(&html, 200)
        .ok_or("Could not generate authentication Page response".to_owned())
}
