

# 链接

[create](https://crates.io/crates/mongodb)

[文档](https://zhuanlan.zhihu.com/p/692665114)

# 1、Docker安装mongodb

​	https://www.runoob.com/docker/docker-install-mongodb.html

```
docker run -d -p 27017:27017 --name my-mongo-container mongo
```



# 2、 使用

添加依赖

```
[dependencies]
mongodb = "2.3.1"
```



启动参数

```
mongodb+srv://<cluster-url>/<dbname>?w=majority
mongodb://localhost:27018,localhost:27019,localhost:27020/?replicaSet=repl
mongodb://example.com/?maxIdleTimeMS=50000&maxPoolSize=5&minPoolSize=3&maxConnecting=5
```



1. 导入必要的库。 打开项目目录中的 `src/main.rs` 文件，并添加以下行到文件顶部以导入必要的库：

```text
extern crate mongodb;
use mongodb::bson::doc;
use mongodb::{Client, options::ClientOptions};
```

1. 连接到你的 MongoDB 服务器。 在你的 `main` 函数中添加以下代码以连接到你的 MongoDB 服务器：

```text
let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
let client = Client::with_options(client_options).unwrap();
```

这将使用连接字符串创建一个新的 `ClientOptions` 对象，然后用 `ClientOptions` 对象创建一个新的 `Client` 对象。

恭喜你，现在你的 Rust 项目已经设置好使用官方的 MongoDB Rust 驱动程序了。



# 3、创建数据库

在使用 MongoDB 之前，你需要有一个数据库来存储集合。在本节中，我将向你展示如何使用 MongoDB Compass 创建一个新的数据库。

打开 MongoDB Compass 并点击屏幕左上角的"Connect"按钮。在 "New Connection"窗口中，输入你的 MongoDB 实例的连接详情。这包括主机名、端口号和必要的认证详情。点击 "Connect"以建立与你的 MongoDB 实例的连接。在左侧导航窗格中，点击"Databases"查看现有数据库列表。点击 "Databases"窗口左上角的"Create Database"按钮。输入你的新数据库名称（例如"mydatabase"）并点击"Create"。

恭喜你，你已经使用 MongoDB Compass 创建了一个新的数据库！现在你可以开始创建集合并向数据库添加文档了。



```
进入容器
docker exec -it 802d182d7cfb bash

使用
mongosh

show dbs
admin   40.00 KiB
config  12.00 KiB
local   40.00 KiB


创建
 use mydatabase
就直接创建了
test> use mydatabase
switched to db mydatabase
mydatabase> 

```



# 4、创建集合

```
async fn  create_collection(client: &Client, db_name: &str, coll_name: &str){
        let db = client.database(db_name);
        db.create_collection(coll_name,None).await.unwrap()
}

   create_collection(
        &client,
        "mydatabase",
        "mycollection"
    ).await
```

这段代码将创建一个新的 `Client` 对象，并调用 `create_collection` 函数来在 `mydatabase` 数据库中创建一个名为"mycollection"的新集合。

恭喜你，现在你已经使用 Rust 在你的 MongoDB 数据库中创建了一个新的集合。

查看集合

```
mydatabase> show collections
mycollection
```

已经创建出来了





# 5、创建文档

```
/// 插入文档
/// 插入文档
async  fn insert_document(client : & Client,db_name:&str,coll_name:&str){
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
     let res = coll.insert_many(docs,None).await.unwrap();
    println!("{:?}",res)
}

InsertManyResult { inserted_ids: {2: ObjectId("6655b897c608f5ef7a046fa1"), 0: ObjectId("6655b897c608f5ef7a046f9f"), 1: ObjectId("6655b897c608f5ef7a046fa0")} }


 insert_document(
        &client,
            "mydatabase",
            "mycollection"
    ).await
```



Cli 查看文档

```
mydatabase> db.mycollection.find()
[
  {
    _id: ObjectId('6655b897c608f5ef7a046f9f'),
    title: '1984',
    author: 'George Orwell'
  },
  {
    _id: ObjectId('6655b897c608f5ef7a046fa0'),
    title: 'Animal Farm',
    author: 'George Orwell'
  },
  {
    _id: ObjectId('6655b897c608f5ef7a046fa1'),
    title: 'The Great Gatsby',
    author: 'F. Scott Fitzgerald'
  }
]
```



# 6、检索文档

```

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}


//检索文档
async  fn get_document(client : & Client,db_name:&str,coll_name:&str){
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
}


 get_document(
        &client,
                "mydatabase",
                "mycollection"
    ).await
```



```
title: 1984
title: Book { title: "1984", author: "George Orwell" }
title: 1984
title: Book { title: "1984", author: "George1111 Orwell" }
title: Document({"_id": ObjectId("6655b897c608f5ef7a046f9f"), "title": String("1984"), "author": String("George Orwell")})
title: Some(String("1984"))
title: "\"1984\""
title: Document({"_id": ObjectId("6655c56d94e7f573a2210504"), "title": String("1984"), "author": String("George1111 Orwell")})
title: Some(String("1984"))
title: "\"1984\""

```



第一种 指定结构体 可以直接取值

第二种 是不知道有没有 mongodb就是这样 字段有没有是不确定的 

最好的方式是全部覆盖 没有的用默认值





# 7、删除文档

在 MongoDB 中，你可以通过指定一个或多个匹配文档的条件来删除集合中的文档。

