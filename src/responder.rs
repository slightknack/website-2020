use wasm_bindgen::JsValue;
use web_sys::{Headers, Response, ResponseInit};

pub fn respond(content: &str, kind: &str, status: u16) -> Option<Response> {
    let mut headers: Headers = Headers::new().ok()?;
    headers.append("content-type", kind).ok()?;

    let mut init = ResponseInit::new();
    init.status(status);
    init.headers(&JsValue::from(headers));

    return Some(Response::new_with_opt_str_and_init(Some(content), &init).ok()?);
}

pub fn html(content: &str, status: u16) -> Option<Response> {
    respond(content, "text/html; charset=utf-8", status)
}
