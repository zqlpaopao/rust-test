

# 1、依赖

```
# if you use tokio
redis = { version = "0.25.4", features = ["tokio-comp"] }

```

### 可选功能

已定义一些功能，如果需要，可以启用附加功能。其中一些功能默认启用。

- `acl`：启用 acl 支持（默认启用）
- `aio`：启用异步 IO 支持（默认启用）
- `geospatial`：启用地理空间支持（默认启用）
- `script`：启用脚本支持（默认启用）
- `r2d2`：启用 r2d2 连接池支持（可选）
- `ahash`：启用 ahash map/set 支持并在内部使用 ahash（性能提升 7-10%）（可选）
- `cluster`：启用 redis 集群支持（可选）
- `cluster-async`：启用异步 redis 集群支持（可选）
- `tokio-comp`：启用对 tokio 的支持（可选）
- `connection-manager`：启用自动重新连接支持（可选）
- `keep-alive`：通过`socket2`crate 启用套接字上的 keep-alive 选项（可选）

### 连接参数

redis-rs 知道定义连接去向的不同方法。参数 to`Client::open`需要实现 `IntoConnectionInfo`trait，有三种实现：

- `redis://`URL 格式的字符串切片。
- 来自 redis-url 包的 URL 对象。
- `ConnectionInfo`对象。

URL 格式为`redis://[<username>][:<password>@]<hostname>[:port][/<db>]`

如果有 Unix 套接字支持，则可以使用以下格式的 unix URL：

```
redis+unix:///<path>[?db=<db>[&pass=<password>][&user=<username>]]
```

为了兼容其他一些 redis 库，还支持“unix”方案：

```
unix:///<path>[?db=<db>][&pass=<password>][&user=<username>]]
```





# 2、建立连接

## 2.1 普通

```
fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    /* do something here */

    Ok(())
}
```



## 2.2 tokio 多路复用

您提供的代码段是`redis-rs`库中的一个函数定义，这个函数是`redis::Client`结构体的一部分，用于创建一个多路复lex的Tokio连接。这个函数在使用`tokio-comp`特性编译时才可用，这个特性是为了与Tokio异步运行时兼容。

这个函数返回一个`RedisResult`，它包含一个元组，第一个元素是`MultiplexedConnection`，这是一个可以用于发送并发命令的多路复用连接对象；第二个元素是一个实现了`Future` trait的类型，当`await`时，它会处理连接上的消息循环。

让我们看一下如何在实际代码中使用这个函数：

```rust
use redis::{AsyncCommands, Client, RedisResult};
use tokio;

#[tokio::main]
async fn main() -> RedisResult<()> {
    // 创建一个客户端并连接到Redis服务器
    let client = Client::open("redis://127.0.0.1/")?;
    // 创建一个多路复用的Tokio连接
    let (connection, driver) = client.create_multiplexed_tokio_connection().await?;

    // 在后台运行驱动程序来处理连接的消息循环
    tokio::spawn(async move {
        driver.await;
    });

    // 获取一个连接的句柄，可以并发地发送命令
    let mut conn = connection;

    // 并发执行两个命令
    let set_fut = conn.set("key", "value");
    let get_fut = conn.get("key");

    // 等待两个命令完成
    let (set_res, get_res) = tokio::try_join!(set_fut, get_fut)?;

    // 输出结果
    println!("Set result: {:?}", set_res);
    println!("Get result: {:?}", get_res);

    Ok(())
}
```

在这个例子中，我们首先创建了一个Redis客户端，然后调用了`create_multiplexed_tokio_connection`方法来获取一个多路复用的连接和它的驱动程序。接着，我们使用`tokio::spawn`来在后台运行这个驱动程序，这样它就可以在不干扰主逻辑的情况下独立运行。然后，我们并发地发送了两个命令，并使用`tokio::try_join!`宏来等待这两个命令都完成。

请注意，这段代码假设你的项目已经启用了`redis-rs`库的`tokio-comp`特性。如果你的`Cargo.toml`文件中没有启用这个特性，你需要按照前面的指示添加它。

## 2.3 tokio timeout

`create_multiplexed_tokio_connection_with_response_timeout` 方法是 `redis-rs` 库中的一个高级功能，它允许你创建一个多路复用的 Redis 连接，并且可以为这个连接设置一个响应超时时间。这意味着，如果 Redis 服务器在指定的时间内没有回应，任何等待的命令都会以超时错误结束。这对于确保你的应用程序能够优雅地处理潜在的网络延迟或服务不可用问题是非常有用的。

这个方法扩展了 `create_multiplexed_tokio_connection` 方法的功能，通过提供额外的参数来设置超时时间。下面是如何使用这个方法的一个示例：

### 示例

首先，请确保你的 `Cargo.toml` 文件中包含了正确的依赖项和特性标志：

```toml
[dependencies]
redis = { version = "0.23.0", features = ["tokio-comp"] } # 请根据需要更新版本
tokio = { version = "1.0", features = ["full"] }
```

然后，你可以使用以下代码段来创建连接：

```rust
use redis::{AsyncCommands, Client, RedisResult};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> RedisResult<()> {
    // 创建一个客户端并连接到Redis服务器
    let client = Client::open("redis://127.0.0.1/")?;
    
    // 创建一个多路复用的Tokio连接，并设置响应超时时间
    let timeout = Duration::from_secs(5); // 设置为5秒
    let (connection, driver) = client.create_multiplexed_tokio_connection_with_response_timeout(timeout).await?;

    // 在后台运行驱动程序来处理连接的消息循环
    tokio::spawn(async move {
        driver.await;
    });

    // 获取一个连接的句柄，可以并发地发送命令
    let mut conn = connection;

    // 尝试执行一个命令
    match conn.get::<_, String>("key").await {
        Ok(value) => println!("Got value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
```

在这个示例中，我们首先创建了一个 Redis 客户端并尝试连接到 Redis 服务器。通过调用 `create_multiplexed_tokio_connection_with_response_timeout` 方法并传入一个 `Duration` 值来设置响应超时时间。这个方法返回一个多路复用连接和一个驱动程序。接着，我们在后台运行这个驱动程序来处理连接的消息循环。最后，我们尝试执行一个 `get` 命令，并根据执行结果打印相应的消息。

请注意，这个方法需要 `tokio-comp` 特性标志在你的 `Cargo.toml` 文件中被启用，以确保与 Tokio 异步运行时的兼容性。



`create_multiplexed_tokio_connection`

这个函数可能用于创建一个多路复用的 Redis 连接，使用 Tokio 异步运行时。多路复用连接允许在单个连接上并发地执行多个 Redis 命令，而不需要为每个命令建立新的连接。这种方式提高了效率，尤其是在高负载情况下。

`create_multiplexed_tokio_connection_with_response_timeout`

这个函数的功能可能在 `create_multiplexed_tokio_connection` 的基础上增加了一个响应超时的特性。这意味着你可以为 Redis 命令执行设置一个最大等待时间，如果在指定的时间内没有收到响应，操作将被取消并报告超时错误。这对于需要高响应性的应用程序非常有用，可以防止因为等待一个长时间未响应的命令而导致的阻塞。



# 3、 流水线操作

```
//pipe
let mut cs = client.get_connection().unwrap();
let (k1, k2) : (i32, i32) = redis::pipe()
.cmd("SET").arg("key_1").arg(42).ignore()
.cmd("SET").arg("key_2").arg(43).ignore()
.cmd("GET").arg("key_1")
.cmd("GET").arg("key_2").query(&mut cs).unwrap();

// 一个原子执行
let (k1, k2) : (i32, i32) = redis::pipe()
.atomic()
.cmd("SET").arg("key_1").arg(44).ignore()
.cmd("SET").arg("key_2").arg(45).ignore()
.cmd("GET").arg("key_1")
.cmd("GET").arg("key_2").query(&mut cs).unwrap();


let (k1, k2) : (i32, i32) = redis::pipe()
.atomic()
.set("key_1", 42).ignore()
.set("key_2", 43).ignore()
.get("key_1")
.get("key_2").query(&mut cs)?;

println!("pipe: {:?},{}", k1,k2);
```



# 4、事务

```
use redis::Commands;
let key = "the_key";
let (new_val,) : (isize,) = redis::transaction(&mut con, &[key], |con, pipe| {
    let old_val : isize = con.get(key)?;
    pipe
        .set(key, old_val + 1).ignore()
        .get(key).query(con)
})?;
println!("The incremented number is: {}", new_val);
```



# 5、发布订阅

```
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_connection()?;
let mut pubsub = con.as_pubsub();
pubsub.subscribe("channel_1")?;
pubsub.subscribe("channel_2")?;

loop {
    let msg = pubsub.get_message()?;
    let payload : String = msg.get_payload()?;
    println!("channel '{}': {}", msg.get_channel_name(), payload);
}
```



# 6、lua

```
let script = redis::Script::new(r"
    return tonumber(ARGV[1]) + tonumber(ARGV[2]);
");
let result : isize = script.arg(1).arg(2).invoke(&mut con)?;
assert_eq!(result, 3);
```



# 7、异步

## 异步

除了上面解释过的同步接口之外，还存在基于[`futures`](https://crates.io/crates/futures)和的异步接口[`tokio`](https://tokio.rs/)。

这个接口存在于`aio`(async io) 模块下（需要`aio`启用该功能），并且在很大程度上反映了同步，但有一些让步，使其符合的限制`futures`。

```
use futures::prelude::*;
use redis::AsyncCommands;

let client = redis::Client::open("redis://127.0.0.1/").unwrap();
let mut con = client.get_async_connection().await?;

con.set("key1", b"foo").await?;

redis::cmd("SET").arg(&["key2", "bar"]).query_async(&mut con).await?;

let result = redis::cmd("MGET")
 .arg(&["key1", "key2"])
 .query_async(&mut con)
 .await;
assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
```

以下是对这些不同的 Redis 连接获取相关函数（假设是在某个 Rust 中功能丰富的 Redis 客户端库中出现的这些方法）的详细介绍及它们之间的区别说明：

### 1. `get_multiplexed_async_connection`
- **功能特点**：
  获取一个多路复用的异步 Redis 连接，允许多个 Redis 命令在同一个底层网络连接上并发地发送和处理，提高在高并发场景下与 Redis 交互的效率，减少连接创建和销毁的开销，适用于频繁执行 Redis 操作的情况，能更好地利用网络资源以及保证一定的操作顺序和原子性（在合理范围内）。
- **适用场景**：
  常用于如 Web 应用中大量读写 Redis 缓存、处理会话信息等需要频繁与 Redis 交互的高并发场景，或者批量执行多个 Redis 命令的操作场景，像批量更新一批键值对数据等情况。

### 2. `get_multiplexed_async_connection_with_timeouts`
- **功能特点**：
  在获取多路复用的异步 Redis 连接基础上，额外允许设置超时相关的参数。可以指定连接建立超时时间、命令执行超时时间等，避免因网络问题、Redis 服务器响应缓慢等原因导致操作长时间阻塞，增强了程序在面对不确定网络环境或服务器负载情况时的健壮性和可控性。
- **适用场景**：
  当应用运行在网络环境不稳定，或者对 Redis 操作的时效性有一定要求的场景下很有用。例如，在一个可能出现网络波动的分布式系统中，需要确保获取连接以及执行 Redis 命令不会无限制等待，通过设置合适的超时时间，能及时处理超时情况并采取相应的补救措施（如重试、报错等）。

### 3. `get_multiplexed_async_connection_with_config`
- **功能特点**：
  除了获取多路复用的异步连接外，还能依据传入的配置参数来定制连接的各项特性。这些配置参数可能涵盖连接池大小（如果有连接池相关逻辑）、最大空闲连接数、连接的验证方式（比如密码验证等）、网络相关的一些高级设置（如 TCP 相关的参数调整等）等多方面内容，提供了更精细化的连接控制能力。
- **适用场景**：
  适用于对 Redis 连接有特殊要求的复杂应用场景，比如根据不同的业务模块需求，配置不同的连接池大小来平衡资源使用和性能；或者在有严格安全要求的环境下，通过配置特定的验证参数确保连接的安全性等情况。

### 4. `get_multiplexed_tokio_connection_with_response_timeouts`
- **功能特点**：
  结合了多路复用连接和 `Tokio` 异步运行时的优势，同时针对 Redis 命令的响应设置了超时机制。获取的连接能很好地融入 `Tokio` 框架中，方便在 `Tokio` 异步任务里执行 Redis 操作，并且通过设置响应超时时间，确保不会因为 Redis 服务器响应过慢而长时间阻塞 `Tokio` 任务调度，保障整个异步应用的性能和响应及时性。
- **适用场景**：
  在基于 `Tokio` 构建的高性能异步应用中，且对 Redis 操作响应速度有要求的场景下使用。例如，在一个实时性要求较高的异步消息处理系统中，使用 Redis 作为消息队列的后端存储，通过这个连接获取方式可以确保从 Redis 获取消息等操作能在规定时间内完成，避免影响整个消息处理流程的时效性。

### 5. `get_multiplexed_tokio_connection`
- **功能特点**：
  侧重于获取适配 `Tokio` 异步运行时的多路复用 Redis 连接，使得在 `Tokio` 生态下能够高效地利用单个连接执行多个 Redis 命令，借助 `Tokio` 的异步调度机制实现并发操作，提升整体的运行效率，并且可以和其他 `Tokio` 异步任务自然地协同工作。
- **适用场景**：
  非常适合在 `Tokio` 构建的复杂网络应用中，有大量 Redis 相关操作需要执行的场景，比如在一个使用 `Tokio` 实现的微服务架构里，多个微服务频繁读写 Redis 缓存或者进行数据交互，通过这个连接方式可以在满足异步操作需求的同时，复用连接以节省资源。

### 6. `create_multiplexed_tokio_connection_with_response_timeout`
- **功能特点**：
  强调“创建”这样一个具有响应超时设置的、适配 `Tokio` 且支持多路复用的 Redis 连接，相比于单纯的“获取”连接函数，可能涉及更多底层的连接初始化、配置和超时相关逻辑的设置工作，通常可以根据传入的参数（如 Redis 服务器地址、端口、可能的认证信息等）来从头构建出满足特定超时要求的合适连接。
- **适用场景**：
  在需要动态地、按照特定需求创建带有响应超时特性的 `Tokio` 适配的多路复用 Redis 连接的场景下使用，例如，根据不同的配置环境（开发环境、测试环境、生产环境等）动态地构建连接，每个环境对响应超时时间有不同要求，就可以通过这个函数来灵活创建。

### 7. `create_multiplexed_tokio_connection`
- **功能特点**：
  主要用于创建一个适配 `Tokio` 的多路复用 Redis 连接，重点在于构建连接的过程，可能会涉及到诸如初始化网络连接、配置相关的连接属性等操作，使得最终创建出的连接能够满足在 `Tokio` 异步环境下进行多路复用操作的需求。
- **适用场景**：
  当需要手动创建一个新的、符合 `Tokio` 异步运行时要求的多路复用 Redis 连接时使用，比如在应用启动阶段，根据配置文件里的 Redis 服务器相关信息来创建连接，为后续的 Redis 操作做好准备工作。

### 8. `get_multiplexed_async_connection_inner`
- **功能特点**：
  从名字推测可能是获取多路复用异步连接的内部实现相关的函数，也许返回的是更接近底层实现或者带有更多内部状态信息的连接对象，相较于对外公开的常规获取连接函数，可能提供了一些额外的、供库内部或者更高级用户进行深度定制、调试等操作的能力，但使用上可能需要对库的内部实现有一定了解。
- **适用场景**：
  一般适用于对 Redis 客户端库内部机制熟悉，需要进行一些特殊的底层操作、自定义连接行为或者调试连接相关问题的场景，比如库的开发者进行功能扩展、优化或者排查特定连接问题时可能会用到这个函数。

### 9. `create_multiplexed_async_connection_inner`
- **功能特点**：
  类似前面的“创建”类函数，不过侧重于创建多路复用异步连接的内部实现逻辑，可能会涉及到直接操作更底层的资源、按照库内部特定的规则和数据结构来构建连接，创建出来的连接对象或许更便于在库内部进行进一步的处理、与其他内部模块协同等，同样要求使用者对库的内部架构有一定认知。
- **适用场景**：
  主要用于库内部开发、扩展或者深度定制连接创建过程的场景，例如在改进库的连接创建机制、添加新的特性支持时，需要在底层操作来创建符合特定内部设计要求的多路复用异步连接时会用到。

### 10. `get_simple_async_connection_dynamically`
- **功能特点**：
  获取一个简单的异步 Redis 连接，并且强调是“动态地”获取，可能意味着可以根据运行时的一些条件（比如不同的配置参数、环境变量等）灵活地决定如何获取连接，不像一些固定配置的连接获取方式。它获取的是单个独立的异步连接，每次使用通常需要单独创建和管理，与多路复用连接相对，适用于一些简单的、偶尔执行的 Redis 操作场景。
- **适用场景**：
  适用于偶尔需要执行简单 Redis 操作，且对连接复用需求不大的情况，比如在一个小型工具应用中，只是偶尔查询一下 Redis 中的某个配置项，或者临时存储一个简单的数据，通过这种动态获取简单异步连接的方式就可以满足需求，同时代码逻辑相对简单，不需要处理复杂的多路复用相关管理工作。

### 11. `get_simple_async_connection`
- **功能特点**：
  获取一个普通的简单异步 Redis 连接，每次调用大概率会创建一个独立的连接用于执行 Redis 操作，相对比较直接简单，适用于对连接独立性有要求，不需要复用连接的场景，操作上比较直观，不过在高并发场景下可能会因为频繁创建连接而消耗更多资源。
- **适用场景**：
  常用于简单的、单次的 Redis 操作场景，或者对连接有特殊独立性要求的情况，比如在一个复杂系统里，某个模块希望独占一个连接来执行特定的、不希望受其他并发操作影响的 Redis 操作，就可以使用这个函数来获取连接。

### 12. `get_async_pubsub`
- **功能特点**：
  专门用于获取一个异步的 Redis 发布/订阅（Pub/Sub）连接，通过这个连接可以实现 Redis 中的消息发布与订阅功能，比如可以在不同的客户端之间通过 Redis 作为中间件来广播消息、实时推送通知等，并且操作是异步的，能与其他异步任务并发执行，符合异步编程的高效性要求。
- **适用场景**：
  在构建实时通信系统、消息推送服务、事件驱动架构等场景中非常有用，例如在一个在线聊天应用中，通过 Redis 的 Pub/Sub 机制来实现消息的实时广播，不同的客户端通过获取这个异步的 Pub/Sub 连接来订阅感兴趣的频道或者发布消息，实现实时的信息交互。

### 13. `get_async_monitor`
- **功能特点**：
  获取一个用于异步监控 Redis 相关情况的连接（具体监控内容可能因库而异，比如监控 Redis 服务器的状态、连接的健康情况、命令执行统计等），通过这个连接可以在后台异步地收集 Redis 的各种信息，以便进行性能分析、故障排查、资源管理等操作，为保障 Redis 服务的稳定运行提供支持。
- **适用场景**：
  适用于需要对 Redis 运行状态进行实时监控的场景，比如在运维管理系统中，希望持续了解 Redis 服务器的负载情况、连接数变化、命令执行效率等信息，通过这个获取监控连接的方式来异步地获取相关数据，进而进行分析和采取相应的维护措施。

总体而言，这些不同的函数各自针对不同的需求和场景提供了多样化的获取 Redis 连接的方式，开发者可以根据具体的项目架构、业务需求、并发情况以及对 Redis 操作的具体要求等因素，来选择合适的连接获取方法以实现高效、可靠的 Redis 操作。

redis cluster

redis.conf 弄三份
````
port 7001  # 修改为 7002 或 7003 对应其他实例
cluster-enabled yes
cluster-config-file nodes.conf
cluster-node-timeout 5000
appendonly yes
daemonize yes
pidfile /usr/local/var/run/redis_7001.pid  # 修改 pidfile 路径以匹配端口号
logfile "/usr/local/var/log/redis_7001.log"  # 修改日志文件路径以匹配端口号
dir /usr/local/var/db/redis_7001  # 修改数据目录路径以匹配端口号
```

启动redis实例
```
redis-server ~/redis-cluster/conf/redis-7001.conf
redis-server ~/redis-cluster/conf/redis-7002.conf
redis-server ~/redis-cluster/conf/redis-7003.conf
```

初始化 Redis 集群
```
redis-cli --cluster create \
  127.0.0.1:7001 127.0.0.1:7002 127.0.0.1:7003 \
  --cluster-replicas 0
```
验证集群状态
```

redis-cli -p 7001 cluster info
redis-cli -p 7001 cluster nodes
```