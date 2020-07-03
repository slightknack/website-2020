use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, Request};
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};
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

async fn check(password: String) -> Result<bool, String> {
    let params = ScryptParams::new(14, 8, 1)
        .ok().ok_or("Could not create hash params")?;
    let hash = kv::value(kv::AuthNS::get("password", "text")).await
        .ok_or("Could not retrieve stored password hash")?
        .as_string()
        .ok_or("Could not unwrap the hash")?;

    return match scrypt_check(&password, &hash).ok() {
        Some(_) => Ok(true),
        None    => Ok(false),
    }
}
