#![allow(unused)]
// Getting Started
// https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

pub async fn test_proto() {
    // make_proto();

    //服务端
    // make_server().await;

    //客户端
    make_client().await;
}
/************************************* 中间件 ******************************/

fn auth_interceptor(request: Request<()>) -> Result<Request<()>, Status> {
    println!("auth_interceptor request: {:?}", request);

    if valid_credentials(&request) {
        Ok(request)
    } else {
        Err(Status::unauthenticated("invalid credentials"))
    }
    //auth_interceptor request:
    // Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.12.3"} }, message: (), extensions: Extensions }
}

fn valid_credentials(request: &Request<()>) -> bool {
    println!("valid_credentials request: {:?}", request);
    true
    //valid_credentials request: Request
    // { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc",
    // "user-agent": "tonic/0.12.3"} }, message: (), extensions: Extensions }
}

fn some_other_interceptor(request: Request<()>) -> Result<Request<()>, Status> {
    println!("some_other_interceptor request: {:?}", request);
    Ok(request)
    //some_other_interceptor request: Request { metadata: MetadataMap {
    // headers: {"te": "trailers", "content-type": "application/grpc",
    // "user-agent": "tonic/0.12.3"} }, message: (), extensions: Extensions }
}

/************************************* grpc client ******************************/
use crate::controller::grpc_protos_tonic::proto::hello_world::greeter_client::GreeterClient;
use crate::controller::grpc_protos_tonic::proto::hello_world::HelloRequest;

async fn make_client() {
    let mut client = GreeterClient::connect("http://0.0.0.0:3000").await.unwrap();

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await.unwrap();

    println!("RESPONSE={:?}", response);
}

/************************************* server ******************************/
use crate::controller::grpc_protos_tonic::proto::hello_world::greeter_server::{
    Greeter, GreeterServer,
};
use crate::controller::grpc_protos_tonic::proto::hello_world::HelloReply;
use std::time::Duration;
use tonic::service::interceptor;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower::ServiceBuilder;
#[derive(Default, Debug)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = HelloReply {
            message: "response".to_string(),
        };

        Ok(Response::new(reply))
    }
}

async fn make_server() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let greeter = MyGreeter::default();

    //中间件
    let layer = ServiceBuilder::new()
        .load_shed()
        .timeout(Duration::from_secs(30))
        .layer(interceptor(auth_interceptor))
        .layer(interceptor(some_other_interceptor))
        .into_inner();

    Server::builder()
        // .accept_http1(true) //启用http
        // .layer(GrpcWebLayer::new())// 启用http
        .layer(layer) //加载中间件
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}

/************************************* proto to rs ******************************/
/// https://docs.rs/tonic-build/0.12.3/tonic_build/
fn make_proto() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/controller/grpc_protos_tonic/proto/")
        .compile_protos(
            //要格式化的文件
            &["src/controller/grpc_protos_tonic/proto/hello.proto"],
            //包含的依赖项的目录
            &["src/controller/grpc_protos_tonic/proto/"],
        )
        .unwrap();
}
