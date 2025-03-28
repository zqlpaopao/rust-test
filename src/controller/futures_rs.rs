#![allow(unused)]

use future::Future;
use futures::future::{self, join_all};
use futures::stream::{self, Stream};
use reqwest::Error;
use std::cmp;
use std::fmt::Debug;
use std::pin::Pin;
use std::task::{Context, Poll};
// use tokio::sync::oneshot;
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::join;
use tokio::time::{sleep, Duration};
pub async fn test_futures_rs() {
    //
    // download().await

    //自定义实现future
    // make_future().await;

    //测试pin
    // test_pin_unpin().await;

    //测试 oneshot
    // test_oneshot().await;

    //测试mpsc
    // test_mpsc();

    //测试tokio mpsc
    // tokio_test().await;

    //测试线程池
    test_futures_thread_pool().await;
}
/******************************************************** 线程池 *********************************/
use futures::executor::ThreadPool;
use futures::future::FutureExt; // 提供 `.boxed()` 方法
use futures::future::FutureObj; // 提供 `.boxed()` 方法

// 假设你有多个异步任务需要执行
async fn async_task1() {
    // 异步代码
    println!(
        "async_task1 thread id - {:?}, name - {:?}",
        std::thread::current().id(),
        std::thread::current().name().unwrap_or("Unnamed")
    );
}

async fn async_task2() {
    // 异步代码
    println!(
        "async_task2 thread id - {:?}, name - {:?}",
        std::thread::current().id(),
        std::thread::current().name().unwrap_or("Unnamed")
    );
}

async fn test_futures_thread_pool() {
    let pool = ThreadPool::new().expect("Failed to create thread pool");

    // new 实现 实现ThreadPoolBuilder
    // pool_size: cmp::max(1, num_cpus::get()), //默认为cpu最大数

    // 设置池中线程的堆栈大小（以字节为单位）。 默认情况下，工作线程使用 Rust 的标准堆栈大小。
    // stack_size: 0,

    //设置未来 ThreadPool 的线程名称前缀。
    // 线程名称前缀用于生成线程名称。例如，如果前缀为 my-pool-，则池中的线程将获得类似my-pool-1等的名称。
    // 默认情况下，工作线程被分配了 Rust 的标准线程名称。
    // name_prefix: None,

    //f在每个工作线程启动之后、但在其上运行任何任务之前立即执行闭包。
    // 此钩子用于记账和监控。 闭包f将在builder被删除且池中的所有工作线程都执行完后被删除。
    // 提供的闭包将接收与其正在运行的工作线程相对应的索引。
    // after_start: None,

    // f在关闭每个工作线程之前执行闭包。
    // 此钩子用于记账和监控。 闭包将在被删除且池中的所有线程都执行完后f被删除。builder
    // 提供的闭包将接收与其正在运行的工作线程相对应的索引。
    // before_stop: None,

    /// builder
    let pool = ThreadPool::builder()
        .pool_size(10) //线程吃大小
        // .stack_size() //堆栈大小
        .name_prefix("test")
        .after_start(|res| {
            //执行前执行的
            println!("after_start {}", res);
        })
        .before_stop(|res| {
            println!("before_stop {}", res);
        })
        .create()
        .expect("创建失败");

    // 使用 `spawn` 方法在线程池上分别运行这些任务
    let task1 = async_task1().boxed();
    let task2 = async_task2().boxed();

    //产生一个任务，轮询给定的未来并输出()完成情况。
    pool.spawn_ok(task2);
    pool.spawn_ok(task1);

    //没啥区别 不同之处在于它保证始终成功。
    // pool.spawn_obj_ok(FutureObj::from(task1));
    // pool.spawn_obj_ok(FutureObj::from(task2));
}

/******************************************************** mpsc *********************************/
use futures::channel::mpsc;
use futures::SinkExt;
use futures::StreamExt;

async fn send_numbers(mut sender: mpsc::Sender<i32>) {
    for i in 0..5 {
        sender.send(i).await.unwrap();
    }
    drop(sender);
}

async fn receive_numbers(mut receiver: mpsc::Receiver<i32>) {
    while let Some(number) = receiver.next().await {
        println!("Received: {}", number);
    }
    println!("Received: end");
}

fn test_mpsc() {
    let (sender, receiver) = mpsc::channel::<i32>(10);

    let sender_task = send_numbers(sender);
    let receiver_task = receive_numbers(receiver);

    block_on(futures::future::join(sender_task, receiver_task));
}

use tokio::runtime::Runtime;
/// tokio
use tokio::sync::mpsc as tm;

async fn send_numbers_tokio(mut sender: tm::Sender<i32>) {
    for i in 0..5 {
        if sender.send(i).await.is_err() {
            println!("Receiver dropped");
            return;
        }
    }
}

async fn receive_numbers_tokio(mut receiver: tm::Receiver<i32>) {
    while let Some(number) = receiver.recv().await {
        println!("Received: {}", number);
    }
    println!("Channel closed, no more messages.");
}

async fn tokio_test() {
    //不可以在tokio中在开启运行时 可使用futures
    // let rt = Runtime::new().unwrap();
    // rt.block_on(async {
    //     let (sender, receiver) = tm::channel::<i32>(10);
    //
    //     tokio::spawn(send_numbers_tokio(sender));
    //     tokio::spawn(receive_numbers_tokio(receiver));
    // });

    let (sender, receiver) = tm::channel::<i32>(10);
    let sd = send_numbers_tokio(sender);
    let srs = receive_numbers_tokio(receiver);

    let rs = join!(sd, srs);

    // tokio::spawn(send_numbers_tokio(sender));
    // tokio::spawn(receive_numbers_tokio(receiver));
}

/******************************************************** oneshot *********************************/
async fn async_function(sender: oneshot::Sender<i32>) {
    // 模拟异步操作
    sender.send(42).unwrap(); // 发送值，如果接收端已经被丢弃，这里会产生错误
}
async fn test_oneshot() {
    //单个的 没有缓存空间
    let (sender, receiver) = oneshot::channel();

    let task = async_function(sender);

    let receiver_task = receiver.map(|res| {
        println!("Received: {:?}", res);
        //42
    });
    block_on(futures::future::join(task, receiver_task));
}

/******************************************************** Pin Unpin *********************************/
struct Unmovable {
    data: String,
}

impl Unmovable {
    fn new(data: &str) -> Self {
        Unmovable {
            data: data.to_string(),
        }
    }
}

impl Unpin for Unmovable {}

fn use_unmovable() -> Pin<Box<Unmovable>> {
    let unmovable = Unmovable::new("hello word");
    Box::pin(unmovable)
}

async fn test_pin_unpin() {
    let pined = use_unmovable();
    println!("Unmovable data : {}", pined.data);
}

/******************************************************** 多任务下载 *********************************/
///例子 下载
async fn download_file(url: &str) -> Result<String, Error> {
    println!("{url}");
    let mut response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn download() {
    let urls = vec![
        "https://www.rust-lang.org",
        "https://www.mozilla.org",
        "https://www.github.com",
    ];

    let futures: Vec<_> = urls.iter().map(|&url| download_file(url)).collect();

    let results = join_all(futures).await;
    for result in results {
        match result {
            Ok(_) => println!("Downloaded content: "),
            Err(e) => eprintln!("Error downloading file: {}", e),
        }
    }
}

//https://www.rust-lang.org
// https://www.mozilla.org
// https://www.github.com
// Downloaded content:
// Downloaded content:
// Downloaded content:

/******************************************************** 自定义实现 future *********************************/

struct CustomFuture {
    state: i32,
}

impl Future for CustomFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("111");
        if self.state < 5 {
            println!("222");

            // 使用tokio的sleep来模拟异步等待。
            // 注意：这里仅为示例，实际上你不能在poll方法内部启动一个新的异步任务。
            // 应该在Future的结构体中持有这个sleep future，并在poll中轮询它。
            // tokio::spawn(async {
            //     sleep(Duration::from_secs(1)).await;
            // });

            self.state += 1;
            cx.waker().clone().wake();
            Poll::Pending
        } else {
            println!("333");

            Poll::Ready(self.state)
        }
    }
}

async fn make_future() {
    let custom_future = CustomFuture { state: 0 };
    let result = custom_future.await;
    println!("CustomFuture result :{}", result);
}
