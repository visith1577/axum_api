use serde::Deserialize;
use axum::{Json, Router, routing::post};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

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
    tracing::info!("->> {:12} - api_login", "HANDLER");

    if payload.username != "visith" || payload.pwd != "root" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result" : {
            "success" : true
        }
    }));

    Ok(body)
}