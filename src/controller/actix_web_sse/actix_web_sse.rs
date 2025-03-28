#![allow(unused)]
use actix_cors::Cors;
use actix_web::web::Bytes;
use actix_web::{web, App, HttpServer, Responder};
use futures::Stream;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::interval;

/// https://mp.weixin.qq.com/s/oywwJyKzgnRcw71Ik0eFGQ

// type SseStream = Pin<Box<dyn Stream<Item = Result<Bytes, actix_web::Error>>>>;

async fn sse_handler() -> impl Responder {
    let mut interval = interval(Duration::from_secs(5));
    let server_events = async_stream::stream! {
        let mut counter = 0;
        loop {
            interval.tick().await;
            counter += 1;
            let data = format!("data: count {}\n\n", counter);
            yield Ok::<_, actix_web::Error>(Bytes::from(data));
        }
    };

    actix_web::HttpResponse::Ok()
        .content_type("text/event-stream")
        .no_chunking(100)
        .streaming(server_events)
}

pub async fn test_sses() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .route("/sse", web::get().to(sse_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
