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
extern crate ramhorns;

mod utils;
mod kv;
mod logger;
mod responder;
mod route;
mod hrdb;
mod template;

use hrdb::{Location, HRDB, Shorthand};
use wasm_bindgen::prelude::*;
use js_sys::Promise;
use web_sys::{FetchEvent, Response};
use url::Url;
use logger::log;
use route::Route;

// main should act as the interface between rust and js
// i.e. no other modules shouldn't have to care about js_sys, etc.
// turbolinks
// codemirrior

/// Takes an event, handles it, and returns a promise containing a response.
#[wasm_bindgen]
pub async fn main(event: FetchEvent) -> Promise {
    let r = HRDB::init().await;
    let mut location = Location::from_branch("master".to_owned());
    location = HRDB::versions(location).await.unwrap().last().unwrap().to_owned();
    location = HRDB::root(location).unwrap();

    HRDB::edit(
        location,
        None,
        Some(
            "#Hello!\nMy friend ( ͡° ͜ʖ ͡°) would like *you* to check back soon. It's **almost done**.".to_owned()
        ),
        None,
    ).await.unwrap();

    if let Err(e) = r {
        let r = responder::html(&format!("<h1>Error</h1><p>{}</p>", e), 500).unwrap();
        return Promise::resolve(&JsValue::from(r));
    }

    let request = event.request();
    let url = match Url::parse(&request.url()) {
        Ok(v) => v,
        Err(_) => return Promise::reject(&JsValue::from("Could not parse url")),
    };
    let path = Route::new(&url.path().to_lowercase());
    // // let method = request.method().to_lowercase();
    //
    // // construct route from event
    // // match against route
    // log("generating response for:");
    // log(url.path());
    let response = respond(path).await;

    // if the response failed, we return an internal server error
    return match response {
        Ok(response) => Promise::resolve(&JsValue::from(response)),
        Err(e) => {
            let r = responder::html(&format!("<h1>Error</h1><p>{}</p>", e), 500).unwrap();
            Promise::resolve(&JsValue::from(r))
        },
    };
}

pub async fn respond(path: Route) -> Result<Response, String> {
    match path.iter().nth(0) {
        // root -> redirect to home
        None => responder::redirect("/home")
            .ok_or("Could not redirect".to_owned()),

        // static -> retrieve a static asset
        Some(s) if s == "static"  => {
            let file = path.iter().nth(1).ok_or("Static request has no asset")?;
            let split = file
                .split(".")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let (_, kind) = (
                split.iter().nth(0).ok_or("Asset is not a file")?,
                split.iter().nth(1).ok_or("Asset has no extension")?,
            );
            let content = template::asset(file).await?;
            let response = match kind {
                h if h == "html" => responder::html(&content, 200),
                c if c == "css"  => responder::css(&content, 200),
                _                => responder::plain(&content, 200),
            }.ok_or("Could not generate static response")?;

            Ok(response)
        },

        // perma -> look up permalink
        Some(p) if p == "perma" => todo!(),

        // loc -> direct hrdb query
        Some(l) if l == "loc" => todo!(),

        // search -> search master for query
        Some(s) if s == "search" => todo!(),

        // otherwise -> call out to create hrdb query
        Some(short) => responder::html(&query(short.to_owned()).await?, 200)
            .ok_or("Could not generate page response".to_owned()),
    }
}

pub async fn query(short: String) -> Result<String, String> {
    HRDB::init().await?;

    // look up the id-path
    let ids = Shorthand::read()
        .await?.unwrap().get(&short)
        .ok_or("Shorthand does not map to Page")?
        .to_owned();

    // find and load the page location
    let master = Location::from_branch("master".to_owned());
    let head = HRDB::versions(master).await?.last()
        .ok_or("No versions found")?
        .to_owned();
    let location = HRDB::locate(head, ids).await?;
    let (title, content, _) = HRDB::read(&location).await?;

    // render the page
    return template::page(title, content).await;
}

// HRDB::init().await.unwrap();
//
// let branches = HRDB::branches().await.unwrap();
// let branch = branches.last().unwrap().to_owned();
//
// let versions = HRDB::versions(branch).await.unwrap();
// let version = versions.last().unwrap().to_owned();
//
// let root = HRDB::root(version).await.unwrap();
// let (title, content, fields) = HRDB::read(&root).await.unwrap();
// // let new_content = "My friend ( ͡° ͜ʖ ͡°) says that the website is close to being functional.";
// // HRDB::edit(
// //     root,
// //     Some(title),
// //     Some(new_content.to_string()),
// //     Some(fields),
// // ).await.unwrap();
//
// let response = JsValue::from(&responder::html(&content, 200).unwrap());
// return Promise::resolve(&response);
