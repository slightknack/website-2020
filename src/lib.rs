extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate js_sys;
extern crate web_sys;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate crypto;
extern crate url;

mod utils;
mod kv;
mod logger;
mod responder;
mod route;
mod hrdb;

use url::Url;
use route::Route;
use logger::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use web_sys::FetchEvent;

// main should act as the interface between rust and js
// i.e. no other modules shouldn't have to care about js_sys, etc.
// turbolinks
// codemirrior

/// Takes an event, handles it, and returns a promise containing a response.
#[wasm_bindgen]
pub async fn main(event: FetchEvent) -> Promise {
    hrdb::HRDB::init();

    // let request = event.request();
    // let url = match Url::parse(&request.url()) {
    //     Ok(v)  => v,
    //     Err(e) => return Promise::reject(&e),
    // };
    // let route = Route::new(url.path().to_lowercase());
    // let method = Method::new(request.method().to_lowercase());

    // match (route.iter().next(), method) {
    //     (Some("static"), Method::Get) => _, // static content, i.e. html, css, etc.
    //     (Some("authenticate"), Method::Post) => _, // cookie management
    //     (Some(_), Method::Get) => _, // hrdb
    //     (Some(_), _) => _, // not allowed
    //     (None, _) => _, // 404
    // }

    return Promise::resolve(&JsValue::from(&responder::html("Hello, world", 200).unwrap()));
}
