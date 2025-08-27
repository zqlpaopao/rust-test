#![allow(unused)]

use std::time::Duration;
use crate::controller::axum::middleware_log;
use crate::controller::axum::route::routes;
use anyhow::Result;
use axum::middleware;
use tokio::net::TcpListener;
use tower::limit::{ConcurrencyLimitLayer, GlobalConcurrencyLimitLayer, rate::RateLimitLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub async fn test_axum() -> Result<()> {

    // 初始化日志订阅器
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE) // 显示请求开始和结束
        .init();

    let app = routes();
    // .layer(middleware::from_fn(middleware_log::logging_middleware));

    let app = app
        // 1、
        // HTTP 请求追踪和日志记录
        // 记录每个请求的详细信息，包括：
        // 请求开始和结束时间
        // HTTP 方法、路径、状态码
        // 处理耗时
        // 错误信息
        // 用途：监控、调试、性能分析
        .layer(TraceLayer::new_for_http())
            // TraceLayer::new_for_http()
            //                 .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            //                 .on_response(DefaultOnResponse::new().level(Level::INFO))

            //输出
            // 2025-08-27T02:32:19.746022Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_request: started processing request
            // 2025-08-27T02:32:19.746179Z ERROR request{method=GET uri=/ version=HTTP/1.1}: my_test::controller::axum::route: Received request for /
            // 2025-08-27T02:32:19.746214Z  INFO request{method=GET uri=/ version=HTTP/1.1}: my_test::controller::axum::route: Received request for /
            // 2025-08-27T02:32:19.746279Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
            // 2025-08-27T02:32:19.746442Z DEBUG request{method=GET uri=/ version=HTTP/1.1}: tower_http::trace::make_span: close time.busy=316µs time.idle=120µs

        // 2、
        // 功能：请求超时控制
        // 参数：Duration::from_secs(2) - 2秒超时
        // 作用：如果请求处理时间超过2秒，自动返回超时错误
        // 用途：防止请求长时间阻塞，提高系统稳定性
        // .layer(TimeoutLayer::new(Duration::from_secs(2)))

        // 功能：并发连接数限制
        // 参数：256 - 最大并发连接数
        // 作用：限制同时处理的请求数量为256个
        // 用途：防止系统过载，保护后端服务
        // .layer(ConcurrencyLimitLayer::new(256));
        .layer(GlobalConcurrencyLimitLayer::new(2));

        // 功能：请求速率限制
        // 参数：
        // 100 - 每秒最多100个请求
        // Duration::from_secs(1) - 时间窗口为1秒
        // 作用：限制请求频率，防止DDoS攻击或滥用
        // 用途：API限流，保护系统资源
        // .layer(RateLimitLayer::new(100, Duration::from_secs(1));

    println!("axum Listening on 0.0.0.0:3000");

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
