// https://mp.weixin.qq.com/s/fPMIcrFVEvWPlCKBCzLSzg

// use bytes::BytesMut;
// use std::net::TcpStream;
// use tokio::io::AsyncReadExt;
//
// pub fn test_stream_tidy() {}
//
// // 零拷贝
// async fn read_frames(mut stream: TcpStream) {
//     // 预分配 8KB 的缓冲区
//     let mut buffer = BytesMut::with_capacity(8 * 1024);
//
//     loop {
//         //  直接读取数据到缓冲区，无需拷贝
//         let bytes_read = stream.read(&mut buffer).await.unwrap();
//         if bytes_read == 0 {
//             break;
//         }
//
//         //原地处理缓冲区，避免拷贝
//         process_buffer(&mut buffer).await;
//     }
// }
//
// async fn process_buffer(buf: &mut BytesMut) {
//     while let Some(frame_length) = parse_frame_size(buf) {
//         // 分割出单条记录的准确字节数
//         let mut frame = buf.split_to(frame_length);
//
//         //现在 frame 包含一条完整记录的原始数据
//         handle_record(frame).await;
//     }
// }
//
// fn parse_frame_size(buf: &BytesMut) -> Option<usize> {
//     if buf.len() < 4 {
//         return None;
//     }
//
//     // 读取前 4 个字节作为网络字节序的 u32
//     let size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
//
//     if buf.len() >= size + 4 {
//         Some(size + 4)
//     } else {
//         None
//     }
// }
//
// use simd_json::{borrowed::Value, to_writer};
//
// async fn handle_record(mut frame: BytesMut) {
//     // 跳过 4 字节的长度头，获取 JSON 负载
//     let json_bytes = &frame[4..];
//
//     // 原地解析 JSON，simd-json 直接从切片借用数据
//     let mut deserialized: Value<'_> = simd_json::to_borrowed_value(json_bytes).unwrap();
//
//     // 应用转换：将 "measurement" 字段乘以 1.23
//     if let Some(measurement) = deserialized.get_mut("measurement") {
//         if let Some(orig) = measurement.as_f64() {
//             *measurement = Value::from_f64(orig * 1.23).unwrap();
//         }
//     }
//
//     // 序列化回新的 BytesMut 以发送到 Kafka
//     let mut out_buffer = BytesMut::with_capacity(json_bytes.len() + 16);
//
//     // 预留 4 字节长度头的空间
//     out_buffer.extend_from_slice(&0u32.to_be_bytes());
//
//     // 直接将更新后的 JSON 写入缓冲区
//     to_writer(&mut out_buffer.writer(), &deserialized).unwrap();
//
//     // 计算新长度并填充前 4 个字节
//     let total_len = (out_buffer.len() - 4) as u32;
//     out_buffer[0..4].copy_from_slice(&total_len.to_be_bytes());
//
//     // 异步发送到 Kafka
//     send_to_kafka(out_buffer).await;
// }
//
// use rdkafka::producer::{FutureProducer, FutureRecord};
//
// async fn send_to_kafka(buffer: BytesMut) {
//     // 将 BytesMut 转换为 Bytes 以满足 API 要求
//     let payload = buffer.freeze();
//
//     // 使用名为 "processed_events" 的主题
//     let record: FutureRecord<'_, _, _> = FutureRecord::to("processed_events")
//         .payload(payload)
//         .key(""); // 本例中不使用键
//
//     // 使用 0 毫秒超时发送；依赖代理的背压机制
//     let produce_future = PRODUCER.send(record, 0);
//
//     // 等待发送结果；记录任何错误
//     match produce_future.await {
//         Ok(Ok(_)) => {}
//         Ok(Err((e, _))) => eprintln!("Kafka 错误: {:?}", e),
//         Err(e) => eprintln!("Kafka 超时: {:?}", e),
//     }
// }
//
// // 创建一个在启动时初始化一次的静态生产者
// lazy_static::lazy_static! {
//     staticref PRODUCER: FutureProducer = {
//         letmut config = rdkafka::ClientConfig::new();
//         config.set("bootstrap.servers", "kafka-broker:9092");
//         config.set("queue.buffering.max.messages", "1000000");
//         config.set("queue.buffering.max.kbytes", "1048576");
//         config.set("batch.num.messages", "1000");
//         config.set("linger.ms", "5");
//         config.set("enable.idempotence", "true");
//         config.create().expect("生产者创建失败")
//     };
// }
