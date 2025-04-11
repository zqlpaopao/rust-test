#![allow(unused)]

use redis::cluster::{cluster_pipe, ClusterClient};
use redis::{AsyncCommands, Client, Commands, IntoConnectionInfo};

pub async fn test_redis() {
    let client = redis::Client::open("redis://localhost:7003").unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();
    let _: () = redis::cmd("PING").query_async(&mut conn).await.unwrap();
    conn.set::<&str, i32, ()>("key1", 1).await.unwrap();
    let res: usize = conn.get("key1").await.unwrap();
    println!("res is {}", res);

    let client = ClusterClient::new(vec![
        // "redis://0.0.0.0:7001/12".to_string(),
        // "redis://0.0.0.0:7002".into_connection_info().unwrap(),
        "redis://0.0.0.0:7003/".into_connection_info().unwrap(),
        // "redis://0.0.0.0:7004/".to_string().parse().unwrap(),
        // "redis://0.0.0.0:7005/12".to_string(),
        // "redis://0.0.0.0:7006/12".to_string(),
    ])
    .unwrap();
    // client.s
    let mut connection = client.get_async_connection().await.unwrap();

    let key = "test";

    connection.set::<&str, i32, ()>("key1", 1).await.unwrap();
    let res: usize = connection.get("key1").await.unwrap();
    println!("res is {}", res);

    // cluster_pipe()
    //     .rpush(key, "123").ignore()
    //     .ltrim(key, -10, -1).ignore()
    //     .expire(key, 60).ignore()
    //     .execute(&mut connection);

    //建立链接
    let mut client = Client::open("redis://127.0.0.1:6379").unwrap();
    let s = client.set::<String, String, bool>("aa".to_string(), "bb".to_string());
    println!("{:?}", s);

    let (connection, driver) = client.create_multiplexed_tokio_connection().await.unwrap();
    // 在后台运行驱动程序来处理连接的消息循环
    tokio::spawn(async move {
        driver.await;
    });

    // 获取一个连接的句柄，可以并发地发送命令
    let mut conn = connection;

    // 并发执行两个命令
    let set_fut: redis::RedisResult<String> = conn.set("key", "value").await;
    let get_fut: redis::RedisResult<String> = conn.get("key").await;

    // 等待两个命令完成
    // let (set_res, get_res): (_, _)  = tokio::try_join!(set_fut, get_fut).unwrap();

    // 输出结果
    println!("Set result: {:?}", set_fut);
    println!("Get result: {:?}", get_fut);

    //Set result: Ok("OK")
    // Get result: Ok("value")

    //pipe
    let mut cs = client.get_connection().unwrap();
    let (k1, k2): (i32, i32) = redis::pipe()
        .cmd("SET")
        .arg("key_1")
        .arg(42)
        .ignore()
        .cmd("SET")
        .arg("key_2")
        .arg(43)
        .ignore()
        .cmd("GET")
        .arg("key_1")
        .cmd("GET")
        .arg("key_2")
        .query(&mut cs)
        .unwrap();

    // 一个事物执行
    let (k1, k2): (i32, i32) = redis::pipe()
        .atomic()
        .cmd("SET")
        .arg("key_1")
        .arg(44)
        .ignore()
        .cmd("SET")
        .arg("key_2")
        .arg(45)
        .ignore()
        .cmd("GET")
        .arg("key_1")
        .cmd("GET")
        .arg("key_2")
        .query(&mut cs)
        .unwrap();

    let (k1, k2): (i32, i32) = redis::pipe()
        .atomic()
        .set("key_1", 42)
        .ignore()
        .set("key_2", 43)
        .ignore()
        .get("key_1")
        .get("key_2")
        .query(&mut cs)
        .unwrap();

    println!("pipe: {:?},{}", k1, k2);

    //发布订阅
    // let mut con = client.get_connection().unwrap();
    // let mut pubsub = con.as_pubsub();
    // pubsub.subscribe("channel_1").unwrap();
    // pubsub.subscribe("channel_2").unwrap();
    //
    // loop {
    //     let msg = pubsub.get_message().unwrap();
    //     let payload : String = msg.get_payload().unwrap();
    //     println!("channel '{}': {}", msg.get_channel_name(), payload);
    // }
}

/*
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
*/
