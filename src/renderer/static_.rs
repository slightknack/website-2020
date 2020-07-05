use web_sys::Response;
use crate::responder;
use crate::template;
use crate::logger::log;
use crate::route::Route;

pub async fn respond(path: Route) -> Result<Response, String> {
    let file = path.iter().nth(1).ok_or("Static request specified no asset")?;
    let split = file
        .split(".")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    let (_, kind) = (
        split.iter().nth(0).ok_or("Asset is not a file")?,
        split.iter().nth(1).ok_or("Asset is a file without an extension")?,
    );

    // TODO: fix annoying cloudflare css bug
    // log(&format!("getting {:?}", file));
    let content = template::base::asset(file).await?;
    // log(&format!("{:?} content", file));

    let response = match kind {
        h if h == "html" => responder::html(&content, 200),
        c if c == "css"  => responder::css(&content, 200),
        _                => responder::plain(&content, 200),
    }.ok_or("Could not generate response for static asset")?;
    Ok(response)
}
