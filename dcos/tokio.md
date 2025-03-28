

# 1 简介

Tokio 是 Rust 编程语言的异步运行时。它提供了编写网络应用程序所需的构建块。它可以灵活地针对各种系统，从具有数十个内核的大型服务器到小型嵌入式设备。

从高层次来看，Tokio 提供了一些主要组件：

- 用于执行异步代码的多线程运行时。
- 标准库的异步版本。
- 庞大的lib生态系统。

优势：

**快速地**

Tokio 速度*很快*，建立在 Rust 编程语言之上，而 Rust 编程语言本身也很快。这是本着 Rust 的精神完成的，其目标是您不应该能够通过手动编写等效代码来提高性能。

Tokio 是*可扩展的*，建立在 async/await 语言功能之上，而 async/await 语言功能本身也是可扩展的。在处理网络时，由于延迟，处理连接的速度会受到限制，因此唯一的扩展方法是同时处理多个连接。借助 async/await 语言功能，增加并发操作的数量变得非常便宜，允许您扩展到大量并发任务。

**可靠的**

Tokio 是使用 Rust 构建的，Rust 是一种使每个人都能够构建可靠且高效的软件的语言。多项研究 发现，大约 70% 的高严重性安全漏洞是内存不安全造成[的](https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/)[。](https://www.chromium.org/Home/chromium-security/memory-safety)使用 Rust 可以消除应用程序中的所有此类错误。

Tokio 还非常注重提供一致的行为，不会出现意外。Tokio 的主要目标是允许用户部署可预测的软件，该软件将日复一日地执行，具有可靠的响应时间，并且不会出现不可预测的延迟峰值。

**简单的**

借助 Rust 的 async/await 功能，编写异步应用程序的复杂性已大大降低。与 Tokio 的实用程序和充满活力的生态系统相结合，编写应用程序变得轻而易举。

Tokio 在有意义时遵循标准库的命名约定。这样可以轻松地将仅使用标准库编写的代码转换为使用 Tokio 编写的代码。凭借 Rust 强大的类型系统，轻松交付正确代码的能力是无与伦比的。

**灵活的**

Tokio 提供了多种运行时变体。从多线程、[工作窃取](https://en.wikipedia.org/wiki/Work_stealing)运行时到轻量级、单线程运行时。每个运行时都带有许多旋钮，允许用户根据自己的需求进行调整。



**不适合场景**

- 通过在多个线程上并行运行 CPU 密集型计算来加速计算。Tokio 专为 IO 密集型应用程序而设计，其中每个单独的任务大部分时间都在等待 IO。如果您的应用程序所做的唯一事情就是并行运行计算，那么您应该使用 [rayon](https://docs.rs/rayon/)。也就是说，如果您需要两者兼而有之，仍然可以“混合搭配”。请参阅[这篇博文了解实际示例](https://ryhl.io/blog/async-what-is-blocking/#the-rayon-crate)。
- 读取大量文件。虽然 Tokio 似乎对于只需要读取大量文件的项目很有用，但与普通线程池相比，Tokio 在这里没有提供任何优势。这是因为操作系统一般不提供异步文件API。
- 发送单个 Web 请求。Tokio 为您提供优势的地方是当您需要同时做很多事情时。如果您需要使用用于异步 Rust 的库（例如[reqwest ）](https://docs.rs/reqwest/)，但不需要一次执行很多操作，那么您应该更喜欢该库的阻塞版本，因为它会让您的项目更简单。当然，使用 Tokio 仍然可以工作，但与阻塞 API 相比并没有真正的优势。如果该库不提供阻塞 API，请参阅[有关使用同步代码桥接的章节](https://tokio.rs/tokio/topics/bridging)。



# 2 Tokio time

## 2.1 sleep

```
use tokio::time::sleep;
use std::time::Duration;
async fn test_sleep(){
    println!("测试sleep");
    sleep(Duration::from_secs(1)).await;
    println!("测试sleep end");
}

测试sleep
测试sleep end
```



## 2.2 timeout

```
use tokio::time::timeout;
use tokio::time::Duration as TD;
async fn test_timeout(){
    let res =  timeout(TD::from_secs(1),long_future()).await;
    if let Err(err) = res {
        println!("operation timeout {:?}", err);
    }
}

async  fn long_future(){
    sleep(Duration::from_secs(2)).await;

}
operation timeout Elapsed(())
```



## 2.3 interval_at

初始化个间隔 后面根据第二个时间间隔执行

```
use tokio::time::interval_at;
use tokio::time::Instant;
async fn test_interval_at(){
    let start = Instant::now()+TD::from_secs(1);
    let mut interval = interval_at(start,TD::from_secs(2));
    println!("test interval_at {:?}" ,Local::now());

    interval.tick().await;
    println!("test interval_at {:?}" ,Local::now());
    interval.tick().await;
    println!("test interval_at {:?}" ,Local::now());

    interval.tick().await;
    println!("test interval_at {:?}" ,Local::now());
}
test interval_at 2024-07-03T14:25:12.425898+08:00
test interval_at 2024-07-03T14:25:13.427952+08:00
test interval_at 2024-07-03T14:25:15.427784+08:00
test interval_at 2024-07-03T14:25:17.428221+08:00


```



## 2.4 sleep_until

到某个时间点执行

```
use tokio::time::sleep_until;
async fn test_sleep_until(){
    println!("test sleep_until {:?}" ,Local::now());
    sleep_until(Instant::now()+TD::from_secs(2)).await;
    println!("test sleep_until {:?}" ,Local::now());
}

test sleep_until 2024-07-03T14:30:06.766858+08:00
test sleep_until 2024-07-03T14:30:08.768441+08:00
```



## 2.5  timeout_at

在某个时间点之前执行完 

```
use tokio::time::timeout_at;
async fn test_timeout_at(){
    println!("test sleep_until {:?}" ,Local::now());
   if let Err(err) = timeout_at(Instant::now()+TD::from_secs(2),test_timeout_at_fn()).await{
       println!("{}",err)
   }
    println!("test sleep_until {:?}" ,Local::now());
}

async fn test_timeout_at_fn(){
    sleep(Duration::from_secs(3)).await;

}

test sleep_until 2024-07-03T14:38:36.576081+08:00
deadline has elapsed
test sleep_until 2024-07-03T14:38:38.578633+08:00
```

