use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use web_sys::Response;
use js_sys::Object;

use crate::responder;
use crate::route::Route;
use crate::kv;
use crate::logger::log;
use crate::hrdb::utils::read;

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

pub async fn get_all_values(keys: Vec<String>) -> Result<Vec<(String, String)>, String> {
    let mut values = vec![];
    for key in keys.into_iter() {
        values.push((
            key.to_string(),
            read(&key).await?,
        ));
    }
    return Ok(values);
}

pub async fn respond(_path: Route) -> Result<Response, String> {
    let keys = kv::value(kv::AddressedNS::list()).await
        .ok_or("Could not list keys")?;

    // let key_object = Object::try_from(&keys)
    //     .ok_or("convert jsvalue to object")?;

    let keys_rust = keys.into_serde::<KeysRust>()
        .ok().ok_or("Could not read the key object into a KeysRust")?;
    let key_list = to_key_list(keys_rust);

    let values = get_all_values(key_list).await?;

    let debug = values.into_iter()
        .map(|(k, v)| format!("{}\n--|\n{}", k, v))
        .collect::<Vec<String>>()
        .join("\n|--\n\n");

    let response = responder::plain(&debug, 200)
        .ok_or("Could not dump keys for namespaces")?;

    Ok(response)
}
