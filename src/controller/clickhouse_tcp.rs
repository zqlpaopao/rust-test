/// https://blog.csdn.net/jdcdev_/article/details/129076829
/// clickhouse https://github.com/loyd/clickhouse.rs?tab=readme-ov-file#insert-a-batch
/// clickhouse tcp https://github.com/suharev7/clickhouse-rs/blob/async-await/examples/simple.rs?tab=readme-ov-file
#[allow(unused)]
use clickhouse_rs::Pool;
use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MyRow<'a> {
    pub customer_id: u32,
    pub amount: u32,
    pub account_name: &'a str,
}
pub async fn test_ck() -> Result<(), Box<dyn Error>> {
    // let ddl = r"
    //     CREATE TABLE IF NOT EXISTS payment (
    //         customer_id  UInt32,
    //         amount       UInt32,
    //         account_name Nullable(FixedString(3))
    //     ) Engine=Memory";
    //
    // let block = Block::new()
    //     .column("customer_id", vec![1_u32, 3, 5, 7, 9])
    //     .column("amount", vec![2_u32, 4, 6, 8, 10])
    //     .column(
    //         "account_name",
    //         vec![Some("foo"), None, None, None, Some("bar")],
    //     );

    let pool = Pool::new("tcp://:@0.0.0.0:9000/test");

    let mut client = pool.get_handle().await.unwrap();
    // client.execute(ddl).await?;
    // client.insert("payment", block).await?;
    let block = client
        .query("SELECT * FROM payment")
        .fetch_all()
        .await
        .unwrap();

    for row in block.rows() {
        let id: u32 = row.get("customer_id").unwrap();
        println!("{:?}", id);

        // let amount: u32 = row.get("amount").unwrap();
        // let name: Option<&str> = row.get("account_name").unwrap();
        // println!("Found payment {}: {} {:?}", id, amount, name);
    }

    Ok(())
}
