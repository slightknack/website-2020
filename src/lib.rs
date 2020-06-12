extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate js_sys;
extern crate web_sys;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate crypto;

mod utils;
mod kv;
mod logger;
mod responder;
mod route;
mod hrdb;

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
    let request = event.request();

    // JsFuture::from(kv::StaticNS::put("working", "Heck <strong>yeah</strong>")).await.unwrap();
    // let working = kv::StaticNS::get("working", "text");
    // let content = format!(
    //     "<h1>Currently a work in progress...</h1>\n<p>Does ( ͡° ͜ʖ ͡°) think the database is working? {}!<p>",
    //     JsFuture::from(working).await.unwrap().as_string().unwrap().trim(),
    // );

    return Promise::resolve(&JsValue::from(&responder::html("Hello, world", 200).unwrap()));
}
