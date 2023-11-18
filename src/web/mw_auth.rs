use crate::{Result, Error, ModelController};
use crate::Ctx;

use axum::extract::State;
use axum::http::request::Parts;
use axum::{response::Response, http::Request, middleware::Next, extract::FromRequestParts, async_trait};
use lazy_regex::regex_captures;
use tower_cookies::cookie::time::{OffsetDateTime, Duration};
use tower_cookies::{Cookies, Cookie};
use tracing::debug;

use super::AUTH_TOKEN;


pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    debug!("{:12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    debug!("{:12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|mut t| {
        let mut now = OffsetDateTime::now_utc();
        now += Duration::weeks(52);
        t.set_expires(now);
        t.value().to_string()
    });

    let result_ctx = match  auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e)
    };

    if result_ctx.is_err() 
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
    {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}


// region: --Ctx Extractor

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:12} - Ctx", "EXTRACTOR");

        parts
        .extensions.get::<Result<Ctx>>()
        .ok_or(Error::AuthFailCtxNotInRequestExt)?
        .clone()
    }
}


// endregion: --Ctx Extractor

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.into(), sign.into()))
}