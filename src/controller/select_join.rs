#![allow(unused)]
use futures::future::join_all;
use std::ops::Sub;
use std::time::Duration;
use tokio::join;
use tokio::sync::oneshot;

/// https://juejin.cn/post/7217669171579420732
///
/// complete 分支：当所有的 Future 和 Stream 完成后才会被执行，它往往配合loop使用，loop用于循环完成所有的 Future
// default分支：若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行
//
//
//
//

/////////////////////////////////// try join! /////////////////////////////////////////////
async fn async_fn1() -> Result<u32, &'static str> {
    Ok(1)
}

async fn async_fn2() -> Result<u32, &'static str> {
    Err("async_fn2 failed")
}

async fn test_try_join() {
    let res = tokio::try_join!(async_fn1(), async_fn2());

    match res {
        Ok((first, second)) => {
            println!("first = {}, second = {}", first, second);
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
}

/////////////////////////////////// join! /////////////////////////////////////////////
async fn test1() -> String {
    println!(
        "test1 thread id {:?} name {:?}",
        std::thread::current().id(),
        std::thread::current().name()
    );
    tokio::time::sleep(Duration::from_secs(3)).await;

    String::from("test1")
}

async fn test2() -> String {
    println!(
        "test2 thread id {:?} name {:?}",
        std::thread::current().id(),
        std::thread::current().name()
    );
    tokio::time::sleep(Duration::from_secs(2)).await;
    String::from("test2")
}

pub async fn test_more_future() {
    let t = tokio::time::Instant::now();

    // join！
    // let (test1,test2) = join!(test1(),test2());

    //spawn
    // let ops = vec![tokio::spawn(test1()),tokio::spawn(test2())];
    // let res = join_all(ops).await;
    // println!("{:?}",res);

    //select
    // let res = tokio::select! {
    //      test1 = test1()=>{
    //         println!("res test1 {}",test1);
    //         test1
    //     }
    //     test2 = test2() => {
    //         println!("res test2 {}",test2);
    //         test2
    //
    //     }
    // };

    //select 顺序执行
    // loop {
    //     let res = tokio::select! {
    //         biased;
    //         test1 = test1()=>{
    //             println!("res test1 {}",test1);
    //             test1
    //         }
    //         test2 = test2() => {
    //             println!("res test2 {}",test2);
    //             test2
    //         }
    //     };
    //     println!("end res {}",res);
    // }

    println!("end times {:?}", tokio::time::Instant::now().sub(t));

    //cancel
    let (mut tx1, rx1) = oneshot::channel::<u32>();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        tokio::select! {
            _ = tx1.closed() => {
                // `val = rx1` is canceled
                println!("tx1 closed");
            }
        }
    });
    tokio::spawn(async {
        let _ = tx2.send("two");
    });
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);

        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}
