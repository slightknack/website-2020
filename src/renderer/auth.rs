use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, Request, FormData};
use serde::{Serialize, Deserialize};
use serde_json;
use js_sys::Object;

use crate::responder;
use crate::template;
use crate::logger::log;
use crate::auth;

pub async fn respond() -> Result<Response, String> {
    let html = template::auth::render().await?;
    responder::html(&html, 200)
        .ok_or("Could not generate authentication Page response".to_owned())
}

pub async fn parse_form(request: Request) -> Result<FormData, String> {
    let promise = request.form_data()
        .ok().ok_or("Could not get form data from request")?;
    let js_value = JsFuture::from(promise).await
        .ok().ok_or("Could not resolve FormData Promise")?;
    return Ok(FormData::from(js_value));
}

pub async fn form(request: Request) -> Result<Response, String> {
    let redirect = match request.headers().get("referer") {
        Ok(Some(v)) => v,
        _ => request.url(),
    };

    let form = parse_form(request).await?;
    let password = form.get("password").as_string()
        .ok_or("Could not retrieve password from request")?;

    return if auth::check(password).await? == false {
        responder::redirect(&redirect)
            .ok_or("Password was incorrect and could not generate redirect".to_owned())
    } else {
        let session = auth::session().await?;
        responder::cookie(session, "/home")
            .ok_or("Could not generate response for authentication".to_owned())
    }
}
