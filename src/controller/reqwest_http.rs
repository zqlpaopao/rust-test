use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;

pub async fn test_middleware() {
    // Retry up to 3 times with increasing intervals between attempts.
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(13);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Trace HTTP requests. See the tracing crate to make use of these traces.
        .with(TracingMiddleware::default())
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    println!("{:?}", client);

    run(client).await;
}

async fn run(client: ClientWithMiddleware) {
    let response = client
        // .get("test.joypaw.jd.com/topology/plan/ping")
        .get("https://jd.com")
        .header("foo", "bar")
        .send()
        .await;

    match response {
        Ok(res) => {
            println!("{}", res.text().await.unwrap());
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
