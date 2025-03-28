#![allow(unused)]

use lazy_static::lazy_static;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::time::Duration;

const DB_URL: &str = "mysql://root:password@127.0.0.1:3306/test";

/// 自定义连接池结构体
pub struct MyPool(Option<Pool<MySql>>);
impl Default for MyPool {
    /// 创建连接池
    fn default() -> Self {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .max_lifetime(Duration::from_secs(3600 * 6))
            .connect_lazy(DB_URL)
            .expect("init mysql pool is error");

        MyPool(Some(pool))
    }
}

/// 实现 Default trait
impl MyPool {
    pub fn get<'a>() -> &'a Pool<MySql> {
        lazy_static! {
            // 此处表示声明全局可变 HashMap
            pub static ref POOL: MyPool = MyPool::default();
        }
        POOL.0.as_ref().unwrap()
    }
}
