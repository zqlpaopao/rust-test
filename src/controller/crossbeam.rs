#![allow(unused)]
use async_std::channel;
use std::sync::Arc;

///  https://mp.weixin.qq.com/s/rtjhGX75rURGJV7E5e6C2g
pub async fn test_crossbeam() {
    //普通模式
    // basic_channel().await;

    //零拷贝模式
    zero_copy().await;
}

async fn zero_copys() {
    let (sender, receiver) = channel::unbounded();
    let big_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    //启动生产者
    for i in 0..10 {
        let sender = sender.clone();
        println!("Sending data {}", i);
        tokio::spawn(async move {
            sender.send(i).await.unwrap();
        });
    }

    for i in 0..3 {
        let receiver = receiver.clone();
        tokio::spawn(async move {
            while let Ok(received) = receiver.recv().await {
                println!("消费者：{}, 获取消息为: {:?}", i, received);
            }
        });
    }
    // Sending data 0
    // Sending data 1
    // Sending data 2
    // Sending data 3
    // Sending data 4
    // Sending data 5
    // Sending data 6
    // Sending data 7
    // Sending data 8
    // Sending data 9
    // 消费者：1, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：1, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：1, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：2, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}

/********************************** 零拷贝的神奇魔法 ****************************/
async fn zero_copy() {
    let (sender, receiver) = channel::unbounded();
    let big_data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    //启动生产者
    for i in 0..10 {
        let data = big_data.clone();
        let sender = sender.clone();
        println!("Sending data {}", i);
        tokio::spawn(async move {
            sender.send(data).await.unwrap();
        });
    }

    for i in 0..3 {
        let receiver = receiver.clone();
        tokio::spawn(async move {
            while let Ok(received) = receiver.recv().await {
                println!("消费者：{}, 获取消息为: {:?}", i, received);
            }
        });
    }
    // Sending data 0
    // Sending data 1
    // Sending data 2
    // Sending data 3
    // Sending data 4
    // Sending data 5
    // Sending data 6
    // Sending data 7
    // Sending data 8
    // Sending data 9
    // 消费者：1, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：1, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：0, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：1, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // 消费者：2, 获取消息为: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}

/********************************** 普通模式 ****************************/

// 普通模式
async fn basic_channel() {
    //无界 channel
    let (sender, receiver) = channel::unbounded();

    //// 创建一个有界 channel，最多存 2 条消息
    // let(sender， receiver)= channel：：bounded(2)；

    //启动生产者
    tokio::spawn(async move {
        for i in 0..10 {
            sender.send(i).await.unwrap();
        }
    });

    //消费者接受消息
    for i in 0..5 {
        let receiver = receiver.clone();
        tokio::spawn(async move {
            while let Ok(received) = receiver.recv().await {
                println!("消费者：{}, 获取消息为: {}", i, received);
            }
        });
    }
    // 消费者：1, 获取消息为: 0
    // 消费者：1, 获取消息为: 1
    // 消费者：1, 获取消息为: 2
    // 消费者：1, 获取消息为: 3
    // 消费者：1, 获取消息为: 4
    // 消费者：1, 获取消息为: 5
    // 消费者：1, 获取消息为: 7
    // 消费者：1, 获取消息为: 8
    // 消费者：1, 获取消息为: 9
    // 消费者：0, 获取消息为: 6
}
