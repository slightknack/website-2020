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
extern crate time;

mod utils;
mod kv;
mod logger;
mod responder;
mod route;
mod hrdb;
mod template;
mod renderer;
mod auth;

use wasm_bindgen::prelude::*;
use js_sys::Promise;
use web_sys::{FetchEvent, Response, Request};
use url::Url;
use logger::log;
use route::Route;
use cookie::Cookie;

/// Takes an event, handles it, and returns a promise containing a response.
#[wasm_bindgen]
pub async fn main(event: FetchEvent) -> Promise {
    let request = event.request();
    let url = match Url::parse(&request.url()) {
        Ok(v) => v,
        Err(_) => return Promise::reject(&JsValue::from("Could not parse url")),
    };
    let path = Route::new(&url.path().to_lowercase());
    let method = request.method().to_lowercase();
    let authed = auth::validate(&request).await;
    let response = respond(request, path, method, authed).await;

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

pub async fn respond(request: Request, path: Route, method: String, authed: bool) -> Result<Response, String> {
    match path.iter().nth(0) {
        // root -> redirect to home
        None => responder::redirect("/home")
            .ok_or("Could not redirect".to_owned()),

        // static -> retrieve a static asset
        Some(s) if s == "static" => match method.as_ref() {
            "get" => renderer::static_::respond(path).await,
            u     => Err(format!("'{}' method not allowed on /static", u)),
        }

        // perma -> direct hrdb query '/branch/version_no/id'
        Some(p) if p == "perma" => match method.as_ref() {
            "get" => renderer::perma::respond(path).await,
            u     => Err(format!("'{}' method not allowed on /perma", u)),
        }

        // branches -> list all branches
        Some(b) if b == "branches" => match method.as_ref() {
            "get" => renderer::branches::respond(path).await,
            u     => Err(format!("'{}' method not allowed on /branches", u)),
        }

        // versions -> list all versions
        Some(v) if v == "versions" => match method.as_ref() {
            "get" => renderer::versions::respond(path).await,
            u     => Err(format!("'{}' method not allowed on /versions", u)),
        }

        // serve login prompt
        Some(a) if a == "auth" => match method.as_ref() {
            "get"  => renderer::auth::respond().await,
            "post" => renderer::auth::form(request).await,
            u     => Err(format!("'{}' method not allowed on /auth", u)),

        }

        // edit -> load the editor '/branch/id'
        Some(e) if e == "edit" => match method.as_ref() {
            "get"  => renderer::edit::respond(path).await,
            // "post" => renderer::edit::form(path, request).await,
            u     => Err(format!("'{}' method not allowed on /auth", u)),

        }


        // unimplemented

        // search -> search master for query
        Some(s) if s == "search" => renderer::search::respond(path).await,

        Some(c) if c == "create" => renderer::create::respond(path).await,
        Some(d) if d == "delete" => renderer::delete::respond(path).await,
        Some(r) if r == "relocate" => renderer::relocate::respond(path).await,

        // otherwise -> call out to create hrdb query
        Some(short) => renderer::shorthand::respond(short).await,
    }
}
