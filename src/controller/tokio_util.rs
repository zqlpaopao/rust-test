#![allow(dead_code)]
use chrono::Utc;
use tokio::task;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

pub async fn test_tokio_util() {
    //上下文任务取消
    // test_cancel_token().await;

    //重新利用future内存
    test_box_future().await;
}

///  CancellationToken
async fn test_cancel_token() {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    let task = task::spawn(async move {
        loop {
            println!(
                "cancel task -time:{}",
                Utc::now().format("%Y-%m-%d %H:%M:%S")
            );
            tokio::select! {
                res = cloned_token.cancelled() =>{
                    println!("cancelled time:{} , {:?}",Utc::now().format("%Y-%m-%d %H:%M:%S"),res);
                    break;
                }
                timeout = tokio::time::sleep(std::time::Duration::from_secs(1))=>{
                    println!("timeout time:{} ,{:?}",Utc::now().format("%Y-%m-%d %H:%M:%S"),timeout);
                }
            }
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    token.cancel();

    let res = task.await;
    println!("time:{},{:?}", Utc::now().format("%Y-%m-%d %H:%M:%S"), res);
}

//
async fn example_task(i: u32) -> u32 {
    sleep(std::time::Duration::from_millis(100)).await;
    i * 2
}

async fn test_box_future() {
    let tracker = TaskTracker::new();

    // 启动一个异步任务并跟踪它
    let task1 = task::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        println!("Task 1 completed");
    });
    tracker.spawn(task1);

    // 启动另一个异步任务并跟踪它
    let task2 = task::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        println!("Task 2 completed");
    });
    tracker.spawn(task2);
    tracker.close();

    // 等待所有跟踪的任务都完成
    tracker.wait().await;

    println!("All tasks completed");
}
