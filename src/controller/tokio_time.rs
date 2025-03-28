#![allow(unused)]

// https://docs.rs/tokio/1.38.0/tokio/time/index.html
pub async fn test_tokio_time() {
    //测试 sleep
    // test_sleep().await;

    //测试timeout
    // test_timeout().await;

    //测试周期性
    // test_interval().await;
    //测试周期性
    // test_interval_at().await;

    //到某个时间点执行
    // test_sleep_until().await;

    //超过 某个时间点 就超时
    test_timeout_at().await;
}

/****************************************** sleep *************************************************/

use chrono::Local;
use std::time::Duration;
use tokio::time::sleep;

async fn test_sleep() {
    println!("测试sleep");
    sleep(Duration::from_secs(1)).await;
    println!("测试sleep end");
}

/****************************************** timeout *************************************************/
use tokio::time::timeout;
use tokio::time::Duration as TD;
async fn test_timeout() {
    let res = timeout(TD::from_secs(1), long_future()).await;
    if let Err(err) = res {
        println!("operation timeout {:?}", err);
    }
}

async fn long_future() {
    sleep(Duration::from_secs(2)).await;
}

/****************************************** interval *************************************************/
async fn task_that_takes_a_second() {
    println!("hello");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

async fn test_interval() {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    // for _i in 0..5{
    interval.tick().await;
    task_that_takes_a_second().await;
    // }

    interval.tick().await;
    task_that_takes_a_second().await;

    // sleep(Duration::from_secs(100)).await;
}

/****************************************** interval_at *************************************************/
use tokio::time::interval_at;
use tokio::time::Instant;
async fn test_interval_at() {
    let start = Instant::now() + TD::from_secs(1);
    let mut interval = interval_at(start, TD::from_secs(2));
    println!("test interval_at {:?}", Local::now());

    interval.tick().await;
    println!("test interval_at {:?}", Local::now());
    interval.tick().await;
    println!("test interval_at {:?}", Local::now());

    interval.tick().await;
    println!("test interval_at {:?}", Local::now());
}

/****************************************** sleep_until *************************************************/
use tokio::time::sleep_until;
async fn test_sleep_until() {
    println!("test sleep_until {:?}", Local::now());
    sleep_until(Instant::now() + TD::from_secs(2)).await;
    println!("test sleep_until {:?}", Local::now());
}

/****************************************** timeout_at *************************************************/
use tokio::time::timeout_at;
async fn test_timeout_at() {
    println!("test sleep_until {:?}", Local::now());
    if let Err(err) = timeout_at(Instant::now() + TD::from_secs(2), test_timeout_at_fn()).await {
        println!("{}", err)
    }
    println!("test sleep_until {:?}", Local::now());
}

async fn test_timeout_at_fn() {
    sleep(Duration::from_secs(3)).await;
}
