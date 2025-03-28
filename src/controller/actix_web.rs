#![allow(unused)]
use crate::controller::thiserror::MyError;
use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_multipart::form::{
    tempfile::{TempFile, TempFileConfig},
    MultipartForm,
};
use actix_web::body::BoxBody;
use actix_web::cookie::time::macros::time;
use actix_web::dev::Payload;
use actix_web::error::ContentTypeError;
use actix_web::guard::{Guard, GuardContext};
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::web::{route, service, Bytes};
use actix_web::{
    error, get, guard, http, http::header::ContentType, middleware::Logger, post, web, App, Error,
    HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, Result,
};
use clap::builder::Str;
use derive_more::Display;
use env_logger::Env;
use futures::future::ok;
use futures::stream;
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio::signal::ctrl_c;
use tokio::time;

async fn post2(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("post2 app_name {}", app_name)
}

#[get("/ok")]
async fn ok1() -> impl Responder {
    HttpResponse::Ok().body("ok")
}
fn config1(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/t2")
            .route(web::get().to(|| async { HttpResponse::Ok().body("get2") }))
            // 很明显，这里可以做权限拦截使用，限制请求的方法，或者解析Token等操作
            .route(web::to(|| async { HttpResponse::Ok().body("not allowed") })),
    );
}

fn config2(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/t3")
            .route(web::post().to(|| async { HttpResponse::Ok().body("get3") }))
            // 其实这里可以看到，如果不指定方法，则匹配所有方法
            .route(web::to(|| async { HttpResponse::Ok().body("not allowed") })),
    );
}

// This struct represents state
struct AppState {
    app_name: String,
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users")] // <- define path parameters
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[derive(Deserialize)]
struct Info {
    username: String,
    age: usize,
}

#[derive(Serialize)]
struct Res {
    name: &'static str,
    address: String,
    age: u8,
}

impl Responder for Res {
    type Body = BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/stream")]
async fn json() -> HttpResponse {
    let body = stream::once(ok::<_, Error>(web::Bytes::from_static(b"test")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

//////////////////////////////////////////// 自定义错误
#[derive(Debug, Display, derive_more::Error)]
enum MyErr {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = " timeout")]
    Timeout,
}

impl error::ResponseError for MyErr {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyErr::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyErr::BadClientData => StatusCode::BAD_REQUEST,
            MyErr::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

#[derive(Debug)]
struct MyErrors {
    name: &'static str,
}

struct ContentTypeHeader;

impl Guard for ContentTypeHeader {
    fn check(&self, req: &GuardContext) -> bool {
        req.head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}
async fn err(req: HttpRequest) -> Result<String> {
    Ok(String::from("OK"))
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        f.file.persist(path).unwrap();
    }
    Ok(HttpResponse::Ok())
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/a/{name}")]
async fn indexs(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http::header};

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

use actix_files::NamedFile;
use actix_web_actors::ws;

async fn file(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

use actix_web::{dev::ServiceRequest, middleware};
use actix_web_httpauth::{
    extractors::basic::BasicAuth, extractors::bearer::BearerAuth, middleware::HttpAuthentication,
};
async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("{:#?}", req);
    println!("{:#?}", _credentials);
    println!("{:#?}", _credentials.password());
    println!("{:#?}", _credentials.user_id());
    Ok(req)
}

async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("{:#?}", req);
    println!("{:#?}", credentials);
    println!("{:#?}", credentials.token());
    Ok(req)
}
pub async fn web() {
    HttpServer::new(|| {
        App::new()
            .wrap(HttpAuthentication::bearer(ok_validator))
            .wrap(Cors::permissive())
            .service(ok1)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}
