#![allow(unused)]
// https://mp.weixin.qq.com/s/3pivqhoyjFqJVjJbKyLCbA

use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;

trait DistributedStore {
    type Key: Clone + Send + Sync + 'static;
    type Value: Clone + Send + Sync + 'static;
    type Error: Error + Send + Sync + 'static;

    type SetFuture<'a>: Future<Output = Result<(), Self::Error>> + Send + 'a
    where
        Self: 'a;
    fn set(&self, key: Self::Key, value: Self::Value) -> Self::SetFuture<'_>;
}

// Implementation for Redis
struct RedisStore {
    client: redis::Client,
}

// impl DistributedStore for RedisStore {
//     type Key = String;
//     type Value = Vec<u8>;
//     type Error = redis::RedisError;
//     type SetFuture<'a> = impl Future<Output = Result<(), Self::Error>> + Send + 'a;
//     fn set(&self, key :String, value:Self::Value) -> Self::SetFuture<'_> {
//         async move {
//             let mut conn = self.client.get_async_connection().await?;
//             redis::cmd("SET")
//                 .arg(key)
//                 .arg(value)
//                 .query_async(&mut conn)
//                 .await
//         }
//     }
// }

//
// //GAT 高级类型级状态机
//
// struct Initialized;
// struct InTransaction;
// struct Committed;
// struct Rolled;
//
// //Transaction type with phantom state
// struct Transactional<'conn,State>{
//     connection : &'conn mut redis::Connection,
//     _state : PhantomData<State>
// }
//
// impl<'conn> Transaction<'conn, Initialized> {
//     fn new(conn: &'conn mut Connection) -> Self {
//         Transaction {
//             connection: conn,
//             _state: PhantomData,
//         }
//     }
//
//     fn begin(self) -> Result<Transaction<'conn, InTransaction>, DbError> {
//         self.connection.execute("BEGIN TRANSACTION")?;
//         Ok(Transaction {
//             connection: self.connection,
//             _state: PhantomData,
//         })
//     }
// }
