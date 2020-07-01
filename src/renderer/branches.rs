use web_sys::Response;
use crate::route::Route;
use crate::responder;
use crate::template;
use crate::hrdb::controller;

pub async fn respond(path: Route) -> Result<Response, String> {
    let branches = controller::branches().await?;
    let names = branches.iter()
        .map(|l| vec![l.branch()])
        .collect::<Vec<Vec<String>>>();

    let html = template::table::render(
        "Branches".to_owned(),
        vec!["Name".to_owned()],
        names,
    ).await?;
    responder::html(&html, 200)
        .ok_or("Could not generate response listing branches".to_owned())
}
