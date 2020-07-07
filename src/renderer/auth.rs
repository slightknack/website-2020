use web_sys::{Response, Request};
use crate::renderer::form;
use crate::responder;
use crate::template;
use crate::auth;

pub async fn respond() -> Result<Response, String> {
    let html = template::auth::render().await?;
    responder::html(&html, 200)
        .ok_or("Could not generate authentication Page response".to_owned())
}

pub async fn form(request: Request) -> Result<Response, String> {
    let redirect = match request.headers().get("referer") {
        Ok(Some(v)) => v,
        _ => request.url(),
    };

    let form = form::parse(request).await?;
    let password = form.get("password").as_string()
        .ok_or("Could not retrieve password from request")?;

    return if auth::check(password).await? == false {
        responder::redirect(&redirect)
            .ok_or("Password was incorrect and could not generate redirect".to_owned())
    } else {
        let session = auth::session().await?;
        responder::cookie(session, "/home")
            .ok_or("Could not generate response for authentication".to_owned())
    }
}
