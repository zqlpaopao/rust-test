
/// https://mp.weixin.qq.com/s/_5Xx0VVY2AYPzBcS4DKqGQ

use std::time::Duration;
use tower::{ServiceBuilder, timeout::TimeoutLayer};
use tower::layer::util::{Identity, Stack};
use tower::limit::{ConcurrencyLimitLayer, RateLimitLayer};
use tower_http::trace::{HttpMakeClassifier, TraceLayer};

pub async  fn rate_limit() -> Stack<RateLimitLayer, Stack<ConcurrencyLimitLayer, Stack<TimeoutLayer, Stack<TraceLayer<HttpMakeClassifier>, Identity>>>> {
    ServiceBuilder::new()
        // HTTP 请求追踪和日志记录
        // 记录每个请求的详细信息，包括：
        // 请求开始和结束时间
        // HTTP 方法、路径、状态码
        // 处理耗时
        // 错误信息
        // 用途：监控、调试、性能分析
        .layer(TraceLayer::new_for_http())

        // 功能：请求超时控制
        // 参数：Duration::from_secs(2) - 2秒超时
        // 作用：如果请求处理时间超过2秒，自动返回超时错误
        // 用途：防止请求长时间阻塞，提高系统稳定性
        .layer(TimeoutLayer::new(Duration::from_secs(2)))

        // 功能：并发连接数限制
        // 参数：256 - 最大并发连接数
        // 作用：限制同时处理的请求数量为256个
        // 用途：防止系统过载，保护后端服务
        .layer(ConcurrencyLimitLayer::new(256))

        // 功能：请求速率限制
        // 参数：
        // 100 - 每秒最多100个请求
        // Duration::from_secs(1) - 时间窗口为1秒
        // 作用：限制请求频率，防止DDoS攻击或滥用
        // 用途：API限流，保护系统资源
        .layer(RateLimitLayer::new(100, Duration::from_secs(1)))
        .into_inner()
}

