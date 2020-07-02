use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, Request};
use crate::responder;
use crate::template;
use crate::logger::log;

pub async fn respond() -> Result<Response, String> {
    let html = template::auth::render().await?;
    responder::html(&html, 200)
        .ok_or("Could not generate authentication Page response".to_owned())
}

pub async fn form(request: Request) -> Result<Response, String> {
    log("Tried to submit password");
    // get redirect url
    // let headers = request.headers();
    // let redirect = match headers.referrer() {
    //     Ok(Some(v)) => v,
    //     _ => request.url(),
    // };

    // get form data
    let form_data_promise = request.form_data()
        .ok().ok_or("Could not extract form data")?;
    let form_data = JsFuture::from(form_data_promise).await
        .ok().ok_or("Could not retrieve form data")?;

    // get password hash
    // compare hashes
    // set cookie if true
    // redirect

    Err("Full authentication not yet implemented.".to_owned())
}
