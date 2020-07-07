use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, FormData};

pub async fn parse(request: Request) -> Result<FormData, String> {
    let promise = request.form_data()
        .ok().ok_or("Could not get form data from request")?;
    let js_value = JsFuture::from(promise).await
        .ok().ok_or("Could not resolve FormData Promise")?;
    return Ok(FormData::from(js_value));
}
