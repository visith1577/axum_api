use serde::Deserialize;
use axum::{Json, Router, routing::post};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies, cookie::time::{OffsetDateTime, Duration}};
use tracing::debug;

use crate::{Error, Result, web::AUTH_TOKEN};


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies:  Cookies, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    debug!("{:12} - api_login", "HANDLER");

    if payload.username != "visith" || payload.pwd != "root" {
        return Err(Error::LoginFail);
    }

    let mut cookie = Cookie::new(AUTH_TOKEN, "user-1.exp.sign");
    cookie.set_http_only(true);
    cookie.set_path("/");
    let mut now = OffsetDateTime::now_utc();
    now += Duration::weeks(52);
    cookie.set_expires(now);
    cookies.add(cookie);

    let body = Json(json!({
        "result" : {
            "success" : true
        }
    }));

    Ok(body)
}