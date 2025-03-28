// // https://mp.weixin.qq.com/s/M1na81d4lx47ksfiPq2IgA
// //https://cloud.tencent.com/developer/article/2390776
//
// use console_subscriber;
// use std::{sync::Arc, time::Duration};
// use tokio::{sync::Semaphore, task, time::sleep};
//
// pub async fn test_tokio_console() {
//     // 注意. 初始化tracing收集
//     console_subscriber::init();
//     // 线程1的令牌桶1初始一个令牌，可以先打印1
//     let semaphore = Arc::new(Semaphore::new(1));
//     let cnt = 3;
//     let semaphore2 = semaphore.clone();
//     // 线程2的令牌桶2初始没有令牌，直到1打印后增加令牌
//     let semaphore_wait = Arc::new(Semaphore::new(0));
//     let semaphore_wait2 = semaphore_wait.clone();
//
//     // 注意. 使用task::Builder来增加task名字，否则等同tokio::spawn
//     let t1 = task::Builder::default()
//         .name("t1")
//         .spawn(async move {
//             for i in 0..cnt {
//                 let permit = semaphore.acquire().await.unwrap();
//                 print!("1 ");
//                 // 注意. 增加等待时间，便于观测
//                 sleep(Duration::from_secs(i)).await;
//                 // 消耗令牌，不放回令牌桶1
//                 permit.forget();
//                 // 令牌桶2增加令牌，可以打印2
//                 semaphore_wait2.add_permits(1);
//             }
//         })
//         .unwrap();
//
//     let t2 = task::Builder::default()
//         .name("t2")
//         .spawn(async move {
//             for i in 0..cnt {
//                 let permit = semaphore_wait.acquire().await.unwrap();
//                 print!("2 ");
//                 // 注意. 增加等待时间，便于观测
//                 sleep(Duration::from_secs(i)).await;
//                 // 消耗令牌，不放回令牌桶2
//                 permit.forget();
//                 // 令牌桶1增加令牌，可以打印1
//                 semaphore2.add_permits(1);
//             }
//         })
//         .unwrap();
//
//     let _ = tokio::try_join!(t1, t2);
//     // println!("{:#?}",s.0);
//     // println!("{:#?}",s.1)
// }
