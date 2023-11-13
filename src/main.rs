mod error;
mod web;
mod model;
mod context;
mod log;

pub use self::model::ModelController;
pub use self::error::{Error, Result};
pub use self::context::Ctx;
pub use self::log::log_request;


use axum::http::{Uri, Method};
use axum::{middleware, Json};
use axum::response::Response;
use axum::{
    routing::{get, get_service},
    Router, 
    response::{IntoResponse, Html}, 
    http::StatusCode,
    extract::{Query, Path}, 
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;
use std::net::SocketAddr;


#[derive(Deserialize, Debug)]
struct HelloParams {
    name: Option<String>
}

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();
    // Route all requests on "/" endpoint to anonymous handler

    // a handler is a async function which returns something that implements axum::response::IntoResponse
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
                                    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));


    let app = Router::new().merge(route_root())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(), 
            web::mw_auth::mw_ctx_resolver
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(route_static())
        .fallback(not_found);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // hyper::server::Server
    tracing::info!("App running on http://{}", addr);
    println!("try:  http://localhost:3000/root?name=visith");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}


async fn handler(
    Query(params): Query<HelloParams>
) -> impl IntoResponse{
    let name = params.name.unwrap();
    let html = format!("<h1>Hello {} </h1>", name);
    (StatusCode::OK ,Html(html)) 
}

async fn handler_path(
    Path(name): Path<String>
) -> impl IntoResponse {
    let name = name;
    let html = format!("<h1>Hello {} </h1>", name);
    (StatusCode::OK ,Html(html)) 
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response
) -> Response {
    tracing::info!("->> {:>12} - main_response_mapper", "RES_MAPPER");
    
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error.as_ref()
    .map(|(status_code, client_error)| {
        let client_error_body = json!({
            "error" : {
                "type" : client_error.as_ref(),
                "req_uuid" : uuid.to_string(),
            }
        });
        tracing::info!("->> - client_error_body: {client_error_body}");

        (*status_code ,Json(client_error_body)).into_response()
    });

    let client_error = client_status_error.unzip().1;

	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ =
		log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}


fn route_root() -> Router {
    Router::new()
    .route("/root", get(handler))
    .route("/root2/:name", get(handler_path))
}

fn route_static() -> Router {
    Router::new()
    .nest_service("/", get_service(ServeDir::new("./static"))
    .handle_error(|err| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }))
}