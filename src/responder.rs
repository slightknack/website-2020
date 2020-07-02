use wasm_bindgen::JsValue;
use web_sys::{Headers, Response, ResponseInit};
use cookie::Cookie;

pub fn respond(content: &str, headers: Headers, status: u16) -> Option<Response> {
    let mut init = ResponseInit::new();
    init.status(status);
    init.headers(&JsValue::from(headers));
    return Some(Response::new_with_opt_str_and_init(Some(content), &init).ok()?);
}

pub fn content(c: &str, kind: &str, status: u16) -> Option<Response> {
    let mut headers: Headers = Headers::new().ok()?;
    headers.append("content-type", kind).ok()?;
    respond(c, headers, status)
}

pub fn html(c: &str, status: u16) -> Option<Response> {
    content(c, "text/html; charset=utf-8", status)
}

pub fn css(c: &str, status: u16) -> Option<Response> {
    content(c, "text/css; charset=utf-8", status)
}

pub fn plain(c: &str, status: u16) -> Option<Response> {
    content(c, "text/plain; charset=utf-8", status)
}

pub fn redirect(url: &str) -> Option<Response> {
    let mut headers: Headers = Headers::new().ok()?;
    headers.set("location", url).ok()?;
    respond("", headers, 302)
}

pub fn cookie(c: Cookie, redirect: &str) -> Option<Response> {
    let mut headers: Headers = Headers::new().ok()?;
    headers.set("location", redirect).ok()?;
    headers.set("set-cookie", &c.to_string()).ok()?;
    respond("", headers, 302)
}
