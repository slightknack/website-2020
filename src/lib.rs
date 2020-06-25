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

use std::usize;
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
    // let r = HRDB::init().await;

    let request = event.request();
    let url = match Url::parse(&request.url()) {
        Ok(v) => v,
        Err(_) => return Promise::reject(&JsValue::from("Could not parse url")),
    };
    let path = Route::new(&url.path().to_lowercase());
    let response = respond(path).await;

    // if the response failed, we return an internal server error
    return match response {
        Ok(response) => Promise::resolve(&JsValue::from(response)),
        Err(e) => {
            let html = template::error(e).await.unwrap();
            let r = responder::html(&html, 500).unwrap();
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
        Some(s) if s == "static" => {
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
            log(&format!("getting {:?}", file));
            let content = template::asset(file).await?;
            log(&format!("{:?} content", file));

            let response = match kind {
                h if h == "html" => responder::html(&content, 200),
                c if c == "css"  => responder::css(&content, 200),
                _                => responder::plain(&content, 200),
            }.ok_or("Could not generate response for static asset")?;
            Ok(response)
        },

        // edit -> load the editor '/branch/id'
        Some(e) if e == "edit" => {
            let (b, id) = (
                path.iter().nth(1).ok_or("No branch specified")?,
                path.iter().nth(2).ok_or("No id specified")?,
            );
            let branch = Location::from_branch(b.to_owned());
            let versions = HRDB::versions(branch).await?;
            let head = versions.last().ok_or("No versions exist on this branch")?;
            let location = HRDB::locate_id(head.to_owned(), id.to_owned()).await?;
            let (title, content, _) = HRDB::read(&location).await?;

            let html = template::edit(title, content).await?;
            responder::html(&html, 200)
                .ok_or("Could not load the editor".to_owned())
        },

        // perma -> direct hrdb query '/branch/version_no/id'
        Some(p) if p == "perma" => {
            let (b, vn, id) = (
                path.iter().nth(1).ok_or("No branch specified")?,
                path.iter().nth(2).ok_or("No version number specified")?,
                path.iter().nth(3).ok_or("No id specified")?,
            );

            let branch = Location::from_branch(b.to_owned());
            let ver_no = vn.parse::<usize>()
                .ok().ok_or("Version number was not a number")?;
            let versions = HRDB::versions(branch).await?;
            let version = versions.iter()
                .nth(ver_no).ok_or("Version with that number does not exist")?;
            // locate_id fairly expensive, might revise schema...
            let location = HRDB::locate_id(version.to_owned(), id.to_owned()).await?;
            let parent = match location.back() {
                Ok(p)  => p.id().await?,
                Err(_) => location.id().await?,
            };

            let (title, content, _) = HRDB::read(&location).await?;
            let html = template::page(
                title,
                content,
                b.to_owned(),
                ver_no,
                id.to_owned(),
                parent,
            ).await?;
            responder::html(&html, 200)
                .ok_or("Could not generate response for location query".to_owned())
        }

        // search -> search master for query
        Some(s) if s == "search" => Err("Searching is not yet implemented".to_owned()),

        // branches -> list all branches
        Some(b) if b == "branches" => {
            let branches = HRDB::branches().await?;
            let names = branches.iter()
                .map(|l| vec![l.branch()])
                .collect::<Vec<Vec<String>>>();

            let html = template::table(
                "Branches".to_owned(),
                vec!["Name".to_owned()],
                names,
            ).await?;
            responder::html(&html, 200)
                .ok_or("Could not generate response listing branches".to_owned())
        }

        // Err("Listing branches is not yet implemented".to_owned()),

        // versions -> list all versions
        Some(v) if v == "versions" => Err("Listing versions is not yet implemented".to_owned()),

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
    let versions = HRDB::versions(master).await?;
    let head = versions.last()
        .ok_or("No versions found")?
        .to_owned();
    let ver_no = versions.len() - 1;
    let location = HRDB::locate(head, ids.clone()).await?;
    let parent = match location.back() {
        Ok(p)  => p.id().await?,
        Err(_) => location.id().await?,
    };

    // render the page
    let (title, content, _) = HRDB::read(&location).await?;
    return template::page(
        title,
        content,
        location.branch(),
        ver_no,
        ids.last().ok_or("No id exists")?.to_owned(),
        parent,
    ).await;
}
