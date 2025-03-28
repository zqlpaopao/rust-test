// //// https://mp.weixin.qq.com/s/OQs0hq0uKlF7R0L58lAhLA
//
// use std::time::Instant;
// use axum::body::Body;
// use axum::http::{Request};
// use axum::middleware::Next;
// use axum::response::Response;
//
// //日志中间件函数
// pub(crate) async fn logging_middleware(request:Request<Body>,next: Next)->Response{
//     let path = request.uri().path().to_owned();
//     let method = request.method().to_owned();
//
//     //记录开始时间
//     let start = Instant::now();
//
//
//     //将请求传递给下一个中间件或者处理程序
//     let response = next.run(request).await;
//
//     //计算请求处理时间
//     let latency = start.elapsed();
//
//     //记录请求信息
//     println!("{} {} - {:?}",method,path,latency);
//
//     //返回响应
//     response
// }