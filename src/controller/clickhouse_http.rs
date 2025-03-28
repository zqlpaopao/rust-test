//
// use clickhouse::Client;
// use serde::Deserialize;
// use clickhouse::Row;
// // docker run -d --name clickhouse --ulimit nofile=262144:262144 \
// // -p 8123:8123 -p 9000:9000 -p 9009:9009 --privileged=true \
// // -v /Users/sss/Documents/docker/clickhouse/log:/var/log/clickhouse-server \
// // -v /Users/sss/Documents/docker/clickhouse/data:/var/lib/clickhouse clickhouse/
// //  clickhouse/clickhouse-server:22.2.3.5
// //https://crates.io/crates/clickhouse
// #[derive(Row, Deserialize, Debug)]
// struct MyRow<'a> {
//     customer_id: u32,
//     amount: u32,
//     account_name: &'a str,
// }
//
//
//

// pub async fn test_clickhouse_http(){
//     // admin:123456@0.0.0.0:9090/monitor
//     let client = Client::default()
//         .with_url("http://localhost:8123")
//         .with_user("")
//         .with_password("")
//         .with_database("test");
//
//     let mut cursor = client
//         .query("SELECT * FROM payment where customer_id = ?")
//         .bind(1)
//
//         .fetch::<MyRow<'_>>().unwrap();
//
//     while let Some(row) = cursor.next().await.unwrap() {
//         println!("{:?}",row);
//     }
//     //MyRow { customer_id: 1, amount: 2, account_name: "zhangsan" }
// }
