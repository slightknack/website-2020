use web_sys::Response;
use crate::route::Route;

pub async fn respond(path: Route) -> Result<Response, String> {
    // calculate filters for query
    // load shorthand
    // iterate through all pages
    // try against all pages, giving a point for each filter that matches
    // sort by points
    // return all pages with score > 1 in score order
    Err("Searching is not yet implemented".to_owned())
}
