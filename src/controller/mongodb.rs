#![allow(unused)]
// use mongodb::bson::doc;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::{options::ClientOptions, Client};
pub async fn test_mongodb() {
    let address = "mongodb://localhost:27017/?maxIdleTimeMS=50000&maxPoolSize=5&minPoolSize=3&maxConnecting=5";
    //连接 mongodb
    // let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client_options = ClientOptions::parse(address).await.unwrap();

    let client = Client::with_options(client_options).unwrap();

    //创建结合
    // create_collection(
    //     &client,
    //     "mydatabase",
    //     "mycollection"
    // ).await

    //插入文档
    // insert_document(
    //     &client,
    //         "mydatabase",
    //         "mycollection"
    // ).await

    //检索文档
    get_document(&client, "mydatabase", "mycollection").await;

    // //删除文档
    // del_doc(
    //     &client,
    //     "mydatabase",
    //     "mycollection"
    // ).await;
}

/// 创建集合
async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(coll_name, None).await.unwrap()
}

/// 插入文档
async fn insert_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection::<Book>(coll_name);

    // coll.insert_one()

    //第一种
    //必须是     let coll = db.collection::<Document>(coll_name);
    let docs = vec![
        doc! { "title": "1984", "author": "George1111 Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    //第二种
    //必须用这种     let coll = db.collection::<Book>(coll_name);
    let docs = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
        },
    ];
    let res = coll.insert_many(docs, None).await.unwrap();
    println!("{:?}", res)
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

//检索文档
async fn get_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);

    //第一种检索方式
    let coll = db.collection::<Book>(coll_name);
    let filter = doc! {  "title":"1984" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = coll.find(filter, find_options).await.unwrap();
    while let Some(book) = cursor.try_next().await.unwrap() {
        println!("title: {}", book.title);
        println!("title: {:?}", book);
    }

    //第二种
    let coll = db.collection::<Document>(coll_name);
    let filter = doc! {  "title":"1984" };

    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = coll.find(filter, find_options).await.unwrap();
    while let Some(book) = cursor.try_next().await.unwrap() {
        println!("title: {:?}", book);
        println!("title: {:?}", book.get("title"));
        println!("title: {:?}", book.get("title").unwrap().to_string());
    }
    //获取全部
    let coll = db.collection::<Document>(coll_name);
    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = coll.find(filter, find_options).await.unwrap();
    while let Some(book) = cursor.try_next().await.unwrap() {
        println!("all title: {:?}", book);
        println!("all title: {:?}", book.get("title"));
        println!("all title: {:?}", book.get("title").unwrap().to_string());
    }
}

/// 删除文档
async fn del_doc(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection::<Document>(coll_name);
    let filter = doc! {
        "name": "John"
    };
    //DeleteResult { deleted_count: 0 } 不存在的

    let filter = doc! {
        "title": "The Great Gatsby"
    };
    //DeleteResult { deleted_count: 1 }
    // coll.delete_many()
    let res = coll.delete_one(filter, None).await.unwrap();
    println!("{:?}", res);

    //第二种 这种需指定所有的 不适合
    // let db = client.database(db_name);
    // let coll = db.collection::<Book>(coll_name);
    // let filter = doc! {
    //     "name": "John"
    // };
    //DeleteResult { deleted_count: 0 } 不存在的

    // let filter = doc! {
    //     Book{
    //
    //     title: "".to_string(),author: "".to_string(),}
    // };
    // //DeleteResult { deleted_count: 1 }
    // // coll.delete_many()
    // let res = coll.delete_one(filter,None).await.unwrap();
    // println!("{:?}",res)
}
