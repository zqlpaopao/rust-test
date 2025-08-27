#![allow(unused)]
use batched::batched;
use futures::executor::block_on;
use rbatis::{crud, RBatis};
use rbdc_mysql::MysqlDriver;
use serde_derive::{Deserialize, Serialize};
use crate::controller::rbatis::BaitsPool;
// use tracing_opentelemetry


/// https://mp.weixin.qq.com/s/NJt8OjY1u1KcF_x6viOEEQ
#[derive(Serialize,Deserialize)]
pub struct Test1  {
   pub id: usize,
    pub name: String,
}

crud!(Test1 {});


const DB_URL: &str = "mysql://root:123456@127.0.0.1:3306/test";
pub async  fn test_batched(){
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Trace),
    );
    let rb = RBatis::new();
    rb.init(MysqlDriver {}, DB_URL).expect("mysql init fail");

    let pool = rb.get_pool().unwrap();
    pool.set_conn_max_lifetime(Some(std::time::Duration::from_secs(86400)))
        .await;
    pool.set_timeout(Some(std::time::Duration::from_secs(6400)))
        .await;
    pool.set_max_idle_conns(3).await;
    pool.set_max_open_conns(10).await;

}


#[batched(window = 100, limit = 1000)]
#[tracing_opentelemetry]
pub async fn insert(client :&RBatis){
    for i in 1..=10{
        Test1::insert(client, &Test1 { id: 0, name: i.to_string() }).await.unwrap();
    }
}