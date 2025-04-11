#![allow(unused)]
use std::time::Duration;
use tokio::sync::{broadcast, mpsc, oneshot, watch};

// https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247486703&idx=1&sn=3946034dd136af916336ae04b4fd6fc3&chksm=cfc2a18cf8b5289a8f14eec502a6f4f8d2222e8c5e8da61fcafcba91d70e64367b995a5059b9&scene=178&cur_album_id=2361592668113420289#rd
pub async fn test_channel() {
    //单生产者、单消费者
    // oneshot().await;

    //单生产者、多消费者
    // watch().await;

    // 多生产者、多消费者
    broadcast().await;

    //多个生产者，单一消费者
    // mpsc().await;
}

//多个生产者，单一消费者
async fn mpsc() {
    let (tx, mut rx) = mpsc::channel(3);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            println!("send value {}", i);
            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        while let Some(msg) = rx.recv().await {
            println!("Received: {}", msg);
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(6)).await;
    //Received: 0
    // Received: 1
    // Received: 2
    // Received: 3
    // Received: 4
}

// 多生产者、多消费者
pub async fn broadcast() {
    let (tx, mut rx1) = broadcast::channel(2);
    let mut rx2 = tx.subscribe();

    let task1 = tokio::spawn(async move {
        tokio::select! {
            res = rx1.recv() => {
                println!("Task 1 is cancelling...{}",res.unwrap());
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task 1 completed normally");
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        tokio::select! {
            res = rx2.recv() => {
                println!("Task 2 is cancelling...{}",res.unwrap());
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task 2 completed normally");
            }
        }
        println!("Task 2 is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // 发送取消信号
    let _ = tx.send("hello");
    let _ = tx.send("hellos");

    // 等待任务完成
    let _ = tokio::join!(task1, task2);

    //Task 2 is cancelling...
    // Task 2 is cleaning up
    // Task 1 is cancelling...
    // Task 1 is cleaning up
}

//单生产者、多消费者
pub async fn watch() {
    let (tx, mut rx1) = watch::channel(false);

    let mut rx2 = tx.subscribe();

    let task1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                res = rx1.changed() => {
                    if *rx1.borrow() {
                        println!("Task 1 is cancelling...{:?}",res);
                        break;
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 1 completed normally");
                    break;
                }
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        loop {
            tokio::select! {
                res = rx2.changed() => {
                    if *rx2.borrow() {
                        println!("Task 2 is cancelling...{:?}",res);
                        break;
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 2 completed normally");
                    break;
                }
            }
        }
        println!("Task 2 is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // 发送取消信号
    let _ = tx.send(true);

    // 等待任务完成
    let _ = tokio::join!(task1, task2);
    //Task 1 is cancelling...
    // Task 1 is cleaning up
    // Task 2 is cancelling...
    // Task 2 is cleaning up
}

// 单生产者 单消费者
pub async fn oneshot() {
    let (tx, rx) = oneshot::channel();

    let task = tokio::spawn(async move {
        tokio::select! {
            res = rx=>{
                println!("receiver {:?}", res);
            }
            sleep_res = tokio::time::sleep(tokio::time::Duration::from_secs(10))=>{
                println!("sleep {:?}", sleep_res);
            }
        }

        println!("Task is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // 发送取消信号
    let _ = tx.send(());

    // 等待任务完成
    let _ = task.await;
    //receiver Ok(())
    // Task is cleaning up
}
