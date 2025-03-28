# 多futrue 并行执行

[](https://mp.weixin.qq.com/s/GDupF6mu4zATjq_R7a6Ycg)



# join

```
use std::ops::Sub;
use std::time::Duration;
use tokio::join;

async fn test1()->String{
   println!("test1 thread id {:?} name {:?}",
            std::thread::current().id(),
            std::thread::current().name());
    tokio::time::sleep(Duration::from_secs(3)).await;

    String::from("test1")
}

async fn test2()->String{
    println!("test2 thread id {:?} name {:?}",
             std::thread::current().id(),
             std::thread::current().name());
    tokio::time::sleep(Duration::from_secs(2)).await;
    String::from("test2")

}


pub async fn test_more_future(){
    let t = tokio::time::Instant::now();
    let (test1,test2) = join!(test1(),test2());
    println!("test1 {} ,test2 {}",test1,test2);
    println!("end times {:?}",tokio::time::Instant::now().sub(t));
}
```



```
test1 thread id ThreadId(1) name Some("main")
test2 thread id ThreadId(1) name Some("main")
test1 test1 ,test2 test2
end times 3.001817084s
```

可以看到是3s左右 最长时间左右

# try_join

如果其中有失败的话，也会返回失败的`Err`。如果想一有失败就立马返回，不等待其他任务完成，可以使用`try_join!`。

```
async fn async_fn1() -> Result<u32, &'static str> {
    Ok(1)
}

async fn async_fn2() -> Result<u32, &'static str> {
    Err("async_fn2 failed")
}

#[tokio::main]
async fn main() {
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
```



# 3 spawn



上边`join`虽然是让多个异步任务并发执行，但其实际还是在同一个`task`上异步执行，如果想让每个异步任务都在一个新的`task`上**独立**执行，可以用`spawn`。

异步任务`spawn`后会在后台立即开始运行，即便没有对其返回的`JoinHandle`进行`await`

这就有点像多线程里的`spawn`，只不过这里粒度不是线程，是`task`。

```
use std::ops::Sub;
use std::time::Duration;
use futures::future::join_all;
use tokio::join;

/////////////////////////////////// join! /////////////////////////////////////////////
async fn test1()->String{
   println!("test1 thread id {:?} name {:?}",
            std::thread::current().id(),
            std::thread::current().name());
    tokio::time::sleep(Duration::from_secs(3)).await;

    String::from("test1")
}

async fn test2()->String{
    println!("test2 thread id {:?} name {:?}",
             std::thread::current().id(),
             std::thread::current().name());
    tokio::time::sleep(Duration::from_secs(2)).await;
    String::from("test2")

}


pub async fn test_more_future(){
    let t = tokio::time::Instant::now();

    // join！
    // let (test1,test2) = join!(test1(),test2());

    //spawn
    let ops = vec![tokio::spawn(test1()),tokio::spawn(test2())];
    let res = join_all(ops).await;
    println!("{:?}",res);
    println!("end times {:?}",tokio::time::Instant::now().sub(t));
}
```





```
test1 thread id ThreadId(11) name Some("tokio-runtime-worker")
test2 thread id ThreadId(10) name Some("tokio-runtime-worker")
[Ok("test1"), Ok("test2")]
end times 3.003001583s
```

时间是一样的，但是线程的id是不一样的



# 4 select

如果是多个异步分支（`branch`）有一个完成就返回，并取消(`drop`来释放异步资源)其他异步分支的话，可以用`select`

```
#![allow(unused)]
use std::ops::Sub;
use std::time::Duration;
use futures::future::join_all;
use tokio::join;

/////////////////////////////////// join! /////////////////////////////////////////////
async fn test1()->String{
   println!("test1 thread id {:?} name {:?}",
            std::thread::current().id(),
            std::thread::current().name());
    tokio::time::sleep(Duration::from_secs(3)).await;

    String::from("test1")
}

async fn test2()->String{
    println!("test2 thread id {:?} name {:?}",
             std::thread::current().id(),
             std::thread::current().name());
    tokio::time::sleep(Duration::from_secs(2)).await;
    String::from("test2")

}


pub async fn test_more_future(){
    let t = tokio::time::Instant::now();

    // join！
    // let (test1,test2) = join!(test1(),test2());

    //spawn
    // let ops = vec![tokio::spawn(test1()),tokio::spawn(test2())];
    // let res = join_all(ops).await;
    // println!("{:?}",res);


    //select
    let res = tokio::select! {
         test1 = test1()=>{
            println!("res test1 {}",test1);
            test1
        }
        test2 = test2() => {
            println!("res test2 {}",test2);
            test2

        }
    };

    println!("end res {}",res);

    println!("end times {:?}",tokio::time::Instant::now().sub(t));
}
```



```
test1 thread id ThreadId(1) name Some("main")
test2 thread id ThreadId(1) name Some("main")
res test2 test2
end res test2
end times 2.001605417s
```

 看到是最短的时间 ，只要有一个结束 就结束了



# 5 select 顺序执行

```
#![allow(unused)]
use futures::future::join_all;
use std::ops::Sub;
use std::time::Duration;
use tokio::join;


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
    loop {
        let res = tokio::select! {
            biased;
            test1 = test1()=>{
                println!("res test1 {}",test1);
                test1
            }
            test2 = test2() => {
                println!("res test2 {}",test2);
                test2
            }
        };
        println!("end res {}",res);
    }

    println!("end times {:?}", tokio::time::Instant::now().sub(t));
}

```



```
test1 thread id ThreadId(1) name Some("main")
test2 thread id ThreadId(1) name Some("main")
res test2 test2
end res test2
test1 thread id ThreadId(1) name Some("main")
test2 thread id ThreadId(1) name Some("main")
res test2 test2
end res test2
```

可以看到是顺序执行的 但是后面的值会**覆盖前面的** 要注意



# 6 cancel

最后在聊聊分支取消。

当`select`有分支完成时，其他分支会被取消。取消依托于`Drop`。当`future`被`drop`，其也会停止被异步调度。

比如下边代码，当`oneshot::Receiver`被取消而`Drop`时，会向`Sender`发送`close`通知，以便于清理`sender`并中断其执行。



```
#![allow(unused)]
use futures::future::join_all;
use std::ops::Sub;
use std::time::Duration;
use tokio::join;
use tokio::sync::oneshot;

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

```



```
end times 0ns
rx2 completed first with Ok("two")
tx1 closed

```

看到rx2收到后 select 执行了drop tx1被closed了















































