use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use web_sys::Response;
use js_sys::Object;

use crate::responder;
use crate::route::Route;
use crate::kv;
use crate::logger::log;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysRust {
    list_complete: bool,
    keys: Vec<KeyRust>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyRust {
    name: String,
}

pub fn to_key_list(keys_rust: KeysRust) -> Vec<String> {
    let mut result = vec![];

    for key_rust in keys_rust.keys.into_iter() {
        result.push(key_rust.name);
    }

    return result;
}

pub async fn respond(_path: Route) -> Result<Response, String> {
    let keys = kv::value(kv::AddressedNS::list()).await
        .ok_or("Could not list keys")?;

    // let key_object = Object::try_from(&keys)
    //     .ok_or("convert jsvalue to object")?;

    let keys_rust = keys.into_serde::<KeysRust>();
    let key_list = to_key_list(keys_rust);

    let debug = format!("1: {:#?}", key_list);

    let response = responder::plain(&debug, 200)
        .ok_or("Could not dump keys for namespaces")?;

    Ok(response)
}
