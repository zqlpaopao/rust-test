#![allow(unused)]
use axum::body::{Body, Bytes};
use axum::extract::{MatchedPath, Path, State};
use axum::http::{header, HeaderMap, Request, Response, StatusCode, Uri};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use log::{error, info};
use reqwest_tracing::reqwest_otel_span_macro::private::span::Span;
use serde_json::json;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
// use tower_http::trace::TraceLayer;
use http_body_util::BodyExt;

pub fn routes() -> axum::Router {
    // define some routes separately
    // let user_routes = Router::new()
    //     .route("/users", get(users_list))
    //     .route("/users/:id", get(users_show));
    //
    // let team_routes = Router::new()
    //     .route("/teams", get(teams_list));
    //
    //
    //

    // let middleware_stack = ServiceBuilder::new()
    //     // add high level tracing of requests and responses
    //     .layer(print_request_response)
    //     // compression responses
    //     .layer(CompressionLayer::new())
    //     // convert the `ServiceBuilder` into a `tower::Layer`;
    //     .into_inner();

    // Router::new()
    //     .route("/", get(|| async { "Hello, World!" }))
    //     .layer(TraceLayer::new_for_http())

    Router::new().route("/", get(index))
    // .layer(middleware_stack)
    // .route("/url", get(all_the_things))
    //
    // .route("/users/:id", get(users_id))
    // .route("/users/:id/tweets", get(get_tweets))
    // .route("/*key", get(handler))
    //      // combine them into one

    //
    // Router::new()
    //       .route("/", get(hello_world()))
    //     // .layer(TraceLayer::new_for_http())
}

async fn index() -> &'static str {
    error!("Received request for /");
    info!("Received request for /");

    "Hello world!"
}


async fn print_request_response(
    req: Request<String>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
#[derive(Clone)]
struct Foo(&'static str);

async fn all_the_things(uri: Uri) -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    if uri.path() == "/" {
        header_map.insert(header::SERVER, "axum".parse().unwrap());
    }

    (
        // set status code
        StatusCode::NOT_FOUND,
        // headers with an array
        [("x-custom", "custom")],
        // some extensions
        Extension(Foo("foo")),
        Extension(Foo("bar")),
        // more headers, built dynamically
        header_map,
        // and finally the body
        "foo",
    )
}

async fn hello_world() -> impl axum::response::IntoResponse {
    Json(json!({"message": "Hello, World!"}))
}

#[derive(Clone, Debug)]
struct InnerState {}

#[derive(Clone, Debug)]
struct OuterState {}

async fn inner_handler(state: State<InnerState>) {
    println!("Inner handler called with state: {:?}", state);
}

async fn outer_handler(state: State<OuterState>) {
    println!("Outer handler called with state: {:?}", state);
}

async fn users_list() -> String {
    "Users list".to_string()
}

async fn users_show(Path(id): Path<String>) -> String {
    format!("User with ID {}", id)
}

async fn teams_list() -> String {
    "Teams list".to_string()
}


/********************************************** 捕获 **************************************/
async fn users_id(Path(id): Path<String>) -> String {
    format!("users/{}", id)
}

async fn get_tweets(pth: MatchedPath) {
    println!("Matched path: {:?}", pth.as_str());
}

/********************************************** 通配符 **************************************/
async fn handler(Path(path): Path<String>) -> String {
    println!("Matched path: {:?}", path);
    path
}
