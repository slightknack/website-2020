use web_sys::Response;
use crate::route::Route;

pub async fn respond(path: Route) -> Result<Response, String> {
    Err("Listing versions is not yet implemented".to_owned())
}
