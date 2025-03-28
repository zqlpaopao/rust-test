# 1、添加依赖

要在您的 Rust 项目中使用futures-rs，请在Cargo.toml文件中添加以下依赖项：

```
[dependencies]
futures = "0.3"
tokio = { version = "1", features = ["full"] }
```

这里，我们还包括了tokio，这是一个流行的异步运行时，与futures-rs配合良好。



# 2、自定义实现future

```
use tokio::time::{sleep, Duration};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CustomFuture {
    state: i32,
}

impl Future for CustomFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("111");
        if self.state < 5 {
            println!("222");
            self.state += 1;

            // 使用tokio的sleep来模拟异步等待。
            // 注意：这里仅为示例，实际上你不能在poll方法内部启动一个新的异步任务。
            // 应该在Future的结构体中持有这个sleep future，并在poll中轮询它。
            tokio::spawn(async {
                sleep(Duration::from_secs(1)).await;
            });

            // 通知执行器未来某时再次轮询此Future
            cx.waker().clone().wake();
            Poll::Pending
        } else {
            println!("333");
            Poll::Ready(self.state)
        }
    }
}

#[tokio::main]
async fn main() {
    let custom_future = CustomFuture { state: 0 };
    let result = custom_future.await;
    println!("CustomFuture result: {}", result);
}


111
222
111
222
111
222
111
222
111
222
111
333
CustomFuture result :5
```

切记在某个时刻进行任务的唤醒执行

```
 cx.waker().clone().wake();
```



# 3、pin和unpin

在 Rust 中，异步编程要求对象在某些情况下不能移动，为此引入了`Pin`和`Unpin`。

- **Pin**: 确保对象在内存中的位置不会改变。
- **Unpin**: 标记对象可以安全地移动。

了解如何正确使用`Pin`和`Unpin`对编写安全的异步代码至关重要。



# 4、channel

- [oneshot](https://docs.rs/futures/0.3.30/futures/channel/oneshot/index.html)，在异步任务之间发送单条消息的通道。
- [mpsc](https://docs.rs/futures/0.3.30/futures/channel/mpsc/index.html)，用于跨异步任务发送值的多生产者、单消费者队列。

## 4.1 oneshot

```
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::FutureExt; // 为Future提供扩展方法，比如`.map`

async fn async_function(sender: oneshot::Sender<i32>) {
    // 模拟异步操作
    sender.send(42).unwrap(); // 发送值，如果接收端已经被丢弃，这里会产生错误
}

fn main() {
    let (sender, receiver) = oneshot::channel();

    // 创建并运行异步任务
    let task = async_function(sender);

    // 将接收操作转换为Future，并处理结果
    let receive_task = receiver.map(|res| {
        println!("Received: {:?}", res);
    });

    // 使用block_on同时运行两个任务
    block_on(futures::future::join(task, receive_task));
}


Received: Ok(42)
```



- 如果`Sender`在发送值之前被丢弃（比如因为所在的异步任务被取消），`Receiver`将会接收到一个错误。
- 如果`Receiver`在值被发送之前被丢弃，尝试发送值的`Sender.send`调用会返回一个`Err`，表明没有值被发送。





## 4.2 tokio oneshot

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    // 创建一个oneshot通道
    let (tx, rx) = oneshot::channel();

    // 发送值的任务
    tokio::spawn(async move {
        if tx.send(42).is_err() {
            println!("接收端已经被丢弃");
        }
    });

    // 接收值的任务
    match rx.await {
        Ok(value) => println!("接收到值: {}", value),
        Err(_) => println!("发送端已经被丢弃"),
    }
}

```



## 4.3 区别

### 区别

尽管`futures`和`tokio`的`oneshot`通道在功能上相似，但它们在以下方面存在差异：

1. **依赖性**：`futures`的`oneshot`不依赖于特定的异步运行时，而`tokio`的`oneshot`需要在`tokio`运行时中使用。
2. **集成度**：`tokio`的`oneshot`与`tokio`的其他功能（如任务调度）更加集成。
3. **使用场景**：如果你已经在使用`tokio`作为你的异步运行时，那么使用`tokio`的`oneshot`可能更方便，因为它与`tokio`的API设计保持一致。如果你需要一个与运行时无关的通道，或者你正在使用不同的异步执行器，那么`futures`的`oneshot`可能更适合。

# map 、and_then and 区别



在讨论`map`和`and_then`的区别之前，重要的是要理解它们都是处理Rust中`Result`和`Option`类型的方法，以及用于操作异步编程中的`Future`。这些方法允许你在保持代码简洁性的同时，有效地链式调用操作。

### 对于`Result`和`Option`

#### `map`

- **用途**：`map`用于将`Result<T, E>`的`Ok(T)`或`Option<T>`的`Some(T)`值转换成另一个值`U`，得到`Result<U, E>`或`Option<U>`。如果是`Err(E)`或`None`，则不做任何操作。
- **特点**：它仅用于转换值，不能用于错误处理或执行返回`Result`或`Option`的函数。

```rust
let some_number = Some(5);
let some_string = some_number.map(|i| i.to_string());
assert_eq!(Some("5".to_string()), some_string);

let x: Result<i32, &str> = Ok(5);
let y: Result<String, &str> = x.map(|i| i.to_string());
assert_eq!(Ok("5".to_string()), y);
```

#### `and_then`

- **用途**：`and_then`用于链式调用返回`Result`或`Option`的函数。对于`Result<T, E>`，它仅在`Ok(T)`时调用，对于`Option<T>`，它仅在`Some(T)`时调用。
- **特点**：它可以用于执行更多的可能失败的操作，并且可以用于错误处理。

```rust
let x: Result<i32, &str> = Ok(5);
let y = x.and_then(|i| Ok(i * 2));
assert_eq!(Ok(10), y);

let some_number = Some(7);
let none: Option<i32> = None;
let some_string = some_number.and_then(|i| Some(i.to_string()));
let none_string = none.and_then(|i| Some(i.to_string()));
assert_eq!(Some("7".to_string()), some_string);
assert_eq!(None, none_string);
```

### 对于`Future`

当处理异步操作时，`map`和`and_then`的概念与`Result`和`Option`类似，但它们操作的是`Future`。

#### `map`

- **用途**：在`Future`成功完成时，使用`map`来转换成功的值。它不改变失败的情况。
- **场景**：当你需要将`Future`的成功输出转换为另一个值而不需要执行另一个异步操作时。

#### `and_then`

- **用途**：与`map`不同，`and_then`允许在当前`Future`成功后执行另一个异步操作。它期望闭包返回另一个`Future`。
- **场景**：当你需要根据前一个`Future`的结果来决定下一步的异步操作时。

### 总结

- 使用`map`进行简单的同步转换，不改变操作的成功或失败状态。
- 使用`and_then`进行链式异步操作，特别是当后续步骤依赖于前一步骤的结果时。

选择`map`或`and_then`取决于你的具体需求：是否需要进行更多的异步操作，或者仅仅是转换值。

```
use futures::future::FutureExt; // 引入map方法

let future = async { 5 };
let mapped_future = future.map(|x| x + 3);
// 此时，mapped_future将在完成时结果为8


use futures::future::FutureExt; // 引入and_then方法

let future = async { Ok::<_, ()>(5) };
let and_then_future = future.and_then(|x| async move {
    Ok(x + 3)
});
// 此时，and_then_future在成功完成时的结果为Ok(8)

```



# 5、mpsc

```
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
    while let Some(number) =  receiver.next().await {
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
```





# 6、tokio mpsc

```
use tokio::sync::mpsc;
use tokio::runtime::Runtime;

async fn send_numbers(mut sender: mpsc::Sender<i32>) {
    for i in 0..5 {
        if sender.send(i).await.is_err() {
            println!("Receiver dropped");
            return;
        }
    }
}

async fn receive_numbers(mut receiver: mpsc::Receiver<i32>) {
    while let Some(number) = receiver.recv().await {
        println!("Received: {}", number);
    }
    println!("Channel closed, no more messages.");
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (sender, receiver) = mpsc::channel::<i32>(10);

        tokio::spawn(send_numbers(sender));
        tokio::spawn(receive_numbers(receiver));
    });
}

```

在这个例子中：

- 使用`tokio::sync::mpsc`模块中的`channel`函数创建了一个 MPSC 通道。
- `send_numbers`异步函数接受一个可变的发送者(`Sender`)，并异步发送一系列数字。如果接收者被丢弃，发送操作会返回一个错误。
- `receive_numbers`异步函数接受一个接收者(`Receiver`)，并在一个循环中等待并打印接收到的每个数字。当通道被关闭，循环结束。
- 在`main`函数中，我们使用`tokio::runtime::Runtime`来运行异步代码块，并使用`tokio::spawn`来并发运行发送和接收任务。

注意，与`futures::channel::mpsc`不同，Tokio 的`recv`方法直接返回`Option<T>`，而不是`Result<Option<T>, E>`。这意味着当通道被关闭且没有更多消息可接收时，它将返回`None`。





# 7、无界通道 unbound

```
use futures::channel::mpsc::unbounded;
use futures::executor::block_on;
use futures::stream::StreamExt; // 提供 `.next()` 方法

async fn send_numbers(mut sender: futures::channel::mpsc::UnboundedSender<i32>) {
    for i in 0..5 {
        // 发送消息，如果接收者已经被丢弃，则停止发送。
        if sender.unbounded_send(i).is_err() {
            println!("Receiver dropped");
            return;
        }
    }
}

async fn receive_numbers(mut receiver: futures::channel::mpsc::UnboundedReceiver<i32>) {
    while let Some(number) = receiver.next().await {
        println!("Received: {}", number);
    }
}

fn main() {
    let (sender, receiver) = unbounded(); // 创建无界通道

    // 使用 futures 的 block_on 来运行异步代码块
    block_on(async {
        let send_task = send_numbers(sender);
        let receive_task = receive_numbers(receiver);

        // 并发运行发送和接收任务
        futures::join!(send_task, receive_task);
    });
}

```

在这个例子中：

- 使用 `futures::channel::mpsc::unbounded` 函数创建了一个无界通道。
- `send_numbers` 函数是一个异步函数，它尝试发送一系列数字到通道中。它使用 `unbounded_send` 来发送消息，这个方法不会因为缓冲区大小限制而挂起或失败，但如果接收者已经被丢弃，发送操作会失败。
- `receive_numbers` 函数是另一个异步函数，它接收并打印出通道中的所有消息。它使用 `next` 方法等待新的消息，这个方法来自 `StreamExt` trait，它将 `UnboundedReceiver` 视为一个异步流（`Stream`）。
- 在 `main` 函数中，使用 `futures::executor::block_on` 来运行这些异步任务。

使用无界通道时，虽然不需要担心发送操作因为缓冲区已满而挂起，但是应当注意控制发送速度和频率，避免因为过度发送而消耗过多内存资源。



# 8、线程池

简单使用

```
use futures::executor::ThreadPool;
use futures::future::FutureExt; // 提供 `.boxed()` 方法


// 假设你有多个异步任务需要执行
async fn async_task1() {
    // 异步代码
    println!("async_task1");
}

async fn async_task2() {
    // 异步代码
    println!("async_task2");

}

async  fn test_futures_thread_pool(){
    let pool = ThreadPool::new().expect("Failed to create thread pool");
    // 使用 `spawn` 方法在线程池上分别运行这些任务
    let task1 = async_task1().boxed();
    let task2 = async_task2().boxed();
    pool.spawn_ok(task2);
    pool.spawn_ok(task1);

}

async_task2
async_task1
```



## 8.1 初始化线程池

**new**

```
let pool = ThreadPool::new()
.expect("Failed to create thread pool");

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
```



**builder**

```
use futures::executor::ThreadPool;
use futures::future::FutureExt; // 提供 `.boxed()` 方法


// 假设你有多个异步任务需要执行
async fn async_task1() {
    // 异步代码
    println!("async_task1 thread id - {:?}, name - {:?}",
             std::thread::current().id(),
             std::thread::current().name().unwrap_or("Unnamed"));
}

async fn async_task2() {
    // 异步代码
    println!("async_task2 thread id - {:?}, name - {:?}",
             std::thread::current().id(),
             std::thread::current().name().unwrap_or("Unnamed"));
}

async  fn test_futures_thread_pool(){
    let pool = ThreadPool::new()
        .expect("Failed to create thread pool");

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
        .after_start(|res| {  //执行前执行的
            println!("after_start {}",res);
        })
        .before_stop(|res| {
            println!("before_stop {}",res);

        })
        .create().expect("创建失败");




    // 使用 `spawn` 方法在线程池上分别运行这些任务
    let task1 = async_task1().boxed();
    let task2 = async_task2().boxed();
    pool.spawn_ok(task2);
    pool.spawn_ok(task1);

}

after_start 0
after_start 1
after_start 2
after_start 3
after_start 4
after_start 5
after_start 6
after_start 7
after_start 8
after_start 9
async_task2 thread id - ThreadId(22), name - "test0"
async_task1 thread id - ThreadId(31), name - "test9"
before_stop 1
before_stop 3
before_stop 4
before_stop 5
before_stop 6
before_stop 7
before_stop 8
before_stop 2
before_stop 0
before_stop 9
```



## 8.2 获取异步返回值

```
use futures::channel::oneshot;
use futures::executor::{block_on, ThreadPool};
use futures::future::FutureExt;

async fn async_task1() {
    // 异步代码
    println!("async_task1 thread id - {:?}, name - {:?}",
             std::thread::current().id(),
             std::thread::current().name().unwrap_or("Unnamed"));
}

async fn async_task2() -> String {
    // 异步代码
    println!("async_task2 thread id - {:?}, name - {:?}",
             std::thread::current().id(),
             std::thread::current().name().unwrap_or("Unnamed"));
    "async_task2".to_string()
}

fn main() {
    let pool = ThreadPool::builder()
        .pool_size(10)
        .name_prefix("test")
        .after_start(|index| {
            println!("after_start {}", index);
        })
        .before_stop(|index| {
            println!("before_stop {}", index);
        })
        .create().expect("创建失败");

    // 创建一个 oneshot 通道
    let (tx, rx) = oneshot::channel();

    // Clone `tx` so that we can move it into the `async_task2`.
    let task2 = async {
        let result = async_task2().await;
        tx.send(result).expect("Failed to send the result");
    }.boxed();

    // 现在，我们在线程池上执行 `async_task2` 并等待它的结果
    pool.spawn_ok(task2);

    // 同样地，我们在线程池上执行 `async_task1`
    pool.spawn_ok(async_task1().boxed());

    // 在主线程中等待 `async_task2` 的结果
    let task2_result = block_on(async {
        rx.await.expect("Failed to receive the result")
    });

    println!("Received task2 result: {}", task2_result);
}

```



# 9、单线程

除了线程池之外，还可以通过执行器在单个线程内完全运行任务（及其生成的任务） [`LocalPool`](https://docs.rs/futures/0.3.30/futures/executor/struct.LocalPool.html)。除了减少同步成本外，此执行器还可以`Send`通过 生成非任务[`spawn_local_obj`](https://docs.rs/futures/0.3/futures/task/trait.LocalSpawn.html#tymethod.spawn_local_obj)。 [`LocalPool`](https://docs.rs/futures/0.3.30/futures/executor/struct.LocalPool.html)最适合运行 I/O 密集型任务，这些任务在 I/O 操作之间执行的工作相对较少。

还有一个便捷函数，[`block_on`](https://docs.rs/futures/0.3.30/futures/executor/fn.block_on.html)用于在当前线程上简单地运行未来直至完成。

`LocalPool` 是 `futures` 库提供的一个非常基础的单线程执行器，它可以用来执行 `Future` 对象，直到完成。`LocalPool` 在某些场景下非常有用，比如在测试中或者在不需要多线程并发的小型应用程序中。它的主要优点是简单和不需要多线程的同步。

下面是一个简单的例子，演示了如何使用 `LocalPool` 来执行异步任务：

```rust
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;

async fn async_task() -> String {
    // 模拟异步操作
    "result of async task".to_string()
}

fn main() {
    // 创建一个新的 LocalPool
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    // 创建一个 future
    let future = async_task();

    // 将 future 提交给 LocalPool 来执行
    spawner.spawn_local(future).expect("Failed to spawn task");

    // 运行 LocalPool 直到所有的任务都执行完毕
    pool.run();
}
```

在这个例子中，我们首先创建了一个 `LocalPool` 实例和对应的 `spawner`。`spawner` 提供了 `spawn_local` 方法，允许我们将 `Future` 提交给 `LocalPool`。

然后，我们定义了一个简单的异步任务 `async_task`，它返回一个 `String`。我们创建了这个任务的 `Future` 并通过 `spawner` 提交给 `LocalPool`。

最后，我们调用 `pool.run()` 来执行 `LocalPool` 中的所有任务。由于 `LocalPool` 是单线程的，`run` 方法将在当前线程阻塞，直到所有任务都执行完毕。

请注意，`LocalPool` 只能在当前线程中运行任务，而且 `spawn_local` 只接受实现了 `LocalFutureObj` trait 的 `Future`，这意味着这些 `Future` 只能在当前线程上轮询。这是因为 `LocalPool` 不支持跨线程的任务调度，这与 `ThreadPool` 不同，后者可以在多个线程之间调度任务。



# 10、block_on 是单线程的

在当前线程上运行未来直至完成。

此函数将阻止调用者直到给定的未来完成为止。

[`LocalPool`](https://docs.rs/futures/0.3.30/futures/executor/struct.LocalPool.html)如果您需要对生成的任务进行更细粒度的控制，请使用。



# 11、futures 方法

以下是使用 `futures` 库中一些列出的功能的示例代码：

###  11.1 err

创建一个立即准备好的带有错误值的 future。

```rust
use futures::future::err;
use futures::executor::block_on;

fn main() {
    let future = err::<(), &str>("immediate error");
    assert_eq!(block_on(future), Err("immediate error"));
}
```

### 11.2 join, join3, join4, join5

并行地执行多个 future，并等待它们全部完成。

```rust
use futures::future::{join, join3, join4, join5};
use futures::executor::block_on;

async fn async_number(n: u32) -> u32 {
    n
}

fn main() {
    let future2 = join(async_number(1), async_number(2));
    let future3 = join3(async_number(1), async_number(2), async_number(3));
    let future4 = join4(async_number(1), async_number(2), async_number(3), async_number(4));
    let future5 = join5(async_number(1), async_number(2), async_number(3), async_number(4), async_number(5));

    assert_eq!(block_on(future2), (1, 2));
    assert_eq!(block_on(future3), (1, 2, 3));
    assert_eq!(block_on(future4), (1, 2, 3, 4));
    assert_eq!(block_on(future5), (1, 2, 3, 4, 5));
}
```

### 11.3 join_all

创建一个 future，表示给定 futures 的输出集合。

```rust
use futures::future::{join_all};
use futures::executor::block_on;

async fn async_number(n: u32) -> u32 {
    n
}

fn main() {
    let futures = vec![async_number(1), async_number(2), async_number(3)];
    let all_results = block_on(join_all(futures));
    assert_eq!(all_results, vec![1, 2, 3]);
}
```

### 11.4 lazy

创建一个允许延迟执行闭包的新 future。

```rust
use futures::future::lazy;
use futures::executor::block_on;

fn main() {
    let future_lazy = lazy(|_| {
        println!("Lazy execution");
        42
    });

    let result = block_on(future_lazy);
    assert_eq!(result, 42);
}
```

### 11.5 maybe_done

将 future 包装为 `MaybeDone`。

```rust
use futures::future::{self, MaybeDone};
use futures::executor::block_on;

async fn async_number() -> i32 {
    10
}

fn main() {
    let mut maybe_done_future = future::maybe_done(async_number());
    assert_eq!(block_on(&mut maybe_done_future), MaybeDone::Done(10));
}
```

### 11.6 ok

创建一个立即准备好的带有成功值的 future。

```rust
use futures::future::ok;
use futures::executor::block_on;

fn main() {
    let future = ok::<_, ()>(123);
    assert_eq!(block_on(future), Ok(123));
}
```

### 11.7 pending

创建一个永远不会解决的 future，代表一个永远不会完成的计算。

```rust
use futures::future::pending;
use futures::executor::block_on;

async fn never_end() {
    let _never = pending::<()>().await;
}

fn main() {
    let never_future = never_end();
    // 这里不调用 block_on，因为它永远不会完成
}
```

### 11.8 poll_fn

创建一个新的 future，围绕返回 `Poll` 的函数包装。

```rust
use futures::future::poll_fn;
use futures::task::{Context, Poll};
use std::pin::Pin;

fn main() {
    let mut counter = 0;

    let poll_fn_future = poll_fn(move |_: &mut Context<'_>| -> Poll<i32> {
        if counter < 5 {
            counter += 1;
            Poll::Pending
        } else {
            Poll::Ready(counter)
        }
    });

    let result = block_on(poll_fn_future);
    assert_eq!(result, 5);
}
```

### 11.9 ready

创建一个立即准备好的带有值的 future。

```rust
use futures::future::ready;
use futures::executor::block_on;

fn main() {
    let future = ready(42);
    assert_eq!(block_on(future), 42);
}
```

### 11.10 select, select_all, select_ok

等待两个不同类型的 future 完成任意一个，或在一个列表的 futures 上选择。

```rust
use futures::future::{self, select, select_all, select_ok};
use futures::executor::block_on;

async fn async_number(n: u32) -> Result<u32, u32> {
    Ok(n)
}

fn main() {
    let future1 = async_number(1);
    let future2 = async_number(2);
    let selected = block_on(select(future1, future2));
    match selected {
        future::Either::Left((ok, _)) => assert!(ok.is_ok()),
        future::Either::Right((ok, _)) => assert!(ok.is_ok()),
    }

    let futures = vec![async_number(1), async_number(2), async_number(3)];
    let all_results = block_on(select_all(futures));
    assert_eq!(all_results.0, Ok(1));

    let futures = vec![async_number(1), async_number(2), async_number(3)];
    let ok_results = block_on(select_ok(futures));
    assert!(ok_results.is_ok());
}
```

`select`, `select_all`, 和 `select_ok` 是 `futures` 库中的三个不同的函数，它们用于处理并发的 `Future` 实例，但它们的行为和用途有所不同。

1. **select**:
   `select` 函数用于同时等待两个 `Future` 中的任意一个完成。一旦其中一个 `Future` 完成，`select` 就会返回一个包含完成的 `Future` 结果和另一个未完成的 `Future` 的元组。它对于那些你只需要知道哪个 `Future` 首先完成的情况非常有用，而不关心另一个 `Future` 何时完成。

   ```rust
   use futures::future::{self, select};
   use futures::executor::block_on;
   
   async fn task_one() -> &'static str {
       // 模拟工作
       "one"
   }
   
   async fn task_two() -> &'static str {
       // 模拟工作
       "two"
   }
   
   let future_one = task_one();
   let future_two = task_two();
   let selected = block_on(select(future_one, future_two));
   match selected {
       future::Either::Left((result, _)) => println!("Task one completed first with result: {}", result),
       future::Either::Right((result, _)) => println!("Task two completed first with result: {}", result),
   }
   ```

2. **select_all**:
   `select_all` 函数用于等待一个 `Future` 列表中的任意一个完成，并返回所有 `Future` 的一个新的 `Vec`，其中第一个元素是第一个完成的 `Future`。与 `select` 不同，`select_all` 可以处理多于两个的 `Future`，并且在任意一个 `Future` 完成时返回，同时提供剩余未完成的 `Future` 列表。

   ```rust
   use futures::future::select_all;
   use futures::executor::block_on;
   
   async fn task(id: u32) -> u32 {
       // 模拟工作
       id
   }
   
   let futures = vec![task(1), task(2), task(3)];
   let (result, index, remaining) = block_on(select_all(futures));
   println!("Task {} completed first with result: {}", index + 1, result);
   // `remaining` 包含所有未完成的任务
   ```

3. **select_ok**:
   `select_ok` 函数类似于 `select_all`，但它专门用于处理返回 `Result` 类型的 `Future`。它会等待所有的 `Future` 完成，并返回第一个成功的结果（即 `Ok`）。如果所有的 `Future` 都返回了 `Err`，那么它会返回第一个 `Err`。这对于需要多个操作中的第一个成功结果，但不关心失败结果的情况非常有用。

   ```rust
   use futures::future::select_ok;
   use futures::executor::block_on;
   
   async fn task(id: u32) -> Result<u32, &'static str> {
       if id % 2 == 0 {
           Ok(id)
       } else {
           Err("Task failed")
       }
   }
   
   let futures = vec![task(1), task(2), task(3)];
   match block_on(select_ok(futures)) {
       Ok((result, _remaining)) => println!("The first successful task result: {}", result),
       Err(e) => println!("All tasks failed with error: {}", e),
   }
   ```

总结来说，`select` 用于两个 `Future`，返回第一个完成的结果；`select_all` 用于多个 `Future`，返回第一个完成的结果和未完成的 `Future` 列表；`select_ok` 用于返回 `Result` 的多个 `Future`，返回第一个成功的 `Ok` 结果或者所有 `Future` 都失败时的第一个 `Err`。



### 11.11 try_join, try_join3, try_join4, try_join5

等待两个或更多的 future 完成，或者在一个产生错误时立即返回。

```rust
use futures::future::{try_join, try_join3, try_join4, try_join5};
use futures::executor::block_on;

async fn async_number(n: u32) -> Result<u32, u32> {
    if n % 2 == 0 { Ok(n) } else { Err(n) }
}

fn main() {
    let future2 = try_join(async_number(2), async_number(4));
    let future3 = try_join3(async_number(2), async_number(4), async_number(6));
    let future4 = try_join4(async_number(2), async_number(4), async_number(6), async_number(8));
    let future5 = try_join5(async_number(2), async_number(4), async_number(6), async_number(8), async_number(10));

    assert_eq!(block_on(future2), Ok((2, 4)));
    assert_eq!(block_on(future3), Ok((2, 4, 6)));
    assert_eq!(block_on(future4), Ok((2, 4, 6, 8)));
    assert_eq!(block_on(future5), Ok((2, 4, 6, 8, 10)));
}
```

### 11.12 try_join_all

创建一个 future，表示给定 futures 的结果集合，或者在出现错误时返回错误。

```rust
use futures::future::{self, try_join_all};
use futures::executor::block_on;

async fn async_number(n: u32) -> Result<u32, u32> {
    if n % 2 == 0 { Ok(n) } else { Err(n) }
}

fn main() {
    let futures = vec![async_number(2), async_number(4), async_number(6)];
    let all_results = block_on(try_join_all(futures));
    assert_eq!(all_results, Ok(vec![2, 4, 6]));
}
```

### 11.13 try_maybe_done

包装一个 future 为 `TryMaybeDone`。

```rust
use futures::future::{self, TryMaybeDone};
use futures::executor::block_on;

async fn async_number() -> Result<i32, i32> {
    Ok(10)
}

fn main() {
    let mut maybe_done_future = future::try_maybe_done(async_number());
    assert_eq!(block_on(&mut maybe_done_future), TryMaybeDone::Done(Ok(10)));
}
```

### 11.14 try_select

等待两个不同类型的 future 完成任意一个。

```rust
use futures::future::{self, try_select};
use futures::executor::block_on;

async fn async_number(n: u32) -> Result<u32, u32> {
    Ok(n)
}

fn main() {
    let future1 = async_number(1);
    let future2 = async_number(2);
    let selected = block_on(try_select(future1, future2));
    match selected {
        Ok(future::Either::Left((ok, _))) => assert_eq!(ok, 1),
        Ok(future::Either::Right((ok, _))) => assert_eq!(ok, 2),
        Err(_) => panic!("unexpected error"),
    }
}
```

这些示例展示



# 12、join

要等待多个 `Future` 全部完成并返回它们的结果，你可以使用 `join!` 或 `join_all` 宏。这两种方法都能够实现等待多个异步任务全部完成的目的，但它们在使用上有一些区别。

### 使用 `join!` 宏

`join!` 宏允许你并行地等待多个 `Future` 完成，它是在编译时确定数量的 `Future`。`join!` 宏返回一个元组，其中包含所有 `Future` 的结果。这个宏非常适合在你事先知道需要并发执行的异步任务数量时使用。

```rust
use futures::join; // 确保引入了 `join` 宏
use futures::executor::block_on;

async fn async_task(i: i32) -> i32 {
    // 模拟异步操作
    i * 2
}

async fn run() {
    let a = async_task(1);
    let b = async_task(2);
    let c = async_task(3);

    // 并行等待所有异步任务完成
    let (result_a, result_b, result_c) = join!(a, b, c);
    println!("Results: {}, {}, {}", result_a, result_b, result_c);
}

fn main() {
    block_on(run());
}
```

### 使用 `join_all` 函数

`join_all` 函数用于在运行时确定数量的 `Future`，它接受一个 `Future` 的迭代器，并返回一个新的 `Future`，这个 `Future` 在所有给定的 `Future` 完成时解析为一个结果向量。`join_all` 是动态的，非常适合在你不知道有多少异步任务需要并发执行时使用。

```rust
use futures::future::join_all;
use futures::executor::block_on;

async fn async_task(i: i32) -> i32 {
    // 模拟异步操作
    i * 2
}

async fn run() {
    let tasks = vec![async_task(1), async_task(2), async_task(3)];
    
    // 并行等待所有异步任务完成，并收集结果
    let results = join_all(tasks).await;
    println!("Results: {:?}", results);
}

fn main() {
    block_on(run());
}
```

### 总结

- 使用 `join!` 宏适合在编译时已知所有 `Future` 数量的情况。
- 使用 `join_all` 函数适合在运行时确定 `Future` 数量的情况，特别是当 `Future` 的数量是动态生成的。

两种方法都可以实现并发等待多个 `Future` 完成并返回它们的结果，选择哪一种取决于你的具体需求和场景。



# 13、取消 future abortable

在 Rust 的异步编程中，有时你可能需要提前取消一个正在执行的 `Future`。这在异步任务需要根据某些条件提前结束，或者在长时间运行的任务需要被用户中断时非常有用。`futures` 库提供了一个 `abortable` 函数，用于创建可以被提前终止的 `Future`。

### 使用 `abortable`

`abortable` 函数接受一个 `Future` 并返回一个元组，其中包含一个新的 `Future` 和一个 `AbortHandle`。这个新的 `Future` 在行为上与原始的 `Future` 相同，但它可以通过对应的 `AbortHandle` 被提前终止。如果 `Future` 被提前终止了，它会返回一个 `Err(Aborted)` 错误。

这里是一个基本的使用示例：

```rust
use futures::future::{self, AbortHandle, Abortable};
use futures::executor::block_on;
use std::time::Duration;
use tokio::time::sleep; // 使用 tokio 的 sleep 来模拟异步等待

async fn long_running_task() -> &'static str {
    sleep(Duration::from_secs(2)).await;
    "Task completed"
}

fn main() {
    let (abortable_future, abort_handle) = future::abortable(long_running_task());

    // 在另一个线程中，我们可能决定在一秒后取消任务
    std::thread::spawn(move || {
        sleep(Duration::from_secs(1)).await;
        abort_handle.abort();
    });

    // 等待 abortable_future 完成
    match block_on(abortable_future) {
        Ok(result) => println!("Task completed: {}", result),
        Err(_) => println!("Task was aborted"),
    }
}
```

在上面的示例中，`long_running_task` 是一个模拟的长时间运行的异步任务。我们使用 `abortable` 将这个任务包装成一个可以被中断的 `Future`，然后在另一个线程中等待一秒后调用 `abort_handle.abort()` 来提前终止这个任务。主线程中，当尝试等待 `abortable_future` 完成时，因为任务被提前终止了，所以会收到一个 `Err`。

### 注意事项

- 当 `Future` 被提前终止时，它会立即停止执行并返回 `Err(Aborted)`。这意味着任何在 `Future` 中已经执行的部分不会被回滚，因此你可能需要手动处理一些清理工作。
- 使用 `abortable` 时，应当注意确保所有资源都能被适当地释放，特别是在 `Future` 中如果有文件操作或网络请求等操作时。
- `abortable` 并不会立即停止线程或者停止 `Future` 中的代码执行，它仅仅是标记这个 `Future` 为已终止，真正的终止行为取决于 `Future` 的实现和它正在执行的操作。



# 14、动态大小

`BoxFuture` 和 `LocalBoxFuture` 是 `futures` 库提供的两个非常有用的类型，它们允许你以动态类型的方式处理 `Future`，这在某些情况下非常有用，尤其是当你无法静态地知道你的 `Future` 的具体类型，或者当你需要在不同的上下文中传递 `Future` 时。

### BoxFuture

`BoxFuture<'a, T>` 是一个类型别名，它表示一个被 `Box` 包装的 `Future`，该 `Future` 的输出类型为 `T`，并且它拥有 `'a` 生命周期。这种 `Future` 被分配在堆上，这意味着它的大小在编译时不需要是已知的，从而允许你使用动态分发。此外，`BoxFuture` 通常要求 `Future` 是 `Send` 的，这意味着它可以安全地在多个线程之间传递。

```rust
use futures::future::{BoxFuture, FutureExt}; // FutureExt 为 Future 提供了 `.boxed()` 方法

fn async_example() -> BoxFuture<'static, i32> {
    async {
        // 异步操作
        42
    }
    .boxed()
}
```

### LocalBoxFuture

`LocalBoxFuture<'a, T>` 与 `BoxFuture` 类似，但它不要求 `Future` 是 `Send` 的。这意味着 `LocalBoxFuture` 不能安全地跨线程移动，但它可以用在仅限于单个线程的上下文中，比如特定的异步任务执行器（例如，`tokio` 的 `current_thread` 执行器）或特定的 GUI 线程。

```rust
use futures::future::{FutureExt, LocalBoxFuture}; // FutureExt 提供了 `.boxed_local()` 方法

fn async_example_local() -> LocalBoxFuture<'static, i32> {
    async {
        // 异步操作
        42
    }
    .boxed_local()
}
```

### 使用场景

`BoxFuture` 和 `LocalBoxFuture` 在以下情况下特别有用：

- **动态分发**：当你的函数需要返回实现了 `Future` trait 的不同类型时，动态分发允许你在运行时决定到底使用哪个实现。
- **递归异步函数**：在某些情况下，编写递归的异步函数可能会导致类型无限递归的问题。使用 `BoxFuture` 可以避免这个问题，因为它提供了一个具体的返回类型。
- **减少泛型参数的复杂性**：在某些复杂的异步流程中，函数签名可能会因为过多的泛型参数而变得难以理解和维护。使用 `BoxFuture` 可以简化这些函数签名。

总之，`BoxFuture` 和 `LocalBoxFuture` 提供了在 Rust 异步编程中处理动态类型 `Future` 的能力，使得代码可以在需要动态分发或处理特定异步模式时更加灵活。







































































































































