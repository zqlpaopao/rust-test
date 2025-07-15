use hyper::{Body, Client, Request};
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
struct Config {
    network: NetworkConfig,
}

#[derive(Deserialize)]
struct NetworkConfig {
    url: String,
}

async fn fetch_block(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let req = Request::builder().uri(url).body(Body::empty())?;
    let res = client.request(req).await?;
    let _body = hyper::body::to_bytes(res.into_body()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--config" {
        eprintln!("Usage: {} --config <config_file>", args[0]);
        std::process::exit(1);
    }

    let config_content = fs::read_to_string(&args[2])?;
    let config: Config = toml::from_str(&config_content)?;
    for _ in 0..100 {
        fetch_block(&config.network.url).await?;
    }
    Ok(())
}
