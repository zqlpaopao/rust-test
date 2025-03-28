use crate::controller::axum::route::routes;
use anyhow::Result;
use axum::middleware;
use tokio::net::TcpListener;
use crate::controller::axum::middleware_log;

pub async fn test_axum() -> Result<()> {
    let app = routes();
        // .layer(middleware::from_fn(middleware_log::logging_middleware));

    println!("axum Listening on 0.0.0.0:3000");

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
