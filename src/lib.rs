extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate js_sys;
extern crate web_sys;
extern crate cookie;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate url;
extern crate getrandom;
extern crate ramhorns;

mod utils;
mod kv;
mod logger;
mod responder;
mod route;
mod hrdb;
mod template;
mod renderer;

use std::usize;
use hrdb::{location::Location, controller, shorthand::Shorthand};
use wasm_bindgen::prelude::*;
use js_sys::Promise;
use web_sys::{FetchEvent, Response};
use url::Url;
use logger::log;
use route::Route;

// todo: break this up

/// Takes an event, handles it, and returns a promise containing a response.
#[wasm_bindgen]
pub async fn main(event: FetchEvent) -> Promise {
    // let r = controller::init().await;

    let request = event.request();
    let url = match Url::parse(&request.url()) {
        Ok(v) => v,
        Err(_) => return Promise::reject(&JsValue::from("Could not parse url")),
    };
    let path = Route::new(&url.path().to_lowercase());
    let response = respond(path, false).await;

    // if the response failed, we return an internal server error
    return match response {
        Ok(response) => Promise::resolve(&JsValue::from(response)),
        Err(e) => {
            let html = template::error::render(e).await.unwrap();
            let r = responder::html(&html, 404).unwrap();
            Promise::resolve(&JsValue::from(r))
        },
    };
}

pub async fn respond(path: Route, authed: bool) -> Result<Response, String> {
    match path.iter().nth(0) {
        // root -> redirect to home
        None => responder::redirect("/home")
            .ok_or("Could not redirect".to_owned()),

        // static -> retrieve a static asset
        Some(s) if s == "static" => renderer::static_::respond(path).await,

        // edit -> load the editor '/branch/id'
        Some(e) if e == "edit" => renderer::edit::respond(path).await,

        // perma -> direct hrdb query '/branch/version_no/id'
        Some(p) if p == "perma" => renderer::perma::respond(path).await,

        // branches -> list all branches
        Some(b) if b == "branches" => renderer::branches::respond(path).await,

        // versions -> list all versions
        Some(v) if v == "versions" => renderer::versions::respond(path).await,

        // unimplemented

        // search -> search master for query
        Some(s) if s == "search" => renderer::search::respond(path).await,

        Some(c) if c == "create" => renderer::create::respond(path).await,
        Some(d) if d == "delete" => renderer::delete::respond(path).await,
        Some(r) if r == "relocate" => renderer::relocate::respond(path).await,
        Some(a) if a == "auth" => renderer::auth::respond(path).await,

        // otherwise -> call out to create hrdb query
        Some(short) => renderer::shorthand::respond(short).await,
    }
}
