//
//
// /// https://mp.weixin.qq.com/s/ynAuVRSZo_PSAEgGLZcrTw
//
// use crate::controller::proto::greeter::{HelloRequest,HelloResponse,greeter_server};
// use tonic::{transport::Server,Request,Response,Status};
//
//
// pub async fn test_tonic(){
//     //生成proto.pb
//     // make_proto_rpc()
//
//     //服务端
//     server().await
// }
//
// // *************************************** 生成 服务端
//
// // pub mod greeter {
// //     tonic::include_proto!("greeter"); // 包含生成的代码
// // }
// #[derive(Debug,Default)]
// pub struct MyGreeter{}
//
// #[tonic::async_trait]
// impl greeter_server::Greeter for MyGreeter {
//     async fn say_hello(
//         &self,
//         request: Request<HelloRequest>,
//     ) -> Result<Response<HelloResponse>, Status> {
//         println!("Got a request from {:?}", request.remote_addr());
//
//         let reply = HelloResponse {
//             message: format!("Hello {}!", request.into_inner().name),
//         };
//         Ok(Response::new(reply))
//     }
// }
//
// async  fn server(){
//     tracing_subscriber::fmt::init();
//     let addr = "127.0.0.1:3000".parse().unwrap();
//
//     let greeter = MyGreeter::default();
//     let greeter = greeter_server::GreeterServer::new(greeter);
//
//     println!("GreeterServer listening on {}", addr);
//
//     Server::builder()
//         // GrpcWeb is over http1 so we must enable it.
//         // .accept_http1(true)
//         .add_service(greeter)
//         .serve(addr)
//         .await.unwrap();
//
// }
//
//
// // *************************************** 生成 proto.pb
// pub fn make_proto_rpc(){
//     // tonic_build::compile_protos("/Users/zhangqiuli24/Desktop/rust/rust_test/my_test/src/controller/proto/greeter.proto").unwrap();
//     tonic_build::configure()
//         .
//         .build_server(true)//生成服务端
//         .out_dir("/Users/zhangqiuli24/Desktop/rust/rust_test/my_test/src/controller/proto/")
//         .compile(&["/Users/zhangqiuli24/Desktop/rust/rust_test/my_test/src/controller/proto/greeter.proto"],
//         &["/Users/zhangqiuli24/Desktop/rust/rust_test/my_test/src/controller/proto/"])
//         .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
// }
