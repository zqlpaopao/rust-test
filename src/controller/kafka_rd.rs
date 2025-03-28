#![allow(unused)]

// https://mp.weixin.qq.com/s/cGaz3labguoPpXEywjuv1g
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{
    BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer,
};
use rdkafka::error::KafkaResult;
use rdkafka::message::{DeliveryResult, Header, OwnedHeaders};
use rdkafka::producer::{
    BaseRecord, FutureProducer, FutureRecord, Producer, ProducerContext, ThreadedProducer,
};
use rdkafka::util::Timeout::Never;
use rdkafka::{ClientContext, Message, TopicPartitionList};
use serde::Deserialize;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
pub async fn test_kafka_rd() {
    //简单生产者
    test_producer().await;

    //生产者回调
    // test_producer_call_back().await;

    //发送json数据

    // tokio::task::spawn(async {
    //     test_json_payload().await;
    // });

    //普通消费者
    // test_customer().await;
    //
    // std::thread::sleep(std::time::Duration::from_secs(5))
}
/*******************************************asynchronous_processing ****************************************************/

/*******************************************消费者 ****************************************************/

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
    }
}

use crate::controller::test_tokio::tokio;
use rdkafka::message::Headers;
use rdkafka::util::{get_rdkafka_version, AsyncRuntime};

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;
use futures::StreamExt;
async fn consume_and_print() {
    let context = CustomContext;

    // let consumer: LoggingConsumer = ClientConfig::new()
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "group_ids1")
        .set("bootstrap.servers", "localhost:9092")
        //enable.partition.eof是一个配置选项，
        // 用于控制RDKafka库是否在消费者到达分区末尾时生成一个特殊的RD_KAFKA_RESP_ERR__PARTITION_EOF事件。
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest") //最早的
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        // .create_with_context(context)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["test_rust1"])
        .expect("Can't subscribe to specified topics");
    //

    /// 这段代码使用的是Rust的rdkafka库，用于与Apache Kafka交互。
    //
    // stream()方法和recv()方法都是用来从Kafka消费者中接收消息的，但它们有以下区别：
    //
    // 异步性：stream()方法返回一个异步的消息流（Stream），可以使用.next().await等待下一个消息的到来。
    // 它是基于Rust的async/await机制实现的，适合在异步上下文中使用。
    //
    // 而recv()方法是一个阻塞式的方法，会一直等待直到有新的消息到来，或者遇到错误才返回。
    // 它不适合在异步上下文中使用。

    // 处理方式：stream()方法将消息作为一个流处理，使用者需要手动处理每个消息，包括解析、打印、提交等操作。

    // recv()方法则是每次只接收一个消息，并返回这个消息的详细信息，
    // 包括key、payload、headers等。使用者可以根据需要进行进一步的处理。

    // 性能：在高并发场景下，stream()方法可能会更有效率，
    // 因为它可以利用Rust的async/await特性来并行处理多个消息。而recv()方法由于是阻塞式的，可能会导致性能瓶颈。
    // 使用场景：stream()方法通常用于需要实时处理大量消息的场景，例如在一个异步的Web服务中，或者在一个数据处理管道中。
    // recv()方法则更适合用于简单的命令行工具或者测试程序中，或者在不需要实时处理消息的场景中。
    // 总的来说，stream()方法提供了更多的灵活性和并行性，而recv()方法则更加简单易用。选择哪种方法取决于你的具体需求和使用场景。
    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload_view::<str>() {
                    println!("Received message: {:?}", payload.unwrap());
                }
                consumer
                    .commit_message(&m, rdkafka::consumer::CommitMode::Async)
                    .unwrap();
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        println!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

async fn test_customer() {
    consume_and_print().await
}

/*******************************************发送json数据 ****************************************************/
pub struct AsyncStdRuntime;

impl AsyncRuntime for AsyncStdRuntime {
    type Delay = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn spawn<T>(task: T)
    where
        T: Future<Output = ()> + Send + 'static,
    {
        async_std::task::spawn(task);
    }

    fn delay_for(duration: Duration) -> Self::Delay {
        Box::pin(async_std::task::sleep(duration))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    email: String,
}

async fn test_json_payload() {
    let producer: FutureProducer<_, AsyncStdRuntime> = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        // .set("produce.offset.report", "true")
        .set("message.max.bytes", (1 * 1024 * 1024).to_string())
        .create()
        .expect("Producer creation error");

    for i in 1..=1000000 {
        println!("send message");
        let user = User {
            id: i,
            email: format!("user-{}@foobar.com", i),
        };

        // let user_json = serde_json::to_string_pretty(&user).expect("json serialization failed");
        let user_json = serde_json::to_vec_pretty(&user).expect("json serialization failed");

        let delivery_status = producer
            .send(
                FutureRecord::to("test_rust1")
                    .key("key")
                    .payload(&user_json),
                Never,
            )
            .await;

        if let Err((e, _)) = delivery_status {
            eprintln!("unable to send message: {}", e);
            std::process::exit(1);
        }
    }
}

/*******************************************生产者回调 ****************************************************/

struct ProducerCallbackLogger;
// impl FutureProducer for ProducerCallbackLogger{}

impl ClientContext for ProducerCallbackLogger {}

impl ProducerContext for ProducerCallbackLogger {
    type DeliveryOpaque = ();

    fn delivery(
        &self,
        delivery_result: &DeliveryResult<'_>,
        delivery_opaque: Self::DeliveryOpaque,
    ) {
        println!("1111");
        let dr = delivery_result.as_ref();
        match dr {
            Ok(msg) => {
                let key: &str = msg.key_view().unwrap().unwrap();
                println!(
                    "produced message with key {} in offset {} of partition {}",
                    key,
                    msg.offset(),
                    msg.partition()
                )
            }
            Err(producer_err) => {
                let key: &str = producer_err.1.key_view().unwrap().unwrap();

                println!(
                    "failed to produce message with key {} - {}",
                    key, producer_err.0,
                )
            }
        }
    }
}
//生产者回调
async fn test_producer_call_back() {
    let producer: ThreadedProducer<ProducerCallbackLogger> = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        //for auth
        /*.set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", "<update>")
        .set("sasl.password", "<update>")*/
        .create_with_context(ProducerCallbackLogger {})
        .expect("invalid producer config");

    for i in 1..100 {
        println!("sending message");

        producer
            .send(
                BaseRecord::to("rust")
                    .key(&format!("key-{}", i))
                    .payload(&format!("value-{}", i)),
            )
            .expect("failed to send message");

        //producer.flush(Duration::from_secs(3));
        //println!("flushed message");
        std::thread::sleep(Duration::from_secs(3));
    }

    //
    // //初始化生产者
    // let producer: FutureProducer<ProducerCallbackLogger> = ClientConfig::new()
    //     .set("bootstrap.servers", "localhost:9092")
    //     // .set("security.protocol", "SASL_SSL")
    //     // .set("sasl.mechanisms", "PLAIN")
    //     // .set("sasl.username", "<update>")
    //     // .set("sasl.password", "<update>")
    //     // .set("message.timeout.ms", "5000")
    //     // .set("queue.buffering.max.ms", "0") // Do not buffer
    //     // .set("enable.auto.commit", "true") // 自动提交
    //     // .set("auto.commit.interval.ms", "5000") // 自动提交 default 5000
    //     .set_log_level(RDKafkaLogLevel::Debug)
    //     .create_with_context(ProducerCallbackLogger)
    //     .expect("Producer creation error");
    //
    // for i in 1..=100{
    //     println!("send message");
    //     let status = producer.send(
    //         FutureRecord::to("test_rust")
    //             .payload(&format!("message {}",i))
    //             .key(&format!("key {}",i))
    //             .headers(OwnedHeaders::new().insert(Header{
    //                 key:"header key",
    //                 value : Some("header_value")
    //             })),
    //         Duration::from_secs(0),//传递 0 秒作为超时时间通常意味着使用默认的超时设置，而不是真正的“无等待”
    //     )
    //         .await;
    //     println!("Delivery status for message {} received status {:#?}", i,status);
    //
    //     //  调用回调
    //     producer.flush(Duration::from_secs(3)).expect("TODO: panic message");
    //     println!("flushed message");
    // }
}

/*******************************************普通发送 ****************************************************/
//普通发送

async fn test_producer() {
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
        .set("bootstrap.servers", "localhost:9093")
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

    for i in 1..=100 {
        println!("send message");
        let status = producer
            .send(
                FutureRecord::to("test_rust")
                    .payload(&format!("message {}", i))
                    .key(&format!("key {}", i))
                    .headers(OwnedHeaders::new().insert(Header {
                        key: "header key",
                        value: Some("header_value"),
                    })),
                Duration::from_secs(0), //传递 0 秒作为超时时间通常意味着使用默认的超时设置，而不是真正的“无等待”
            )
            .await;
        println!(
            "Delivery status for message {} received status {:#?}",
            i, status
        );
    }

    //send message
    // Delivery status for message 100 received status Ok(
    //     (
    //         0,
    //         299,//消息id
    //     ),
    // )

    // await 发送
    //https://github.com/fede1024/rust-rdkafka/blob/master/examples/simple_producer.rs
}
