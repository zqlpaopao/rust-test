#![allow(unused)]

pub async fn test_context() {
    //test_cancel
    // 任务取消
    // test_cancel().await

    // 上下文传播
    // test_watch().await

    //future abort
    test_future_abort().await
}

use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;
// cancel
async fn test_cancel() {
    // 创建一个取消令牌
    let token = CancellationToken::new();

    // 创建一个子任务，并传递取消令牌的克隆
    let child_token = token.clone();
    let handle = tokio::spawn(async move {
        // 模拟一个长时间运行的任务
        for i in 0..10 {
            if child_token.is_cancelled() {
                println!("Task was cancelled");
                return;
            }
            println!("Working on task {}", i);
            sleep(Duration::from_secs(1)).await;
        }
        println!("Task completed");
    });

    // 等待一段时间后取消任务
    sleep(Duration::from_secs(3)).await;
    token.cancel();

    // 等待子任务完成
    let _ = handle.await;
}

//Working on task 0
// Working on task 1
// Working on task 2
// Task was cancelled

//******************************* watch
use tokio::sync::watch;
async fn test_watch() {
    let (tx, mut rx) = watch::channel("Initial context".to_string());

    // 创建一个子任务，并传递接收者
    let handle = tokio::spawn(async move {
        loop {
            if let Ok(message) = rx.changed().await {
                println!("Received context: {}", *rx.borrow());
            }
        }
    });

    // 修改上下文信息
    sleep(Duration::from_secs(2)).await;
    tx.send("Updated context".to_string()).unwrap();

    sleep(Duration::from_secs(2)).await;
    tx.send("Another update".to_string()).unwrap();

    // 等待子任务完成
    let _ = handle.await;
}

//Received context: Updated context
// Received context: Another update

//******************************* futures::future::Abortable
use futures::future::{abortable, AbortHandle};

async fn test_future_abort() {
    // 创建一个可取消的任务和一个取消句柄
    let task = async {
        for i in 0..10 {
            println!("Working on task {}", i);
            sleep(Duration::from_secs(1)).await;
        }
        println!("Task completed");
    };
    let (abort_handle, abort_registration) = abortable(task);

    // 启动任务
    let handle = tokio::spawn(abort_handle);

    // 等待一段时间后取消任务
    sleep(Duration::from_secs(3)).await;
    abort_registration.abort();

    // 等待任务完成
    let _ = handle.await;
}
