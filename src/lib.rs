extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate js_sys;
extern crate web_sys;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate url;
extern crate getrandom;

mod utils;
mod kv;
mod logger;
mod responder;
mod route;
mod hrdb;

use hrdb::{Location, HRDB};
use wasm_bindgen::prelude::*;
use web_sys::FetchEvent;
use logger::log;

// main should act as the interface between rust and js
// i.e. no other modules shouldn't have to care about js_sys, etc.
// turbolinks
// codemirrior

/// Takes an event, handles it, and returns a promise containing a response.
#[wasm_bindgen]
pub async fn main(event: FetchEvent) -> JsValue {
    HRDB::init().await.unwrap();


    let branches = HRDB::branches().await.unwrap();
    let branch = branches.last().unwrap().to_owned();
    let versions = HRDB::versions(branch).await.unwrap();
    let version = versions.last().unwrap().to_owned();

    let root = HRDB::root(version).await.unwrap();

    let (title, content, fields) = &HRDB::read(&root).await.unwrap();

    let response = JsValue::from(&responder::html("Hello, world", 200).unwrap());

    return JsValue::from("okok");
    // return Promise::resolve(&response);
}
