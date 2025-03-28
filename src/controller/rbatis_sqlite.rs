use rbatis::{crud, RBatis};
use rbdc_sqlite::SqliteDriver;
use serde_json::json;

//// https://mp.weixin.qq.com/s/QXlQfWkvRbxe4HvpQ6UFZw

const DB_URL_SQLITE: &str = "sqlite://target/sqlite.db";

pub async fn test_sqlite() {
    insert().await
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ActivitySqlite {
    id: Option<i32>,
    name: Option<String>,
    age: Option<i32>,
}

//custom table name
//crud!(Activity {},"activity");
crud!(ActivitySqlite {}); // impl_insert!($table {}) + impl_select!($table {}) + impl_update!($table {}) + impl_delete!($table {});

async fn insert() {
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, DB_URL_SQLITE).unwrap();
    rb.get_pool().unwrap().set_max_open_conns(10).await;
    rb.exec(
        "CREATE TABLE IF NOT EXISTS activity_sqlite (
                  id    INTEGER PRIMARY KEY,
                  name  TEXT NOT NULL,
                  age   INTEGER NOT NULL)",
        vec![],
    )
    .await
    .unwrap();
    let table = ActivitySqlite {
        id: Some(2),
        name: Some("2".into()),
        age: Some(5),
    };
    // let cli = BaitsPool::get();
    // let data = Activity::insert(rb, &table).await;
    // println!("insert = {}", json!(data));
    //insert = {"Ok":{"last_insert_id":0,"rows_affected":1}}

    //batch
    let tables = [table.clone(), {
        let mut t3 = table.clone();
        t3.id = Some(3);
        t3
    }];
    let data = ActivitySqlite::insert_batch(&rb, &tables, 10).await;
    println!("insert_batch = {}", json!(data));
    //insert_batch = {"Ok":{"last_insert_id":0,"rows_affected":2}}

    let data = ActivitySqlite::select_by_column(&rb, "id", 2).await;
    println!("select_by_column = {}", json!(data));
}
