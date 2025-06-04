



// https://mp.weixin.qq.com/s/rh4w-1S8nezsROvNtRckBQ

//

use std::sync::Arc;
use tokio_mpmc::Queue;
use tokio::time::{self, Duration};

pub async fn test_tokio_mpmc() {
    // 创建一个容量为 100 的队列
    let queue = Queue::new(100);

    // 克隆队列，用于多个生产者和消费者
    let producer_queue = queue.clone();
    let consumer_queue = queue.clone();


    // 启动一个消费者任务
    let consumer_queue = consumer_queue.clone();
   _ =  tokio::spawn(async move {
        for j in 0..10 {
            let consumer_queue = consumer_queue.clone();
            let consumer_handle = tokio::spawn(async move {
                loop {
                    match consumer_queue.receive().await {
                        Ok(Some(msg)) => {
                            println!("Consumer_{} received: {}", j,msg);
                        }
                        Ok(None) => {
                            // 队列已关闭且为空
                            println!("Consumer finished: Queue closed and empty.");
                            break;
                        }
                        Err(e) => {
                            eprintln!("Consumer receive failed: {}", e);
                            break;
                        }
                    }
                }
            });
            // 等待消费者任务完成
            // consumer_handle.await.unwrap();
        }
    }).await;


    // 启动一个生产者任务
    // for j in 0..num_cpus::get() {
        let producer_queue = producer_queue.clone();
        let producer_handle = tokio::spawn(async move {
            for i in 0..10000 {
                let msg = format!("message {}", i);
                println!("Producer sending: {}", msg);
                if let Err(e) = producer_queue.send(msg).await {
                    eprintln!("Producer send failed: {}", e);
                    break;
                }
                // time::sleep(Duration::from_millis(10)).await;
            }
            println!("Producer finished.");
        });
        // 等待生产者和消费者任务完成
        producer_handle.await.unwrap();
    // }







    // 关闭队列，通知消费者不再有新的消息
    println!("Closing queue...");
    // queue.close().await;


    time::sleep(Duration::from_secs(50)).await;


    println!("Example finished.");
}