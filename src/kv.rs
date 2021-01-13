use wasm_bindgen::prelude::*;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    pub type StaticNS;

    #[wasm_bindgen(static_method_of = StaticNS)]
    pub fn get(key: &str, data_type: &str) -> Promise;

    #[wasm_bindgen(static_method_of = StaticNS)]
    pub fn put(key: &str, val: &str) -> Promise;

    #[wasm_bindgen(static_method_of = StaticNS)]
    pub fn delete(key: &str) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type AddressedNS;

    #[wasm_bindgen(static_method_of = AddressedNS)]
    pub fn get(key: &str, data_type: &str) -> Promise;

    #[wasm_bindgen(static_method_of = AddressedNS)]
    pub fn put(key: &str, val: &str) -> Promise;

    #[wasm_bindgen(static_method_of = AddressedNS)]
    pub fn delete(key: &str) -> Promise;

    #[wasm_bindgen(static_method_of = AddressedNS)]
    pub fn list() -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type AuthNS;

    #[wasm_bindgen(static_method_of = AuthNS)]
    pub fn get(key: &str, data_type: &str) -> Promise;

    #[wasm_bindgen(static_method_of = AuthNS)]
    pub fn put(key: &str, val: &str, params: JsValue) -> Promise;

    #[wasm_bindgen(static_method_of = AuthNS)]
    pub fn delete(key: &str) -> Promise;
}

pub async fn value(promise: Promise) -> Option<JsValue> {
    JsFuture::from(promise).await.ok()
}
