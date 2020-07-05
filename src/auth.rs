// TODO: make an authentication namespace
use cookie::Cookie;
use web_sys::Request;
use js_sys::JSON;
use time::Duration;
use crate::hrdb::utils;
use crate::kv;
use crate::logger::log;

// AuthNS stores cookie code hashes ('checks') -> said codes.
// A cookie stores a check.
// When a cookie is to be validated, its check is looked up in  AuthNS.
// the retrieved value is then hashed and compared against the cookie's check.
// the check is largely pointless, I just feel bad not using the stored value.

pub async fn validate(request: &Request) -> bool {
    let headers = request.headers();
    let cookie_header = match headers.get("cookie") {
        Ok(Some(v)) => v,
        _ => return false,
    };

    for cookie_str in cookie_header.split(';').map(|s| s.trim()) {
        if let Ok(c) = Cookie::parse(cookie_str) {
            if c.name() == "auth_code" {
                let check = c.value().to_owned();
                return match kv::value(kv::AuthNS::get(&check, "text")).await {
                    Some(v) => {
                        let code = match v.as_string() {
                            Some(v) => v,
                            None => return false
                        };
                        return utils::hash(&code) == check
                    },
                    None => false,
                }
            }
        }
    }
    return false;
}

pub async fn session<'a>() -> Result<Cookie<'a>, String> {
    let code  = utils::stamp()?;
    let check = utils::hash(&code);

    // cookies expire in 8 days = 691200 seconds
    let expiration = JSON::parse("{\"expirationTtl\": 691200}")
        .ok().ok_or("Could not set session expiration")?;
    kv::value(kv::AuthNS::put(&check, &code, expiration)).await
        .ok_or("Could not record session server-side")?;

    let session = Cookie::build("auth_code", check)
        .domain("slightknack.dev")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::seconds(691200))
        .finish();

    return Ok(session);
}

pub async fn check(password: String) -> Result<bool, String> {
    let hash = kv::value(kv::AuthNS::get("password", "text")).await
        .ok_or("Could not retrieve stored password hash")?
        .as_string()
        .ok_or("Could not unwrap the hash")?;

    let salt = kv::value(kv::AuthNS::get("salt", "text")).await
        .ok_or("Could not retrieve stored password salt")?
        .as_string()
        .ok_or("Could not unwrap the salt")?;

    let salted = password + &salt;
    let attempt = utils::hash(&salted);
    return Ok(hash == attempt);
}
