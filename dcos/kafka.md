

# 1、安装kafka

```
docker pull apache/kafka:3.7.1

docker run -p 9092:9092 apache/kafka:3.7.1
```



**zk kafka **

**kafka-manager**

Docker-composer

```
version: '2'
services:
  zookeeper:
    image: confluentinc/cp-zookeeper:latest
    container_name: zookeeper
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    networks:
      - kafka-net

  kafka:
    image: confluentinc/cp-kafka:latest
    container_name: kafka
    depends_on:
      - zookeeper
    ports:
      - 9092:9092
      - 9093:9093
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka:9092,PLAINTEXT_HOST://localhost:9093
      KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: PLAINTEXT:PLAINTEXT,PLAINTEXT_HOST:PLAINTEXT
      KAFKA_INTER_BROKER_LISTENER_NAME: PLAINTEXT
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
    networks:
      - kafka-net

  cmak:
    image: hlebalbau/kafka-manager:latest
    container_name: cmak
    depends_on:
      - zookeeper
      - kafka
    ports:
      - "9010:9000"
    environment:
      ZK_HOSTS: "zookeeper:2181"
      APPLICATION_SECRET: "random-secret"
    networks:
      - kafka-net

  kafka-ui:
    image: provectuslabs/kafka-ui:latest
    container_name: kafka-ui
    restart: always
    ports:
      - 10010:8080
    environment:
      - DYNAMIC_CONFIG_ENABLED=true
      - SERVER_SERVLET_CONTEXT_PATH=/ui-kafka
      - KAFKA_CLUSTERS_0_NAME=local
      - KAFKA_CLUSTERS_0_BOOTSTRAPSERVERS=kafka:9092
      - KAFKA_CLUSTERS_0_PROPERTIES_SECURITY_PROTOCOL=PLAINTEXT
    networks:
      - kafka-net

networks:
  kafka-net:


```
当访问 localhost:10010 返回 404 错误时，通常是由于 Kafka-UI 的上下文路径（SERVER_SERVLET_CONTEXT_PATH）配置与访问方式不匹配导致的。以下是完整的排查和解决方案：

您在环境变量中设置了 SERVER_SERVLET_CONTEXT_PATH=/ui-kafka，这意味着：

Kafka-UI 的实际访问路径应为 http://localhost:10010/ui-kafka

直接访问 http://localhost:10010 会返回 404

docker 设置有认证的[地址](https://blog.csdn.net/yztezhl/article/details/127627854)


# 2、 添加依赖

```
rdkafka = "0.36.2"
```



# 3、 简单生产者





# 4、 生产者参数

[-](https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md)

## 4.1 全局配置属性



| 财产                                    | 合同/付款 | 范围                                                         | 默认                                                         | 重要性 | 描述                                                         |
| --------------------------------------- | --------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------ | ------------------------------------------------------------ |
| builtin.features                        | *         |                                                              | gzip、snappy、ssl、sasl、正则表达式、lz4、sasl_gssapi、sasl_plain、sasl_scram、插件、zstd、sasl_oauthbearer、http、oidc | 低的   | 表示此 librdkafka 版本的内置功能。应用程序可以查询此值，也可以尝试使用其所需功能列表来设置它以检查库支持。 *类型：CSV 标志* |
| client.id                               | *         |                                                              | 德卡夫卡                                                     | 低的   | 客户端标识符。 *类型：字符串*                                |
| metadata.broker.list                    | *         |                                                              |                                                              | 高的   | 代理的初始列表为代理主机或主机：端口的 CSV 列表。应用程序还可用于`rd_kafka_brokers_add()`在运行时添加代理。 *类型：字符串* |
| bootstrap.servers                       | *         |                                                              |                                                              | 高的   | 别名`metadata.broker.list`：代理的初始列表，以代理主机或主机：端口的 CSV 列表形式显示。应用程序还可用于`rd_kafka_brokers_add()`在运行时添加代理。 *类型：字符串* |
| message.max.bytes                       | *         | 1000...1000000000                                            | 1000000                                                      | 中等的 | Kafka 协议请求消息的最大大小。由于协议版本之间的帧开销不同，生产者无法在生产时可靠地执行严格的最大消息限制，并且可能在协议 ProduceRequests 中超过最大大小一条消息，代理将强制执行主题的限制`max.message.bytes`（请参阅 Apache Kafka 文档）。 *类型：整数* |
| message.copy.max.bytes                  | *         | 0...1000000000                                               | 65535                                                        | 低的   | 复制到缓冲区的消息的最大大小。大于此大小的消息将通过引用传递（零复制），但代价是更大的 iovecs。 *类型：整数* |
| receive.message.max.bytes               | *         | 1000..2147483647                                             | 100000000                                                    | 中等的 | 最大 Kafka 协议响应消息大小。这可作为安全预防措施，避免在协议出现故障时内存耗尽。此值必须至少为`fetch.max.bytes` + 512 以允许协议开销；除非明确设置配置属性，否则该值会自动调整。 *类型：整数* |
| max.in.flight.requests.per.connection   | *         | 1..1000000                                                   | 1000000                                                      | 低的   | 每个代理连接的最大在途请求数。这是适用于所有代理通信的通用属性，但它主要与生产请求相关。特别注意，其他机制将每个代理的未完成消费者提取请求数限制为一个。 *类型：整数* |
| max.in.flight                           | *         | 1..1000000                                                   | 1000000                                                      | 低的   | 别名`max.in.flight.requests.per.connection`：每个代理连接的最大在途请求数。这是适用于所有代理通信的通用属性，但它主要与生产请求相关。特别要注意的是，其他机制将每个代理的未完成消费者提取请求数限制为一个。 *类型：整数* |
| topic.metadata.refresh.interval.ms      | *         | -1..3600000                                                  | 300000                                                       | 低的   | 主题和代理元数据刷新的时间段（以毫秒为单位），以便主动发现任何新的代理、主题、分区或分区领导者更改。使用 -1 禁用间隔刷新（不推荐）。如果没有本地引用的主题（没有创建主题对象、没有生成消息、没有订阅或没有分配），则只有代理列表会每隔一段时间刷新一次，但刷新频率不会超过每 10 秒一次。 *类型：整数* |
| metadata.max.age.ms                     | *         | 1..86400000                                                  | 900000                                                       | 低的   | 元数据缓存最大年龄。默认为 topic.metadata.refresh.interval.ms * 3 *类型：整数* |
| topic.metadata.refresh.fast.interval.ms | *         | 1..60000                                                     | 100                                                          | 低的   | 当主题失去其领导者时，新的元数据请求将立即入队，然后以此初始间隔呈指数增长至`retry.backoff.max.ms`，直到主题元数据被刷新。如果未明确设置，则默认为`retry.backoff.ms`。这用于从过渡领导者代理中快速恢复。 *类型：整数* |
| topic.metadata.refresh.fast.cnt         | *         | 0 ... 1000                                                   | 10                                                           | 低的   | **已弃用**不再使用。 *类型：整数*                            |
| topic.metadata.refresh.sparse           | *         | 真假                                                         | 真的                                                         | 低的   | 稀疏元数据请求（消耗较少的网络带宽） *类型：布尔值*          |
| topic.metadata.propagation.max.ms       | *         | 0...3600000                                                  | 30000                                                        | 低的   | Apache Kafka 主题创建是异步的，新主题需要一些时间才能在整个集群中传播到所有代理。如果客户端在手动创建主题之后请求主题元数据，但在主题完全传播到客户端请求元数据的代理之前，该主题将被视为不存在，客户端将标记该主题，使排队的生成消息失败`ERR__UNKNOWN_TOPIC`。此设置会延迟将主题标记为不存在，直到配置的最大传播时间过去。最大传播时间是从客户端中首次引用主题的时间开始计算的，例如在 produce() 上。 *类型：整数* |
| topic.blacklist                         | *         |                                                              |                                                              | 低的   | 主题黑名单，用于匹配主题名称的正则表达式的逗号分隔列表，这些主题名称应在代理元数据信息中被忽略，就好像这些主题不存在一样。 *类型：模式列表* |
| debug                                   | *         | generic, broker, topic, metadata, feature, queue, msg, protocol, cgrp, security, fetch, interceptor, plugin, consumer, admin, eos, mock, assignor, conf, telemetry, all |                                                              | 中等的 | 要启用的调试上下文的逗号分隔列表。详细的生产者调试：broker、topic、msg。消费者：consumer、cgrp、topic、fetch *类型：CSV 标志* |
| socket.timeout.ms                       | *         | 10..300000                                                   | 60000                                                        | 低的   | `socket.timeout.ms`网络请求的默认超时。生产者：ProduceRequests 将使用和 中较小的值`message.timeout.ms`作为批次中的第一条消息。消费者：FetchRequests 将使用`fetch.wait.max.ms`+ `socket.timeout.ms`。管理员：管理员请求将使用`socket.timeout.ms`或明确设置`rd_kafka_AdminOptions_set_operation_timeout()`值。 *类型：整数* |
| socket.blocking.max.ms                  | *         | 1..60000                                                     | 1000                                                         | 低的   | **已弃用**不再使用。 *类型：整数*                            |
| socket.send.buffer.bytes                | *         | 0...100000000                                                | 0                                                            | 低的   | Broker 套接字发送缓冲区大小。如果为 0，则使用系统默认值。 *类型：整数* |
| socket.receive.buffer.bytes             | *         | 0...100000000                                                | 0                                                            | 低的   | Broker 套接字接收缓冲区大小。如果为 0，则使用系统默认值。 *类型：整数* |
| socket.keepalive.enable                 | *         | 真假                                                         | 错误的                                                       | 低的   | 在代理套接字上启用 TCP 保持活动 (SO_KEEPALIVE) *类型：布尔值* |
| socket.nagle.disable                    | *         | 真假                                                         | 错误的                                                       | 低的   | 在代理套接字上禁用 Nagle 算法 (TCP_NODELAY)。 *类型：布尔值* |
| socket.max.fails                        | *         | 0 ... 1000000                                                | 1                                                            | 低的   | 当发送失败次数达到此值时（例如，请求超时），断开与代理的连接。禁用值为 0。警告：强烈建议将此设置保留为默认值 1，以避免在请求超时的情况下客户端和代理不同步。注意：连接会自动重新建立。 *类型：整数* |
| broker.address.ttl                      | *         | 0..86400000                                                  | 1000                                                         | 低的   | Broker 地址解析结果缓存时间（毫秒）， *类型：整数*           |
| broker.address.family                   | *         | 任意，v4，v6                                                 | 任何                                                         | 低的   | 允许的代理 IP 地址系列：任意、v4、v6 *类型：枚举值*          |
| socket.connection.setup.timeout.ms      | *         | 1000..2147483647                                             | 30000                                                        | 中等的 | 允许代理连接设置的最大时间（TCP 连接设置以及 SSL 和 SASL 握手）。如果在此之后与代理的连接未完全正常，则将关闭并重试连接。 *类型：整数* |
| connections.max.idle.ms                 | *         | 0..2147483647                                                | 0                                                            | 中等的 | 在指定的不活动时间后关闭代理连接。使用 0 禁用。如果此属性保留其默认值，则会执行一些启发式方法来确定合适的默认值，目前仅限于识别 Azure 上的代理（有关更多信息，请参阅 librdkafka 问题 #3109）。 *类型：整数* |
| reconnect.backoff.jitter.ms             | *         | 0...3600000                                                  | 0                                                            | 低的   | **已弃用**不再使用。请参阅`reconnect.backoff.ms`和`reconnect.backoff.max.ms`。 *类型：整数* |
| reconnect.backoff.ms                    | *         | 0...3600000                                                  | 100                                                          | 中等的 | 连接关闭后重新连接到代理之前等待的初始时间。该时间会呈指数增加，直到`reconnect.backoff.max.ms`达到。每次重新连接退避都会应用 -25% 到 +50% 的抖动。值为 0 表示禁用退避并立即重新连接。 *类型：整数* |
| reconnect.backoff.max.ms                | *         | 0...3600000                                                  | 10000                                                        | 中等的 | 连接关闭后重新连接到代理之前等待的最长时间。 *类型：整数*    |
| statistics.interval.ms                  | *         | 0..86400000                                                  | 0                                                            | 高的   | librdkafka 统计信息发出间隔。应用程序还需要使用 注册统计回调`rd_kafka_conf_set_stats_cb()`。粒度为 1000 毫秒。值为 0 表示禁用统计信息。 *类型：整数* |
| enabled_events                          | *         | 0..2147483647                                                | 0                                                            | 低的   | 参见`rd_kafka_conf_set_events()` *类型：整数*                |
| error_cb                                | *         |                                                              |                                                              | 低的   | 错误回调（使用 rd_kafka_conf_set_error_cb() 设置） *类型：参见专用 API* |
| throttle_cb                             | *         |                                                              |                                                              | 低的   | 节流回调（使用 rd_kafka_conf_set_throttle_cb() 设置） *类型：参见专用 API* |
| stats_cb                                | *         |                                                              |                                                              | 低的   | 统计回调（使用 rd_kafka_conf_set_stats_cb() 设置） *类型：参见专用 API* |
| log_cb                                  | *         |                                                              |                                                              | 低的   | 日志回调（使用 rd_kafka_conf_set_log_cb() 设置） *类型：参见专用 API* |
| log_level                               | *         | 0..7                                                         | 6                                                            | 低的   | 日志记录级别（syslog(3) 级别） *类型：整数*                  |
| log.queue                               | *         | 真假                                                         | 错误的                                                       | 低的   | 从内部 librdkafka 线程禁用自发的 log_cb，而是将日志消息排入队列设置`rd_kafka_set_log_queue()`并通过标准轮询 API 提供日志回调或事件。**注意**：日志消息将停留在临时队列中，直到设置日志队列。 *类型：布尔值* |
| log.thread.name                         | *         | 真假                                                         | 真的                                                         | 低的   | 在日志消息中打印内部线程名称（用于调试 librdkafka 内部） *类型：布尔值* |
| enable.random.seed                      | *         | 真假                                                         | 真的                                                         | 低的   | 如果启用，librdkafka 将在第一次调用 rd_kafka_new() 时使用 srand(current_time.milliseconds) 初始化 PRNG（仅当您的平台上没有 rand_r() 时才需要）。如果禁用，应用程序必须在调用 rd_kafka_new() 之前调用 srand()。 *类型：布尔值* |
| log.connection.close                    | *         | 真假                                                         | 真的                                                         | 低的   | 日志代理断开连接。与具有积极值的 0.9 代理交互时，关闭此功能可能会很有用`connections.max.idle.ms`。 *类型：布尔值* |
| background_event_cb                     | *         |                                                              |                                                              | 低的   | 后台队列事件回调（使用 rd_kafka_conf_set_background_event_cb() 设置） *类型：参见专用 API* |
| socket_cb                               | *         |                                                              |                                                              | 低的   | 套接字创建回调以提供无竞争的 CLOEXEC *类型：请参阅专用 API*  |
| connect_cb                              | *         |                                                              |                                                              | 低的   | 套接字连接回调 *类型：参见专用 API*                          |
| closesocket_cb                          | *         |                                                              |                                                              | 低的   | 套接字关闭回调 *类型：参见专用 API*                          |
| open_cb                                 | *         |                                                              |                                                              | 低的   | 文件打开回调提供无竞争的 CLOEXEC *类型：参见专用 API*        |
| resolve_cb                              | *         |                                                              |                                                              | 低的   | 地址解析回调（使用 rd_kafka_conf_set_resolve_cb() 设置）。 *类型：参见专用 API* |
| opaque                                  | *         |                                                              |                                                              | 低的   | 应用程序不透明（使用 rd_kafka_conf_set_opaque() 设置） *类型：参见专用 API* |
| default_topic_conf                      | *         |                                                              |                                                              | 低的   | 自动订阅主题的默认主题配置 *类型：见专用API*                 |
| internal.termination.signal             | *         | 0..128                                                       | 0                                                            | 低的   | librdkafka 将使用的信号，用于在 rd_kafka_destroy() 上快速终止。如果未设置此信号，则在 rd_kafka_wait_destroyed() 返回 true 之前会有延迟，因为内部线程正在超时其系统调用。但是，如果设置了此信号，延迟将最小。应用程序应屏蔽此信号，因为已安装内部信号处理程序。 *类型：整数* |
| api.version.request                     | *         | 真假                                                         | 真的                                                         | 高的   | 请求代理支持的 API 版本以根据可用的协议功能调整功能。如果设置为 false，或者 ApiVersionRequest 失败，`broker.version.fallback`则将使用后备版本。**注意**：取决于代理版本 >=0.10.0。如果（较旧的）代理不支持该请求，则`broker.version.fallback`使用后备版本。 *类型：布尔值* |
| api.version.request.timeout.ms          | *         | 1..300000                                                    | 10000                                                        | 低的   | 代理 API 版本请求超时。 *类型：整数*                         |
| api.version.fallback.ms                 | *         | 0..604800000                                                 | 0                                                            | 中等的 | `broker.version.fallback`指示在 ApiVersionRequest 失败的情况下使用回退的时间长度。**注意**：仅在与代理建立新连接时（例如升级后）才会发出 ApiVersionRequest。 *类型：整数* |
| broker.version.fallback                 | *         |                                                              | 0.10.0                                                       | 中等的 | 较旧的代理版本（0.10.0 之前）不提供客户端查询支持的协议功能（ApiVersionRequest，请参阅`api.version.request`），因此客户端无法知道可以使用哪些功能。作为一种解决方法，用户可以将此属性设置为预期的代理版本，如果 ApiVersionRequest 失败（或被禁用），客户端将自动调整其功能集。后备代理版本将用于`api.version.fallback.ms`。有效值为：0.9.0、0.8.2、0.8.1、0.8.0。任何其他 >= 0.10 的值（例如 0.10.2.1）均启用 ApiVersionRequests。 *类型：字符串* |
| allow.auto.create.topics                | *         | 真假                                                         | 错误的                                                       | 低的   | 订阅或分配不存在的主题时，允许在代理上自动创建主题。代理还必须配置`auto.create.topics.enable=true`才能使此配置生效。注意：生产者的默认值（true）与消费者的默认值（false）不同。此外，消费者默认值与 Java 消费者（true）不同，并且 Java 生产者不支持此属性。需要代理版本 >= 0.11.0.0，对于较旧的代理版本，仅代理配置适用。 *类型：布尔值* |
| security.protocol                       | *         | plaintext, ssl, sasl_plaintext, sasl_ssl                     | 纯文本                                                       | 高的   | 用于与经纪人沟通的协议。 *类型：枚举值*                      |
| ssl.cipher.suites                       | *         |                                                              |                                                              | 低的   | 密码套件是身份验证、加密、MAC 和密钥交换算法的命名组合，用于协商使用 TLS 或 SSL 网络协议的网络连接的安全设置。请参阅手册页和`ciphers(1)`“SSL_CTX_set_cipher_list(3)”。 *类型：字符串* |
| ssl.curves.list                         | *         |                                                              |                                                              | 低的   | TLS ClientHello 消息中的受支持曲线扩展指定客户端愿意让服务器使用的曲线（标准/命名，或“显式”GF(2^k) 或 GF(p)）。请参阅手册页`SSL_CTX_set1_curves_list(3)`。需要 OpenSSL >= 1.0.2。 *类型：字符串* |
| ssl.sigalgs.list                        | *         |                                                              |                                                              | 低的   | 客户端使用 TLS ClientHello signature_algorithms 扩展向服务器指示哪些签名/哈希算法对可用于数字签名。请参阅手册页`SSL_CTX_set1_sigalgs_list(3)`。需要 OpenSSL >= 1.0.2。 *类型：字符串* |
| ssl.sigalgs.location                    | *         |                                                              |                                                              | 低的   | 用于身份验证的客户端私钥 (PEM) 的路径。 *类型：字符串*       |
| ssl.key.password                        | *         |                                                              |                                                              | 低的   | 私钥密码（与`ssl.key.location`和 一起使用`set_ssl_cert()`） *类型：字符串* |
| ssl.key.pem                             | *         |                                                              |                                                              | 低的   | 用于身份验证的客户端私钥字符串（PEM格式）。 *类型：字符串*   |
| ssl_key                                 | *         |                                                              |                                                              | 低的   | 客户端的私钥由 rd_kafka_conf_set_ssl_cert() 设置 *类型：参见专用 API* |
| ssl.certificate.location                | *         |                                                              |                                                              | 低的   | 用于身份验证的客户端公钥 (PEM) 路径。 *类型：字符串*         |
| ssl.certificate.pem                     | *         |                                                              |                                                              | 低的   | 用于身份验证的客户端公钥字符串（PEM 格式）。 *类型：字符串*  |
| ssl_certificate                         | *         |                                                              |                                                              | 低的   | 客户端的公钥由 rd_kafka_conf_set_ssl_cert() 设置 *类型：参见专用 API* |
| ssl.ca.location                         | *         |                                                              |                                                              | 低的   | 用于验证代理密钥的 CA 证书的文件或目录路径。默认值：在 Windows 上，系统的 CA 证书会自动在 Windows 根证书存储中查找。在 Mac OSX 上，此配置默认为`probe`。建议使用 Homebrew 安装 openssl，以提供 CA 证书。在 Linux 上，安装发行版的 ca-certificates 包。如果 OpenSSL 是静态链接的或`ssl.ca.location`设置为`probe`，将探测标准路径列表，并将找到的第一个路径用作默认 CA 证书位置路径。如果 OpenSSL 是动态链接的，则将使用 OpenSSL 库的默认路径（参见`OPENSSLDIR`）`openssl version -a`。 *类型：字符串* |
| ssl.ca.pem                              | *         |                                                              |                                                              | 低的   | 用于验证代理密钥的 CA 证书字符串（PEM 格式）。 *类型：字符串* |
| ssl_ca                                  | *         |                                                              |                                                              | 低的   | 由 rd_kafka_conf_set_ssl_cert() 设置的 CA 证书 *类型：参见专用 API* |
| ssl.ca.certificate.stores               | *         |                                                              | 根                                                           | 低的   | 以逗号分隔的 Windows 证书存储列表，用于从中加载 CA 证书。证书将按照存储指定的顺序加载。如果无法从任何指定的存储中加载证书，则会记录错误，并使用 OpenSSL 库的默认 CA 位置。存储名称通常是以下一个或多个：MY、Root、Trust、CA。 *类型：字符串* |
| ssl.crl.location                        | *         |                                                              |                                                              | 低的   | 用于验证代理证书有效性的 CRL 路径。 *类型：字符串*           |
| ssl.keystore.location                   | *         |                                                              |                                                              | 低的   | 用于身份验证的客户端密钥库 (PKCS#12) 的路径。 *类型：字符串* |
| ssl.keystore.password                   | *         |                                                              |                                                              | 低的   | 客户端的密钥库（PKCS#12）密码。 *类型：字符串*               |
| ssl.providers                           | *         |                                                              |                                                              | 低的   | OpenSSL 3.0.x 实现提供商的逗号分隔列表。例如，“default,legacy”。 *类型：字符串* |
| ssl.engine.location                     | *         |                                                              |                                                              | 低的   | **已弃用**OpenSSL 引擎库的路径。需要 OpenSSL >= 1.1.x。已弃用：OpenSSL 引擎支持已弃用，应由 OpenSSL 3 提供商替换。 *类型：字符串* |
| ssl.engine.id                           | *         |                                                              | 动态的                                                       | 低的   | OpenSSL engine id 是用于加载引擎的名称。 *类型：字符串*      |
| ssl_engine_callback_data                | *         |                                                              |                                                              | 低的   | OpenSSL 引擎回调数据（使用 rd_kafka_conf_set_engine_callback_data() 设置）。 *类型：参见专用 API* |
| enable.ssl.certificate.verification     | *         | 真假                                                         | 真的                                                         | 低的   | 启用 OpenSSL 的内置代理（服务器）证书验证。应用程序可以通过实现 certificate_verify_cb 来扩展此验证。 *类型：布尔值* |
| ssl.endpoint.identification.algorithm   | *         | 无，https                                                    | https                                                        | 低的   | 使用代理证书验证代理主机名的端点识别算法。https - 服务器（代理）主机名验证，如 RFC2818 中所述。none - 无端点验证。需要 OpenSSL >= 1.0.2。 *类型：枚举值* |
| ssl.certificate.verify_cb               | *         |                                                              |                                                              | 低的   | 用于验证代理证书链的回调。 *类型：请参阅专用 API*            |
| sasl.mechanisms                         | *         |                                                              | 全局搜索应用程序编程接口                                     | 高的   | 用于身份验证的 SASL 机制。支持：GSSAPI、PLAIN、SCRAM-SHA-256、SCRAM-SHA-512、OAUTHBEARER。**注意**：尽管名称如此，但只需配置一种机制。 *类型：字符串* |
| sasl.mechanism                          | *         |                                                              | 全局搜索应用程序编程接口                                     | 高的   | 别名`sasl.mechanisms`：用于身份验证的 SASL 机制。支持：GSSAPI、PLAIN、SCRAM-SHA-256、SCRAM-SHA-512、OAUTHBEARER。**注意**：尽管名称如此，但只能配置一种机制。 *类型：字符串* |
| sasl.kerberos.service.name              | *         |                                                              | 卡夫卡                                                       | 低的   | Kafka 运行的 Kerberos 主体名称，不包括 /hostname@REALM *类型：字符串* |
| sasl.kerberos.principal                 | *         |                                                              | kafka客户端                                                  | 低的   | 此客户端的 Kerberos 主体名称。（Windows 不支持，将使用登录用户的主体）。 *类型：字符串* |
| sasl.kerberos.kinit.cmd                 | *         |                                                              | kinit -R -t “％{sasl.kerberos.keytab}” -k ％{sasl.kerberos.principal} \|\| kinit -t “％{sasl.kerberos.keytab}” -k ％{sasl.kerberos.principal} | 低的   | 用于刷新或获取客户端 Kerberos 票证的 Shell 命令。此命令在客户端创建时以及每次 sasl.kerberos.min.time.before.relogin（0=禁用）时执行。%{config.prop.name} 被相应的配置对象值替换。 *类型：字符串* |
| sasl.kerberos.keytab                    | *         |                                                              |                                                              | 低的   | Kerberos keytab 文件的路径。此配置属性仅用作`sasl.kerberos.kinit.cmd`as中的变量` ... -t "%{sasl.kerberos.keytab}"`。 *类型：字符串* |
| sasl.kerberos.min.time.before.relogin   | *         | 0..86400000                                                  | 60000                                                        | 低的   | 密钥刷新尝试之间的最短时间（以毫秒为单位）。将此属性设置为 0 可禁用自动密钥刷新。 *类型：整数* |
| sasl.username                           | *         |                                                              |                                                              | 高的   | 用于 PLAIN 和 SASL-SCRAM-.. 机制的 SASL 用户名 *类型：字符串* |
| sasl.password                           | *         |                                                              |                                                              | 高的   | 用于 PLAIN 和 SASL-SCRAM-.. 机制的 SASL 密码 *类型：字符串*  |
| sasl.oauthbearer.config                 | *         |                                                              |                                                              | 低的   | SASL/OAUTHBEARER 配置。格式取决于实现，必须进行相应的解析。默认的非安全令牌实现（请参阅https://tools.ietf.org/html/rfc7515#appendix-A.5）可识别空格分隔的名称=值对，有效名称包括 principalClaimName、principal、scopeClaimName、scope 和 lifeSeconds。principalClaimName 的默认值为“sub”，scopeClaimName 的默认值为“scope”，lifeSeconds 的默认值为 3600。范围值为 CSV 格式，默认值为无/空范围。例如：`principalClaimName=azp principal=admin scopeClaimName=roles scope=role1,role2 lifeSeconds=600`。此外，SASL 扩展可以通过 传达给代理`extension_NAME=value`。例如：`principal=admin extension_traceId=123` *类型：字符串* |
| enable.sasl.oauthbearer.unsecure.jwt    | *         | 真假                                                         | 错误的                                                       | 低的   | 如果未设置 oauthbearer_refresh_cb，则启用内置不安全 JWT OAUTHBEARER 令牌处理程序。此内置处理程序仅应用于开发或测试，不应用于生产。 *类型：布尔值* |
| oauthbearer_token_refresh_cb            | *         |                                                              |                                                              | 低的   | SASL/OAUTHBEARER 令牌刷新回调（使用 rd_kafka_conf_set_oauthbearer_token_refresh_cb() 设置，由 rd_kafka_poll() 触发等。当需要刷新客户端的 OAUTHBEARER 令牌时，将触发此回调。另请参阅`rd_kafka_conf_enable_sasl_queue()`。 *类型：参见专用 API* |
| sasl.oauthbearer.method                 | *         | 默认，oidc                                                   | 默认                                                         | 低的   | 设置为“default”或“oidc”以控制要使用的登录方法。如果设置为“oidc”，还必须指定以下属性：`sasl.oauthbearer.client.id`、`sasl.oauthbearer.client.secret`和`sasl.oauthbearer.token.endpoint.url`。 *类型：枚举值* |
| sasl.oauthbearer.client.id              | *         |                                                              |                                                              | 低的   | 应用程序的公共标识符。在授权服务器处理的所有客户端中必须是唯一的。仅在`sasl.oauthbearer.method`设置为“oidc”时使用。 *类型：字符串* |
| sasl.oauthbearer.client.secret          | *         |                                                              |                                                              | 低的   | 客户端机密只有应用程序和授权服务器知道。这应该是足够随机的字符串，不可猜测。仅在`sasl.oauthbearer.method`设置为“oidc”时使用。 *类型：字符串* |
| sasl.oauthbearer.scope                  | *         |                                                              |                                                              | 低的   | 客户端用它来指定对代理的访问请求的范围。仅当`sasl.oauthbearer.method`设置为“oidc”时才使用。 *类型：字符串* |
| sasl.oauthbearer.extensions             | *         |                                                              |                                                              | 低的   | 允许向经纪人提供更多信息。以逗号分隔的键值对列表。例如，“supportFeatureX=true,organizationId=sales-emea”。仅在`sasl.oauthbearer.method`设置为“oidc”时使用。 *类型：字符串* |
| sasl.oauthbearer.token.endpoint.url     | *         |                                                              |                                                              | 低的   | 用于检索令牌的 OAuth/OIDC 发行者令牌端点 HTTP(S) URI。仅当`sasl.oauthbearer.method`设置为“oidc”时使用。 *类型：字符串* |
| plugin.library.paths                    | *         |                                                              |                                                              | 低的   | 要加载的插件库列表（以;分隔）。库搜索路径取决于平台（请参阅 Unix 的 dlopen(3) 和 Windows 的 LoadLibrary()）。如果未指定文件扩展名，则会自动附加特定于平台的扩展名（例如 .dll 或 .so）。 *类型：字符串* |
| interceptors                            | *         |                                                              |                                                              | 低的   | 通过 rd_kafka_conf_interceptor_add_..() 添加的拦截器以及拦截器处理的任何配置。 *类型：参见专用 API* |
| group.id                                | C         |                                                              |                                                              | 高的   | 客户端组 id 字符串。所有共享同一个 group.id 的客户端都属于同一个组。 *类型：字符串* |
| group.instance.id                       | C         |                                                              |                                                              | 中等的 | 启用静态组成员身份。静态组成员能够离开并重新加入配置中的组，`session.timeout.ms`而不会提示组重新平衡。这应该与更大的组合使用，`session.timeout.ms`以避免因暂时不可用（例如进程重新启动）导致的组重新平衡。需要代理版本 >= 2.3.0。 *类型：字符串* |
| partition.assignment.strategy           | C         |                                                              | 范围，循环                                                   | 中等的 | 一个或多个分区分配策略的名称。当选的组长将使用组内所有成员都支持的策略将分区分配给组成员。如果有多个符合条件的策略，则优先顺序由此列表的顺序决定（列表中较早的策略具有较高的优先级）。合作和非合作（急切）策略不得混合使用。可用策略：范围、循环、合作粘性。 *类型：字符串* |
| session.timeout.ms                      | C         | 1..3600000                                                   | 45000                                                        | 高的   | 客户端组会话和故障检测超时。消费者定期发送心跳（heartbeat.interval.ms）以向代理表明其活跃性。如果代理在会话超时内未收到组成员的心跳，则代理将从组中删除消费者并触发重新平衡。允许的范围是使用**代理**配置属性`group.min.session.timeout.ms`和配置的`group.max.session.timeout.ms`。另请参阅`max.poll.interval.ms`。 *类型：整数* |
| heartbeat.interval.ms                   | C         | 1..3600000                                                   | 3000                                                         | 低的   | 组会话保活心跳间隔。 *类型：整数*                            |
| group.protocol.type                     | C         |                                                              | 消费者                                                       | 低的   | 组协议的组协议类型`classic`。注意：目前仅支持 组协议类型`consumer`。 *类型：字符串* |
| group.protocol                          | C         | 经典，消费者                                                 | 经典的                                                       | 高的   | 要使用的组协议。`classic`用于原始协议和`consumer`KIP-848 中引入的新协议。可用协议：经典或消费者。默认值为，但将在下一个版本中`classic`更改为。*类型：枚举值*`consumer` |
| group.remote.assignor                   | C         |                                                              |                                                              | 中等的 | 要使用的服务器端分配器。保留为空以使服务器为组选择合适的分配器。可用的分配器：统一或范围。默认为空 *类型：字符串* |
| coordinator.query.interval.ms           | C         | 1..3600000                                                   | 600000                                                       | 低的   | 查询当前客户端组协调器的频率。如果当前分配的协调器关闭，则配置的查询间隔将除以十，以便在协调器重新分配的情况下更快地恢复。 *类型：整数* |
| max.poll.interval.ms                    | C         | 1..86400000                                                  | 300000                                                       | 高的   | 高级消费者消费消息（例如 rd_kafka_consumer_poll()）的两次调用之间允许的最大时间。如果超过此间隔，则消费者被视为失败，并且该组将重新平衡，以便将分区重新分配给另一个消费者组成员。警告：此时可能无法提交偏移量。注意：建议`enable.auto.offset.store=false`为长时间处理应用程序设置，然后在消息处理*后*明确存储偏移量（使用 offsets_store()） ，以确保在处理完成之前不会自动提交偏移量。每秒检查两次间隔。有关更多信息，请参阅 KIP-62。 *类型：整数* |
| enable.auto.commit                      | C         | 真假                                                         | 真的                                                         | 高的   | 在后台自动定期提交偏移量。注意：将其设置为 false 不会阻止消费者获取之前提交的起始偏移量。要避免此行为，请在对assign()的调用中为每个分区设置特定的起始偏移量。 *类型：布尔值* |
| auto.commit.interval.ms                 | C         | 0..86400000                                                  | 5000                                                         | 中等的 | 消费者偏移提交（写入）到偏移存储的频率（以毫秒为单位）。（0 = 禁用）。此设置由高级消费者使用。 *类型：整数* |
| enable.auto.offset.store                | C         | 真假                                                         | 真的                                                         | 高的   | 自动存储提供给应用程序的最后一条消息的偏移量。偏移量存储是每个分区的下一个要（自动）提交的偏移量的内存存储。 *类型：布尔值* |
| queued.min.messages                     | C         | 1..10000000                                                  | 100000                                                       | 中等的 | librdkafka 尝试在本地消费者队列中维护的每个主题+分区的最小消息数。 *类型：整数* |
| queued.max.messages.kbytes              | C         | 1..2097151                                                   | 65536                                                        | 中等的 | 本地消费者队列中排队预取消息的最大千字节数。如果使用高级消费者，则此设置适用于单个消费者队列，无论分区数是多少。当使用旧式简单消费者或使用单独的分区队列时，此设置适用于每个分区。此值可能会被 fetch.message.max.bytes 超过。此属性的优先级高于queued.min.messages。 *类型：整数* |
| fetch.wait.max.ms                       | C         | 0 ... 300000                                                 | 500                                                          | 低的   | 代理可能等待用 fetch.min.bytes 的消息填充 Fetch 响应的最长时间。 *类型：整数* |
| fetch.queue.backoff.ms                  | C         | 0 ... 300000                                                 | 1000                                                         | 中等的 | 如果当前提取队列阈值（queued.min.messages 或queued.max.messages.kbytes）已超出，则推迟主题+分区的下一个提取请求的时间。如果队列阈值设置较低且应用程序在消息之间遇到较长（~1s）的延迟，则可能需要降低此属性。较低的值可能会增加 CPU 利用率。 *类型：整数* |
| queued.max.messages.kbytes              | C         | 1..1000000000                                                | 1048576                                                      | 中等的 | 从代理获取消息时每个主题+分区请求的初始最大字节数。如果客户端遇到大于此值的消息，它将逐渐尝试增加该值，直到可以获取整个消息。 *类型：整数* |
| max.partition.fetch.bytes               | C         | 1..1000000000                                                | 1048576                                                      | 中等的 | 别名`fetch.message.max.bytes`：从代理获取消息时请求的每个主题+分区的初始最大字节数。如果客户端遇到大于此值的消息，它将逐渐尝试增加该值，直到可以获取整个消息。 *类型：整数* |
| fetch.max.bytes                         | C         | 0..2147483135                                                | 52428800                                                     | 中等的 | 代理应为获取请求返回的最大数据量。消费者按批获取消息，如果获取请求的第一个非空分区中的第一个消息批次大于此值，则仍将返回该消息批次以确保消费者可以取得进展。代理接受的最大消息批次大小通过`message.max.bytes`（代理配置）或`max.message.bytes`（代理主题配置）定义。`fetch.max.bytes`自动向上调整为至少`message.max.bytes`（消费者配置）。 *类型：整数* |
| fetch.min.bytes                         | C         | 1..100000000                                                 | 1                                                            | 低的   | 代理响应的最小字节数。如果 fetch.wait.max.ms 过期，则无论此设置如何，累积的数据都将发送到客户端。 *类型：整数* |
| fetch.error.backoff.ms                  | C         | 0 ... 300000                                                 | 500                                                          | 中等的 | 如果发生获取错误，主题+分区的下一个获取请求将推迟多长时间。 *类型：整数* |
| offset.store.method                     | C         | 无，文件，经纪人                                             | 经纪人                                                       | 低的   | **已弃用的**偏移提交存储方法：“file”- 已弃用：本地文件存储（offset.store.path 等），“broker”- 代理提交存储（需要代理上的 Apache Kafka 0.8.2 或更高版本）。 *类型：枚举值* |
| isolation.level                         | C         | 读取未提交，读取已提交                                       | 读已提交                                                     | 高的   | 控制如何读取事务性写入的消息：`read_committed`- 仅返回已提交的事务性消息。`read_uncommitted`- 返回所有消息，甚至已中止的事务性消息。 *类型：枚举值* |
| consume_cb                              | C         |                                                              |                                                              | 低的   | 消息消费回调（使用 rd_kafka_conf_set_consume_cb() 设置） *类型：参见专用 API* |
| rebalance_cb                            | C         |                                                              |                                                              | 低的   | 在消费者组重新平衡后调用（使用 rd_kafka_conf_set_rebalance_cb() 设置） *类型：参见专用 API* |
| offset_commit_cb                        | C         |                                                              |                                                              | 低的   | 偏移提交结果传播回调。（使用 rd_kafka_conf_set_offset_commit_cb() 设置） *类型：参见专用 API* |
| enable.partition.eof                    | C         | 真假                                                         | 错误的                                                       | 低的   | 每当使用者到达分区末尾时，都会发出 RD_KAFKA_RESP_ERR__PARTITION_EOF 事件。 *类型：布尔值* |
| check.crcs                              | C         | 真假                                                         | 错误的                                                       | 中等的 | 验证已使用消息的 CRC32，确保消息未发生在线或磁盘损坏。此检查会略微增加 CPU 使用率。 *类型：布尔值* |
| client.rack                             | *         |                                                              |                                                              | 低的   | 此客户端的机架标识符。这可以是任何字符串值，表示此客户端的物理位置。它与代理配置相对应`broker.rack`。 *类型：字符串* |
| transactional.id                        | 磷        |                                                              |                                                              | 高的   | 启用事务生产者。transactional.id 用于在进程重新启动时识别相同的事务生产者实例。它允许生产者保证在开始任何新事务之前，与同一生产者的早期实例相对应的事务已经完成，并且任何僵尸实例都被隔离。如果没有提供 transactional.id，则生产者仅限于幂等交付（如果设置了 enable.idempotence）。需要代理版本 >= 0.11.0。 *类型：字符串* |
| transaction.timeout.ms                  | 磷        | 1000..2147483647                                             | 60000                                                        | 中等的 | 事务协调器在主动中止正在进行的事务之前等待生产者发送的事务状态更新的最长时间（以毫秒为单位）。如果此值大于`transaction.max.timeout.ms`代理中的设置，则 init_transactions() 调用将失败并出现 ERR_INVALID_TRANSACTION_TIMEOUT。事务超时会自动调整`message.timeout.ms`和`socket.timeout.ms`，除非明确配置，在这种情况下它们不得超过事务超时（`socket.timeout.ms`必须至少比低 100 毫秒`transaction.timeout.ms`）。如果没有为事务 API 方法提供超时（-1），这也是默认超时值。 *类型：整数* |
| enable.idempotence                      | 磷        | 真假                                                         | 错误的                                                       | 高的   | 当设置为 时`true`，生产者将确保消息成功生成一次，并且按照原始生成顺序生成。当幂等性启用时，以下配置属性会自动调整（如果用户未修改）：（`max.in.flight.requests.per.connection=5`必须小于或等于 5）、`retries=INT32_MAX`（必须大于 0）、、`acks=all`。`queuing.strategy=fifo`如果用户提供的配置不兼容，生产者实例化将失败。 *类型：布尔值* |
| enable.gapless.guarantee                | 磷        | 真假                                                         | 错误的                                                       | 低的   | **实验性**：可能会更改或删除。设置为 时`true`，当一批消息失败时，任何可能导致生成的消息系列出现间隙的错误都将引发致命错误 (ERR__GAPLESS_GUARANTEE) 并停止生产者。由于 而失败的消息`message.timeout.ms`不在此保证范围内。需要`enable.idempotence=true`。 *类型：布尔值* |
| queue.buffering.max.messages            | 磷        | 0..2147483647                                                | 100000                                                       | 高的   | 生产者队列允许的最大消息数。此队列由所有主题和分区共享。值为 0 表示禁用此限制。 *类型：整数* |
| queue.buffering.max.kbytes              | 磷        | 1..2147483647                                                | 1048576                                                      | 高的   | 生产者队列允许的最大消息大小总和。此队列由所有主题和分区共享。此属性的优先级高于queue.buffering.max.messages。 *类型：整数* |
| queue.buffering.max.ms                  | 磷        | 0..900000                                                    | 5                                                            | 高的   | 在构建消息批次 (MessageSet) 以传输到代理之前，等待生产者队列中的消息累积的延迟（以毫秒为单位）。更高的值允许更大、更有效（开销更少、压缩率更高）的消息批次累积，但代价是增加消息传递延迟。 *类型：浮点数* |
| linger.ms                               | 磷        | 0..900000                                                    | 5                                                            | 高的   | 别名`queue.buffering.max.ms`：等待生产者队列中的消息累积，然后再构建消息批次（消息集）以传输到代理的延迟（以毫秒为单位）。更高的值允许更大、更有效（开销更少、压缩率更高）的消息批次累积，但代价是增加消息传递延迟。 *类型：浮点数* |
| message.send.max.retries                | 磷        | 0..2147483647                                                | 2147483647                                                   | 高的   | 重试发送失败消息的次数。**注意：**重试可能会导致重新排序，除非`enable.idempotence`设置为 true。 *类型：整数* |
| retries                                 | 磷        | 0..2147483647                                                | 2147483647                                                   | 高的   | 别名`message.send.max.retries`：重试发送失败消息的次数。**注意：**重试可能会导致重新排序，除非`enable.idempotence`设置为 true。 *类型：整数* |
| retry.backoff.ms                        | *         | 1..300000                                                    | 100                                                          | 中等的 | 重试协议请求之前的退避时间（以毫秒为单位），这是第一次退避时间，将按指数退避，直到重试次数耗尽，上限为 retry.backoff.max.ms。 *类型：整数* |
| retry.backoff.max.ms                    | *         | 1..300000                                                    | 1000                                                         | 中等的 | 重试协议请求之前的最大退避时间（以毫秒为单位），这是指数退避请求允许的最大退避时间。 *类型：整数* |
| queue.buffering.backpressure.threshold  | 磷        | 1..1000000                                                   | 1                                                            | 低的   | 需要对生产者的消息累加器施加背压的未完成的尚未传输的代理请求的阈值。如果尚未传输的请求数等于或超过此数字，则将延迟创建原本会触发的生产请求（例如，根据 linger.ms）。较低的数字会产生更大、更有效的批次。在慢速机器上使用压缩时，较高的值可以改善延迟。 *类型：整数* |
| compression.codec                       | 磷        | 无、gzip、snappy、lz4、zstd                                  | 没有任何                                                     | 中等的 | 用于压缩消息集的压缩编解码器。这是所有主题的默认值，可能会被主题配置属性覆盖`compression.codec`。 *类型：枚举值* |
| compression.type                        | 磷        | 无、gzip、snappy、lz4、zstd                                  | 没有任何                                                     | 中等的 | 别名`compression.codec`：用于压缩消息集的压缩编解码器。这是所有主题的默认值，可能会被主题配置属性覆盖`compression.codec`。 *类型：枚举值* |
| batch.num.messages                      | 磷        | 1..1000000                                                   | 10000                                                        | 中等的 | 一个消息集中批量处理的最大消息数。消息集总大小也受 batch.size 和 message.max.bytes 限制。 *类型：整数* |
| batch.size                              | 磷        | 1..2147483647                                                | 1000000                                                      | 中等的 | 一个消息集中批量处理的所有消息的最大大小（以字节为单位），包括协议帧开销。此限制在第一条消息添加到批处理后应用，无论第一条消息的大小如何，这是为了确保生成超过 batch.size 的消息。消息集总大小也受 batch.num.messages 和 message.max.bytes 的限制。 *类型：整数* |
| delivery.report.only.error              | 磷        | 真假                                                         | 错误的                                                       | 低的   | 仅提供失败消息的传递报告。 *类型：布尔值*                    |
| dr_cb                                   | 磷        |                                                              |                                                              | 低的   | 投递报告回调（使用 rd_kafka_conf_set_dr_cb() 设置） *类型：参见专用 API* |
| dr_msg_cb                               | 磷        |                                                              |                                                              | 低的   | 投递报告回调（使用 rd_kafka_conf_set_dr_msg_cb() 设置） *类型：参见专用 API* |
| sticky.partitioning.linger.ms           | 磷        | 0..900000                                                    | 10                                                           | 低的   | 等待为每个主题分配新的粘性分区的延迟（以毫秒为单位）。默认情况下，设置为 linger.ms 时间的两倍。要禁用粘性行为，请设置为 0。此行为在所有情况下都会影响键为 NULL 的消息，并且在使用 consistent_random 分区程序时会影响键长度为零的消息。否则，这些消息将被随机分配。更高的值可以更有效地对这些消息进行批处理。 *类型：整数* |
| client.dns.lookup                       | *         | use_all_dns_ips，resolve_canonical_bootstrap_servers_only    | 使用所有 dns ips                                             | 低的   | 控制客户端如何使用 DNS 查找。默认情况下，当查找返回主机名的多个 IP 地址时，将尝试连接所有 IP 地址，然后才认为连接失败。这适用于引导服务器和广告服务器。如果将值设置为`resolve_canonical_bootstrap_servers_only`，则每个条目都将被解析并扩展为规范名称列表。**警告**：`resolve_canonical_bootstrap_servers_only`只能与`GSSAPI`（Kerberos）一起使用`sasl.mechanism`，因为这是此配置值的唯一用途。**注意**：此处的默认值与 Java 客户端的默认行为不同，后者仅连接到为主机名返回的第一个 IP 地址。 *类型：枚举值* |
| enable.metrics.push                     | *         | 真假                                                         | 真的                                                         | 低的   | 如果集群具有与此客户端匹配的客户端指标订阅，是否启用将客户端指标推送到集群 *类型：布尔值* |



## 4.2 主题配置属性



| 财产                          | 合同/付款 | 范围                                                         | 默认     | 重要性 | 描述                                                         |
| ----------------------------- | --------- | ------------------------------------------------------------ | -------- | ------ | ------------------------------------------------------------ |
| request.required.acks         | 磷        | -1 .. 1000                                                   | -1       | 高的   | 此字段表示领导者代理在响应请求之前必须从 ISR 代理收到的确认数：*0* = 代理不向客户端发送任何响应/确认，*-1*或*全部*= 代理将阻塞，直到所有同步副本 (ISR) 提交消息。如果 ISR 集合中的 (代理配置) 少于此数量，则`min.insync.replicas`生成请求将失败。 *类型：整数* |
| acks                          | 磷        | -1 .. 1000                                                   | -1       | 高的   | 别名`request.required.acks`：此字段表示领导者代理在响应请求之前必须从 ISR 代理收到的确认数：*0* = 代理不向客户端发送任何响应/确认，*-1*或*全部*`min.insync.replicas`= 代理将阻塞，直到所有同步副本 (ISR) 提交消息。如果ISR 集合中的 (代理配置)少于，则生成请求将失败。 *类型：整数* |
| request.timeout.ms            | 磷        | 1..900000                                                    | 30000    | 中等的 | 生产者请求的确认超时（以毫秒为单位）。此值仅由代理强制执行，并且依赖于`request.required.acks`!= 0。 *类型：整数* |
| message.timeout.ms            | 磷        | 0..2147483647                                                | 300000   | 高的   | 本地消息超时。此值仅在本地强制执行，并限制生成的消息等待成功传递的时间。时间为 0 表示无限。这是 librdkafka 传递消息（包括重试）的最大时间。当重试次数或消息超时超过时，会发生传递错误。`transaction.timeout.ms`如果`transactional.id`配置了，则消息超时会自动调整。 *类型：整数* |
| delivery.timeout.ms           | 磷        | 0..2147483647                                                | 300000   | 高的   | 别名`message.timeout.ms`：本地消息超时。此值仅在本地强制执行，并限制生成的消息等待成功传递的时间。时间为 0 表示无限。这是 librdkafka 传递消息（包括重试）的最大时间。当重试次数或消息超时超过时，会发生传递错误。`transaction.timeout.ms`如果`transactional.id`配置了，则消息超时会自动调整。 *类型：整数* |
| queuing.strategy              | 磷        | fifo, lifo                                                   | 先进先出 | 低的   | **实验性**：可能会更改或删除。**已弃用的**生产者排队策略。FIFO 保留生产顺序，而 LIFO 优先处理新消息。 *类型：枚举值* |
| produce.offset.report         | 磷        | 真假                                                         | 错误的   | 低的   | **已弃用**不再使用。 *类型：布尔值*                          |
| partitioner                   | 磷        |                                                              | 一致随机 | 高的   | 分区器：`random`- 随机分布，`consistent`- 键的 CRC32 哈希值（空键和 NULL 键映射到单个分区），`consistent_random`- 键的 CRC32 哈希值（空键和 NULL 键随机分区），`murmur2`- 与 Java Producer 兼容的 Murmur2 键哈希值（NULL 键映射到单个分区），`murmur2_random`- 与 Java Producer 兼容的 Murmur2 键哈希值（NULL 键随机分区。这在功能上等同于 Java Producer 中的默认分区器。），`fnv1a`- 键的 FNV-1a 哈希值（NULL 键映射到单个分区），`fnv1a_random`- 键的 FNV-1a 哈希值（NULL 键随机分区）。 *类型：字符串* |
| partitioner_cb                | 磷        |                                                              |          | 低的   | 自定义分区器回调（使用 rd_kafka_topic_conf_set_partitioner_cb() 设置） *类型：参见专用 API* |
| msg_order_cmp                 | 磷        |                                                              |          | 低的   | **实验性**：可能会更改或删除。**已弃用的**消息队列排序比较器（使用 rd_kafka_topic_conf_set_msg_order_cmp() 设置）。另请参阅`queuing.strategy`。 *类型：请参阅专用 API* |
| opaque                        | *         |                                                              |          | 低的   | 应用程序不透明（使用 rd_kafka_topic_conf_set_opaque() 设置） *类型：参见专用 API* |
| compression.codec             | 磷        | 无、gzip、snappy、lz4、zstd、继承                            | 继承     | 高的   | 用于压缩消息集的压缩编解码器。inherit = 继承全局 compression.codec 配置。 *类型：枚举值* |
| compression.type              | 磷        | 无、gzip、snappy、lz4、zstd                                  | 没有任何 | 中等的 | 别名`compression.codec`：用于压缩消息集的压缩编解码器。这是所有主题的默认值，可能会被主题配置属性覆盖`compression.codec`。 *类型：枚举值* |
| compression.level             | 磷        | -1..12                                                       | -1       | 中等的 | 配置属性选择的算法的压缩级别参数`compression.codec`。值越高，压缩效果越好，但 CPU 使用率越高。可用范围取决于算法：gzip 为 [0-9]；lz4 为 [0-12]；snappy 仅为 0；-1 = 取决于编解码器的默认压缩级别。 *类型：整数* |
| auto.commit.enable            | C         | 真假                                                         | 真的     | 低的   | **已弃用**[**旧属性：**此属性仅由简单旧消费者使用。使用高级 KafkaConsumer 时，`enable.auto.commit`必须改用全局属性]。如果为 true，则定期提交传递给应用程序的最后一条消息的偏移量。当进程重新启动以从中断处继续时，将使用此已提交的偏移量。如果为 false，则应用程序必须调用`rd_kafka_offset_store()`以存储偏移量（可选）。偏移量将根据 offset.store.method 写入代理或本地文件。 *类型：布尔值* |
| enable.auto.commit            | C         | 真假                                                         | 真的     | 低的   | **已弃用的**别名`auto.commit.enable`：[**旧属性：**此属性仅由简单的旧消费者使用。使用高级 KafkaConsumer 时，`enable.auto.commit`必须改用全局属性]。如果为 true，则定期提交传递给应用程序的最后一条消息的偏移量。当进程重新启动以从中断处继续时，将使用此已提交的偏移量。如果为 false，则应用程序必须调用`rd_kafka_offset_store()`以存储偏移量（可选）。偏移量将根据 offset.store.method 写入代理或本地文件。 *类型：布尔值* |
| auto.commit.interval.ms       | C         | 10..86400000                                                 | 60000    | 高的   | [ **LEGACY PROPERTY：**此设置仅由简单旧式消费者使用。使用高级 KafkaConsumer 时，`auto.commit.interval.ms`必须改用全局属性]。消费者偏移量提交（写入）到偏移量存储的频率（以毫秒为单位）。 *类型：整数* |
| auto.offset.reset             | C         | smallest, earliest, beginning, largest, latest, end, error最小、最早、开始、最大、最晚、结束、错误 | 最大的   | 高的   | 当偏移量存储中没有初始偏移量或所需偏移量超出范围时要采取的操作：'smallest'，'earliest' - 自动将偏移量重置为最小偏移量，'largest'，'latest' - 自动将偏移量重置为最大偏移量，'error' - 触发错误（ERR__AUTO_OFFSET_RESET），通过使用消息并检查'message->err'来检索该错误。 *类型：枚举值* |
| offset.store.path             | C         |                                                              | 。       | 低的   | **已弃用**用于存储偏移量的本地文件路径。如果路径是目录，将根据主题和分区在该目录中自动生成文件名。基于文件的偏移量存储将在未来版本中删除。 *类型：字符串* |
| offset.store.sync.interval.ms | C         | -1..86400000                                                 | -1       | 低的   | **已弃用的**偏移文件的 fsync() 间隔（以毫秒为单位）。使用 -1 可禁用同步，使用 0 可在每次写入后立即同步。基于文件的偏移存储将在未来的版本中被删除。 *类型：整数* |
| offset.store.method           | C         | 文件，经纪人                                                 | 经纪人   | 低的   | **已弃用的**偏移提交存储方法：“file”- 已弃用：本地文件存储（offset.store.path 等），“broker”- 代理提交存储（需要配置“group.id”并在代理上安装 Apache Kafka 0.8.2 或更高版本）。 *类型：枚举值* |
| consume.callback.max.messages | C         | 0 ... 1000000                                                | 0        | 低的   | 一次调用中发送的最大消息数`rd_kafka_consume_callback*()`（0 = 无限制） *类型：整数* |





# 5、ThreadedProducer 和 FutureProducer 区别

`rdkafka` 库提供了两种 Kafka 生产者客户端的实现：`ThreadedProducer` 和 `FutureProducer`。这两种生产者在设计和用途上有所不同，适用于不同的使用场景和编程模型。下面是它们主要的区别：

### ThreadedProducer

- **阻塞式 API**：`ThreadedProducer` 提供了一个基于线程的、阻塞式的 API。当你发送消息时，可以选择同步等待消息被发送（及其确认），或者注册一个回调函数来异步接收发送结果。这种方式更接近于传统的同步编程模型。
- **内部线程**：`ThreadedProducer` 在内部使用一个或多个线程来处理消息的发送和确认。这意味着它会为你管理线程和并发，简化了并发编程的复杂性。
- **回调处理**：通过实现 `ProducerContext` trait 并使用回调函数，你可以处理消息的送达报告（无论成功或失败）。这些回调将在 `ThreadedProducer` 内部的线程中被调用。

### FutureProducer

- **非阻塞式 API**：`FutureProducer` 提供了一个基于 `Future` 的非阻塞式 API。这允许你使用 Rust 的异步/等待语法来处理消息发送，非常适合现代的异步编程模型。
- **与 Rust 异步生态集成**：由于基于 `Future`，`FutureProducer` 可以很容易地与 Rust 的异步运行时（如 `tokio` 或 `async-std`）集成，使得在异步应用程序中处理 Kafka 消息发送变得更加自然和高效。
- **手动轮询**：虽然 `FutureProducer` 提供了非阻塞的 API，但在某些情况下，你可能仍需要手动轮询生产者以确保其内部状态得到更新，特别是在不使用异步运行时的环境中。

### 选择建议

- 如果你的应用程序主要基于同步编程模型，或者你不想深入管理异步任务和运行时，`ThreadedProducer` 可能是更合适的选择。
- 如果你的应用程序已经使用了 Rust 的异步特性，或者你打算构建一个高度可扩展的异步系统，`FutureProducer` 将更加适合。它允许你充分利用 Rust 异步编程的优势，如非阻塞 I/O 操作和轻量级的并发。

总的来说，选择哪种生产者主要取决于你的应用程序架构、编程模型以及个人偏好。两者在功能上是相似的，差异主要在于它们如何被集成和使用在不同类型的应用程序中。

# 6、简单生产者



```
use std::time::Duration;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{ FutureProducer, FutureRecord};

pub async  fn test_kafka_rd(){
    test_producer().await

}

async  fn test_producer(){

    /*
    bootstrap.servers（全局）：指定作为broker主机或主机：端口的CSV列表的初始broker列表。这对于客户端连接到Kafka集群至关重要。
    client.id（全局）：唯一的字符串，用于将客户端标识给broker。
    group.id（消费者）：用于群组管理和偏移量跟踪的消费者群组标识。
    enable.auto.commit（消费者）：如果为true，消费者的偏移量将在后台定期提交。
    auto.commit.interval.ms（消费者）：如果enable.auto.commit为true，消费者偏移量自动提交到Kafka的频率（以毫秒为单位）。
    compression.type（生产者）：生产者生成的所有数据的压缩编解码器。有效值为none、gzip、snappy、lz4、zstd。
    acks（生产者）：生产者在认为请求完成之前，需要领导者接收到的确认数。这影响了发送的记录的持久性。
    linger.ms（生产者）：当多个记录被发送到同一个分区时，生产者将尝试将记录批量组合到更少的请求中。
     */

    //初始化生产者
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        // .set("security.protocol", "SASL_SSL")
        // .set("sasl.mechanisms", "PLAIN")
        // .set("sasl.username", "<update>")
        // .set("sasl.password", "<update>")
        // .set("message.timeout.ms", "5000")
        // .set("queue.buffering.max.ms", "0") // Do not buffer
        // .set("enable.auto.commit", "true") // 自动提交
        // .set("auto.commit.interval.ms", "5000") // 自动提交 default 5000
        .create()
        .expect("Producer creation error");

    for i in 1..=100{
        println!("send message");
        let status = producer.send(
            FutureRecord::to("test_rust")
                .payload(&format!("message {}",i))
                .key(&format!("key {}",i))
                .headers(OwnedHeaders::new().insert(Header{
                    key:"header key",
                    value : Some("header_value")
                })),
            Duration::from_secs(0),//传递 0 秒作为超时时间通常意味着使用默认的超时设置，而不是真正的“无等待”
        ).await;
        println!("Delivery status for message {} received status {:?}", i,status);
    }

    //https://github.com/fede1024/rust-rdkafka/blob/master/examples/simple_producer.rs



}


send message
Delivery status for message 100 received status Ok((0, 199))
```



# 7、线程生产者 回调

https://github.com/abhirockzz/rust-kafka-101/blob/master/part1/src/2_threaded_producer.rs



# 8、追踪详细信息

```
RUST_LOG="librdkafka=trace,rdkafka::client=debug"
```



# 9、批量 写入

在使用 `rdkafka` 库进行批量写入时，你需要考虑几个关键点来有效地实现这一功能。`rdkafka` 是一个 Rust 语言的 Kafka 客户端库，它提供了与 Apache Kafka 集群交互的能力。尽管 `rdkafka` 本身没有直接的“批量写入”API，但你可以通过以下方式高效地发送大量消息：

### 1. 使用 `FutureProducer`

`FutureProducer` 是异步生产者，它允许你发送消息而不阻塞当前线程。你可以在短时间内发送多条消息，`rdkafka` 和 Kafka 本身会处理消息的批处理和缓冲。

```rust
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::config::ClientConfig;

async fn produce_messages(topic: &str, messages: Vec<String>) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let futures = messages.into_iter().map(|message| {
        let record = FutureRecord::to(topic)
            .payload(&message)
            .key(&"some_key");
        producer.send(record, 0)
    });

    let results = futures::future::join_all(futures).await;
    for result in results {
        match result {
            Ok((_partition, _offset)) => println!("Message sent successfully"),
            Err((e, _)) => eprintln!("Error sending message: {:?}", e),
        }
    }
}
```

### 2. 调整生产者配置

Kafka 生产者有多个配置选项可以帮助控制批处理行为，如 `batch.size` 和 `linger.ms`。

- `batch.size` 控制批次的大小，单位是字节。增加这个值可以让生产者缓冲更多的消息进行批量发送。
- `linger.ms` 控制生产者在发送批次之前等待更多消息的最长时间。增加这个值可以提高吞吐量，但会增加消息的延迟。

```rust
let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("batch.size", "16384")
    .set("linger.ms", "5")
    .create()
    .expect("Producer creation error");
```

### 3. 处理错误和重试

当发送消息时，可能会遇到错误，如网络问题或 Kafka 服务不可用。确保你的代码能够妥善处理这些情况，可能包括重试逻辑。

### 4. 关注性能和资源使用

虽然发送大量消息很容易，但也要注意不要过度使用资源或造成网络拥塞。监控你的应用性能和 Kafka 集群的状态，以确保系统的稳定性。

通过上述方法，你可以有效地使用 `rdkafka` 库在 Rust 应用中实现批量写入 Kafka。记得考虑消息的序列化，以及如何最有效地组织和发送这些消息，以达到最佳的性能和资源利用率。



# 10、批量消费

在使用 `rdkafka` 库进行批量消费时，主要考虑如何高效地从 Kafka 主题中拉取并处理大量消息。`rdkafka` 提供了强大的消费者 API，使得批量处理变得可行且高效。以下是一些关键步骤和建议，帮助你实现批量消费：

### 1. 使用 `StreamConsumer`

`StreamConsumer` 是 `rdkafka` 提供的一个基于 Futures 和 Tokio 的消费者接口。它允许你以异步的方式从 Kafka 主题中消费消息，非常适合于处理消息流或进行批量消息处理。

```rust
use futures::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::config::ClientConfig;

async fn consume_messages(topic: &str) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test_group")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[topic]).expect("Can't subscribe to specified topic");

    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload_view::<str>() {
                    println!("Received message: {:?}", payload.unwrap());
                }
                consumer.commit_message(&m, rdkafka::consumer::CommitMode::Async).unwrap();
            },
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}
```

### 2. 批量处理消息

虽然上面的代码示例是逐条处理消息的，你可以通过收集一定数量的消息后再一次性处理它们来实现批量处理。例如，你可以使用 `.take(n)` 方法从流中获取一定数量的消息，或者使用其他方法根据时间或数量累积消息。

### 3. 调整消费者配置

为了更有效地进行批量消费，你可能需要调整一些消费者配置，如 `fetch.min.bytes` 和 `fetch.max.wait.ms`。

- `fetch.min.bytes` 控制服务器在返回给消费者之前在单个 fetch 请求中累积的最小数据量。
- `fetch.max.wait.ms` 控制消费者在不满足 `fetch.min.bytes` 条件时等待数据的最长时间。

通过调整这些设置，你可以控制消费者的拉取行为，以便更高效地进行批量处理。

### 4. 确保健壮的错误处理和重试机制

在批量消费过程中，健壮的错误处理和重试机制至关重要。确保你的应用能够妥善处理可能遇到的各种异常情况，如网络问题、消费者组重新平衡等。

### 5. 监控和调优

监控你的消费者应用和 Kafka 集群的性能指标，可以帮助你识别瓶颈和进行必要的调优。根据应用的实际表现，适时调整消费者配置和批量处理策略。

通过以上步骤，你可以有效地利用 `rdkafka` 实现 Kafka 消息的批量消费。记得根据你的具体需求和环境调整代码和配置，以达到最佳的性能。



# 11、stream 和recv的区别

在 `rdkafka` 库中，`consumer.recv()` 和使用 `stream` 方法来接收消息是两种不同的接收消息的方式。下面是它们之间的一些主要区别：

### 使用 `consumer.recv().await`

当你使用 `consumer.recv().await` 方法时，你正在等待并异步接收单个消息。这个方法将会挂起当前任务直到一个消息到达或者发生错误。这种方式允许你以非常直接和简单的方式处理每条消息，但它不是基于流的，因此不适用于处理消息流或实现复杂的异步流控制。

```rust
async fn consume_messages(consumer: &StreamConsumer) {
    loop {
        match consumer.recv().await {
            Ok(message) => {
                // 处理消息
            },
            Err(e) => {
                // 处理错误
            }
        }
    }
}
```

### 使用 `stream` 方法

使用 `stream` 方法时，你可以将消费者转换为 `Stream`，这是 Rust 中的一个异步迭代器。这样，你可以使用 `futures` 和 `tokio` 提供的所有异步流工具和组合器，比如 `map`、`filter`、`for_each`、`next` 等。这种方式非常适合于在消息处理中实现复杂的逻辑，或者当你想要以声明式的方式处理整个消息流时。

```rust
use futures::stream::StreamExt;

async fn consume_and_process_messages(stream_consumer: &StreamConsumer) {
    let mut message_stream = stream_consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                // 处理消息
            },
            Err(e) => {
                // 处理错误
            }
        }
    }
}
```

### 总结

- 使用 `consumer.recv().await` 时，你每次只处理一条消息。它是一个简单的方法，适用于不需要复杂流处理的情况。
- 使用 `stream` 方法时，你可以利用 Rust 异步生态系统中的强大工具来处理消息流。它允许你以更加函数式的方式来处理消息，适合于需要复杂处理或者批量处理的场景。

根据你的具体需求，你可以选择最适合你场景的方法来消费 Kafka 消息。如果你需要简单的处理，可能 `recv().await` 就足够了；如果你需要更复杂的处理或者想利用异步流的优势，那么使用 `stream` 方法可能更合适。