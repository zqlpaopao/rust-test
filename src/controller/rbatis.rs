#![allow(unused)]
use futures::executor::block_on;
use lazy_static::lazy_static;
use rbatis::crud;
use rbatis::impl_delete;
use rbatis::impl_select;
use rbatis::impl_select_page;
use rbatis::impl_update;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::pool::Pool;
use rbatis::table_sync::SqliteTableMapper;
use rbatis::{table_sync, RBatis};
use rbdc_mysql::MysqlDriver;
use serde_json::json;
use std::option::Option;

// ********************************************** init pool ****************************************
const DB_URL: &str = "mysql://root:meimima123@127.0.0.1:3306/test";
#[derive(Debug)]
pub struct BaitsPool(Option<RBatis>);
impl Default for BaitsPool {
    fn default() -> Self {
        _ = fast_log::init(
            fast_log::Config::new()
                .console()
                .level(log::LevelFilter::Trace),
        );
        let rb = RBatis::new();
        rb.init(MysqlDriver {}, DB_URL).expect("mysql init fail");

        let pool = rb.get_pool().unwrap();
        block_on(set(pool));
        BaitsPool(Option::from(rb))
    }
}

async fn set(pool: &dyn Pool) {
    pool.set_conn_max_lifetime(Some(std::time::Duration::from_secs(86400)))
        .await;
    pool.set_timeout(Some(std::time::Duration::from_secs(6400)))
        .await;
    pool.set_max_idle_conns(3).await;
    pool.set_max_open_conns(10).await;
}

impl BaitsPool {
    pub async fn get<'a>() -> &'a RBatis {
        lazy_static! {
            // 此处表示声明全局可变 HashMap
            pub static ref POOL: BaitsPool = BaitsPool::default();
        }
        POOL.0.as_ref().unwrap()
    }
}

// **************************** 表结构定义 ****************************************
/*
    CREATE TABLE `activity`
(
    `id`            varchar(50)  NOT NULL DEFAULT '' COMMENT '唯一活动码',
    `name`          varchar(255) NOT NULL,
    `pc_link`       varchar(255)          DEFAULT NULL,
    `h5_link`       varchar(255)          DEFAULT NULL,
    `sort`          varchar(255) NOT NULL COMMENT '排序',
    `status`        int(11) NOT NULL COMMENT '状态（0：已下线，1：已上线）',
    `version`       int(11) NOT NULL,
    `remark`        varchar(255)          DEFAULT NULL,
    `create_time`   datetime     NOT NULL,
    `delete_flag`   int(1) NOT NULL,
    `pc_banner_img` varchar(255)          DEFAULT NULL,
    `h5_banner_img` varchar(255)          DEFAULT NULL,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8  COMMENT='运营管理-活动管理';

INSERT INTO `activity`
VALUES ('1', '活动1', NULL, NULL, '1', 1, 1, 'fff', '2019-12-12 00:00:00', 0, NULL, NULL),
       ('178', 'test_insret', '', '', '1', 1, 0, '', '2020-06-17 20:08:13', 0, NULL, NULL),
       ('221', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL),
       ('222', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL),
       ('223', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL);

 */

/// table
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

//custom table name
//crud!(Activity {},"activity");
crud!(Activity {}); // impl_insert!($table {}) + impl_select!($table {}) + impl_update!($table {}) + impl_delete!($table {});

// ******************************************** 测试 *********************************************

pub async fn init_rb() {
    //插入
    // insert().await;
    //修改
    // update().await;

    //查询
    select().await;

    //删除
    // delete().await;

    //事物
    // transaction().await;

    //表同步
    // sync().await;

    //宏
    // macros().await;
}

// ******************************************** insert  *********************************************

async fn insert() {
    let rb = BaitsPool::get().await;
    let table = Activity {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: Some("2".to_string()),
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
    // let cli = BaitsPool::get();
    // let data = Activity::insert(rb, &table).await;
    // println!("insert = {}", json!(data));
    //insert = {"Ok":{"last_insert_id":0,"rows_affected":1}}

    //batch
    let tables = [table.clone(), {
        let mut t3 = table.clone();
        t3.id = "3".to_string().into();
        t3
    }];
    let data = Activity::insert_batch(rb, &tables, 10).await;
    println!("insert_batch = {}", json!(data));
    //insert_batch = {"Ok":{"last_insert_id":0,"rows_affected":2}}
}
// ******************************************** update  *********************************************
impl_update!(Activity{update_by_name(name:&str) => "`where id = '2'`"});
impl_update!(Activity{update_by_sort(name:&str) => "`where name = #{name}`"});
impl_update!(Activity{update_by_struct(s:&Activity) => "`where name = #{s.name}`"});

async fn update() {
    let rb = BaitsPool::get().await;
    let table = Activity {
        id: Some("223".into()),
        name: Some("244".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: None,
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };

    // let data = Activity::update_by_column(rb, &table, "id").await;
    // println!("update_by_column = {}", json!(data));
    /*
    2024-03-13 16:45:35.263164  [INFO] [rbatis] [610395144469681311] exec  => `update activity set name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=? where id = ?` ["2","2","2",2,"2","2024-03-13T16:45:35.237191+08:00",1,1,"178"]
    2024-03-13 16:45:35.26931   [INFO] [rbatis] [610395144469681311] exec  <= rows_affected={"rows_affected": 1, "last_insert_id": U64(0)}
    update_by_column = {"Ok":{"last_insert_id":0,"rows_affected":1}}
     */

    // let data = Activity::update_by_name(rb, &table, "2").await;
    // println!("update_by_name = {}", json!(data));
    /*
    2024-03-13 16:47:05.376593  [INFO] [rbatis] [610395522431087776] exec  => `update activity set id=?,name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=? where id = '2'` ["2","2","2","2",2,"2","2024-03-13T16:47:05.353819+08:00",1,1]
    2024-03-13 16:47:05.381922  [INFO] [rbatis] [610395522431087776] exec  <= rows_affected={"rows_affected": 1, "last_insert_id": U64(0)}
    update_by_name = {"Ok":{"last_insert_id":0,"rows_affected":1}}
     */

    // let data = Activity::update_by_sort(rb, &table, "233").await;
    // println!("update_by_sort = {}", json!(data));
    /*
    2024-03-13 16:53:41.519798  [INFO] [rbatis] [610397183975653391] exec  => `update activity set id=?,name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=? where name = ?` ["223","244","2","2",2,"2","2024-03-13T16:53:41.494476+08:00",1,1,"233"]
    2024-03-13 16:53:41.523382  [INFO] [rbatis] [610397183975653391] exec  <= rows_affected={"rows_affected": 0, "last_insert_id": U64(0)}
    update_by_sort = {"Ok":{"last_insert_id":0,"rows_affected":0}}
     */

    // Activity::update_by_column_batch()

    let data = Activity::update_by_struct(rb, &table, &table).await;
    println!("update_by_struct = {}", json!(data));
    /*
    2024-03-13 17:15:03.624716  [INFO] [rbatis] [610402561515065416] exec  =>
    `update activity set id=?,name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=? where name = ?` ["223","244","2","2",2,"2","2024-03-13T17:15:03.57596+08:00",1,1,"244"]
    update_by_struct = {"Ok":{"last_insert_id":0,"rows_affected":1}}
     */
}

// ******************************************** select  *********************************************
impl_select!(Activity{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
impl_select!(Activity{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});
impl_select!(Activity{select_by_struct(s:&Activity) -> Option => "`where id = #{s.id} limit 1`"});
impl_select_page!(Activity{select_page_by_limit(name:&str) => "`where name != #{name}`"});
impl_select_page!(Activity{select_page() =>"
     if do_count== false:
       `order by create_time desc`"});
impl_select_page!(Activity{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});

//你用 if do_count == false:  来判断吧不要直接判断sql语句
async fn select() {
    let rb = BaitsPool::get().await;
    // let data = Activity::select_by_column(rb, "id","1").await;
    // println!("select_by_column = {}", json!(data));
    /*
    2024-03-14 10:51:40.724969  [INFO] [rbatis] [610668468288580916] query => `select * from activity  where id = ?` ["1"]
    select_by_column = {"Ok":[{"create_time":"2019-12-12T00:00:00Z","delete_flag":0,"h5_banner_img":null,"h5_link":null,"id":"1","name":"活动1","pc_banner_img":null,"pc_link":null,"remark":"fff","sort":"1","status":1,"version":1}]}
    */

    // let data = Activity::select_all_by_id(rb, "1", "1").await;
    // println!("select_all_by_id = {}", json!(data));
    /*
    2024-03-14 10:51:40.740479  [INFO] [rbatis] [610668468288580916] query <= len=1
    2024-03-14 10:51:40.755069  [INFO] [rbatis] [610668468418604372] query => `select * from activity where id = ? and name = ?` ["1","1"]
    select_all_by_id = {"Ok":[]}

     */

    // let data = Activity::select_by_id(rb, "1").await;
    // println!("select_by_id = {}", json!(data));
    /*
    2024-03-14 10:51:40.756263  [INFO] [rbatis] [610668468418604372] query <= len=0
    2024-03-14 10:51:40.766895  [INFO] [rbatis] [610668468464741728] query => `select * from activity where id = ? limit 1` ["1"]
    2024-03-14 10:51:40.769488  [INFO] [rbatis] [610668468464741728] query <= len=1
    select_by_id = {"Ok":{"create_time":"2019-12-12T00:00:00Z","delete_flag":0,"h5_banner_img":null,"h5_link":null,"id":"1","name":"活动1"pc_banner_img":null,"pc_link":null,"remark":"fff","sort":"1","status":1,"version":1}}

     */

    // let data = Activity::select_by_struct(rb, &Activity{
    //     id: Some("2".into()),
    //     name: None,
    //     pc_link: None,
    //     h5_link: None,
    //     pc_banner_img: None,
    //     h5_banner_img: None,
    //     sort: None,
    //     status: None,
    //     remark: None,
    //     create_time: None,
    //     version: None,
    //     delete_flag: None,
    // }).await;
    // println!("select_by_struct = {}", json!(data));
    /*
    2024-03-14 10:51:40.779128  [INFO] [rbatis] [610668468519267694] query => `select * from activity where id = ? limit 1` ["2"]
    select_by_struct = {"Ok":{"create_time":"2024-03-13T16:47:05Z","delete_flag":1,"h5_banner_img":null,"h5_link":"2","id":"2","name":"2","pc_banner_img":null,"pc_link":"2","remark":"2","sort":"2","status":2,"version":1}}
     */

    let data = Activity::select_page_by_limit(rb, &PageRequest::new(1, 10), "2").await;
    println!("select_page_by_limit = {}", json!(data));
    /*
        2024-03-14 11:23:05.652405  [INFO] [rbatis] [610676374247321654] query <= len=0
    2024-03-14 11:23:05.661219  [INFO] [rbatis] [610676374289264705] query => `select count(1) as count from activity where name != ?` ["2"]
    2024-03-14 11:23:05.664203  [INFO] [rbatis] [610676374289264705] query <= len=1
    2024-03-14 11:23:05.673077  [INFO] [rbatis] [610676374339596366] query => `select * from activity where name != ? limit 0,10 ` ["2"]
    2024-03-14 11:23:05.674044  [INFO] [rbatis] [610676374339596366] query <= len=4

         */

    // let data = Activity::select_page(rb, &PageRequest::new(1, 10)).await;
    // println!("select_page = {}", json!(data));
    /*
    2024-03-14 11:33:59.586638  [INFO] [rbatis] [610679117050161826] query => `select count(1) as count from activity ` []
    2024-03-14 11:33:59.590164  [INFO] [rbatis] [610679117050161826] query <= len=1
    2024-03-14 11:33:59.599505  [INFO] [rbatis] [610679117104687792] query => `select * from activity order by create_time desc limit 0,10` []
    2024-03-14 11:33:59.600164  [INFO] [rbatis] [610679117104687792] query <= len=4
    select_page = {"Ok":{"do_count":true,"page_no":1,"page_size":10,"records":[{"create_time":"2024-03-13T18:54:14Z","delete_flag":1,"h5_banner_img":null,"h5_link":"2","id":"223","name":"244","pc_banner_img":null,"pc_link":"2","remark":"2","sort":"0","status":2,"version":1},{"create_time":"2020-06-17T20:10:23Z","delete_flag":0,"h5_banner_img":null,"h5_link":"","id":"221","name":"test","pc_banner_img":null,"pc_link":"","remark":"","sort":"0","status":0,"version":0},{"create_time":"2020-06-17T20:10:23Z","delete_flag":0,"h5_banner_img":null,"h5_link":"","id":"222","name":"test","pc_banner_img":null,"pc_link":"","remark":"","sort":"0","status":0,"version":0},{"create_time":"2019-12-12T00:00:00Z","delete_flag":0,"h5_banner_img":null,"h5_link":null,"id":"1","name":"活动1","pc_banner_img":null,"pc_link":ll,"remark":"fff","sort":"1","status":1,"version":1}],"total":4}}

    */

    // let data = Activity::select_page_by_name(rb, &PageRequest::new(1, 10), "").await;
    // println!("select_page_by_name = {}", json!(data));
    /*
    2024-03-14 11:35:45.984679  [INFO] [rbatis] [610679563315825216] query => `select count(1) as count from activity where name != ''` []
    2024-03-14 11:35:45.986885  [INFO] [rbatis] [610679563315825216] query <= len=1
    2024-03-14 11:35:46.004619  [INFO] [rbatis] [610679563399711317] query => `select * from activity where name != '' limit 0,10` []
    2024-03-14 11:35:46.005648  [INFO] [rbatis] [610679563399711317] query <= len=4
    select_page_by_name = {"Ok":{"do_count":true,"page_no":1,"page_size":10,"records":[{"create_time":"2019-12-12T00:00:00Z","delete_flag":0,"h5_banner_img":null,"h5_link":null,"id":"1","name":"活动1","pc_banner_img":null,"pc_link":null,"remark":"fff","sort":"1","status":"version":1},{"create_time":"2020-06-17T20:10:23Z","delete_flag":0,"h5_banner_img":null,"h5_link":"","id":"221","name":"test","pc_banner_img":null,"pc_link":"","remark":"","sort":"0","status":0,"version":0},{"create_time":"2020-06-17T20:10:23Z","delete_flag":0,"h5_banner_img":null,"h5_link":"","id":"222","name":"test","pc_banner_img":null,"pc_link":"","remark":"","sort":"0","status":0,"version":0},{"create_time":"2024-03-13T18:54:14Z","delete_flag":1,"h5_banner_img":null,"h5_link":"2","id":"223","name":"244","pc_banner_img":null,"pc_link":"2","remark":"2","sort":"0","status":2,"version":1}],"total":4}}

     */
}

// ******************************************** delete  *********************************************
impl_delete!(Activity {delete_by_names(name:&str) => "`where name =#{name}`"});

async fn delete() {
    let rb = BaitsPool::get().await;

    let data = Activity::delete_by_column(rb, "id", "2").await;
    println!("delete_by_column = {}", json!(data));
    /*
    2024-03-14 11:10:42.112269  [INFO] [rbatis] [610673255617976256] exec  => `delete from activity where id = ?` ["2"]
    2024-03-14 11:10:42.134272  [INFO] [rbatis] [610673255617976256] exec  <= rows_affected={"rows_affected": 1, "last_insert_id": U64(0)}
    delete_by_column = {"Ok":{"last_insert_id":0,"rows_affected":1}}

     */

    let data = Activity::delete_by_names(rb, "2".into()).await;
    println!("delete_by_name = {}", json!(data));
    /*
    2024-03-14 11:12:21.325865  [INFO] [rbatis] [610673671747558222] exec  => `delete from activity where name =?` ["2"]
    delete_by_name = {"Ok":{"last_insert_id":0,"rows_affected":0}}
     */

    Activity::delete_by_column_batch::<&str>(rb, "name", &["2".into(), "3".into()], 2).await;
    println!("delete_by_column_batch = {}", json!(data));
    /*
    2024-03-14 11:10:42.155661  [INFO] [rbatis] [610673255798331373] exec  => `delete from activity where name in (?,?)` ["2","3"]
    delete_by_column_batch = {"Ok":{"last_insert_id":0,"rows_affected":0}}
    2024-03-14 11:10:42.161019  [INFO] [rbatis] [610673255798331373] exec  <= rows_affected={"rows_affected": 2, "last_insert_id": U64(0)}


     */
}
// ******************************************** transaction  *********************************************
//支持defer_async()防止忘记提交

async fn transaction() {
    let t = Activity {
        id: None,
        name: Some("211".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: Some("None".into()),
        h5_banner_img: Some("None".into()),
        sort: Some("None".into()),
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };

    let rb = BaitsPool::get().await;
    let mut tx = rb.acquire_begin().await.unwrap();
    // defer_async will be rollback if tx drop
    // let mut tx = tx.defer_async(|mut tx| async move {
    //     if !tx.done {
    //         tx.rollback().await.unwrap();
    //         println!("rollback");
    //     }
    // });
    Activity::insert(&tx, &t).await.unwrap();
    tx.commit().await.unwrap();
    tx.rollback().await.unwrap();
    /*
    2024-03-14 14:19:00.335361  [INFO] [rbatis] [610718933409861633] exec  => `insert into activity (name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (?,?,?,?,?,?,?,?,?,?,?)` ["211","2","2","None","None","None",2,"2","2024-03-14T14:19:00.312547+08:00",1,1]
    2024-03-14 14:19:00.339861  [INFO] [rbatis] [610718933409861633] exec  <= rows_affected={"rows_affected": 1, "last_insert_id": U64(0)}
     */
}

// ******************************************** 表同步  *********************************************
//表的同步 不存在创建表，但是数据不会有的
use rbs::to_value;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RBUser {
    pub id: i32,
    pub name: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
async fn sync() {
    let rb = BaitsPool::get().await;

    let mapper = &table_sync::MysqlTableMapper {} as &dyn table_sync::ColumMapper;

    //第一种同步
    // let map = rbs::to_value!{
    //         "id":"INT",
    //         "name":"TEXT",
    //  };
    // let _ = RBatis::sync(rb,mapper,&map,"rb_user").await;
    /*
    2024-03-14 16:35:22.616483  [INFO] [rbatis] [610754962793305976] exec  => `CREATE TABLE rb_user (id INT,name TEXT);` []
    2024-03-14 16:35:22.633652  [INFO] [rbatis] [610754962793305976] exec  <= rows_affected={"rows_affected": 0, "last_insert_id": U64(0)}
     */
    RBatis::sync(
        &rb.acquire().await.unwrap(),
        mapper,
        &RBUser {
            id: 0,
            //// Custom String Database Type
            //name: Some("TEXT".to_string()),
            name: Some("".to_string()),
            //// Custom String Database Type
            //remark: Some("TEXT".to_string()),
            remark: Some("".to_string()),
            create_time: Some(DateTime::utc()),
            version: Some(1),
            delete_flag: Some(1),
        },
        "rb_user",
    )
    .await
    .unwrap();
    /*
    2024-03-14 16:36:17.662625  [INFO] [rbatis] [610755193673019006] exec  => `CREATE TABLE rb_user (id INT,name TEXT,remark TEXT,create_time DATETIME,version BIGINT,delete_flag INT);` []
    2024-03-14 16:36:17.664449  [INFO] [rbatis] [610755193673019006] exec  <= 1050 (42S01): Table 'rb_user' already exists
    2024-03-14 16:36:17.664457  [INFO] [rbatis] [610755193673019008] exec  => `alter table rb_user add id INT  PRIMARY KEY ;` []
    2024-03-14 16:36:17.666156  [INFO] [rbatis] [610755193673019008] exec  <= 1060 (42S21): Duplicate column name 'id'
    2024-03-14 16:36:17.666161  [DEBUG] ADD COLUMN fail=1060 (42S21): Duplicate column name 'id'
    2024-03-14 16:36:17.666164  [INFO] [rbatis] [610755193673019010] exec  => `alter table rb_user add name TEXT ;` []
    2024-03-14 16:36:17.666442  [INFO] [rbatis] [610755193673019010] exec  <= 1060 (42S21): Duplicate column name 'name'
    2024-03-14 16:36:17.666446  [DEBUG] ADD COLUMN fail=1060 (42S21): Duplicate column name 'name'
    2024-03-14 16:36:17.666449  [INFO] [rbatis] [610755193673019010] exec  => `alter table rb_user add remark TEXT ;` []
    2024-03-14 16:36:17.674565  [INFO] [rbatis] [610755193673019010] exec  <= rows_affected={"rows_affected": 0, "last_insert_id": U64(0)}
    2024-03-14 16:36:17.67458   [INFO] [rbatis] [610755193673019018] exec  => `alter table rb_user add create_time DATETIME ;` []
    2024-03-14 16:36:17.678101  [INFO] [rbatis] [610755193673019018] exec  <= rows_affected={"rows_affected": 0, "last_insert_id": U64(0)}
    2024-03-14 16:36:17.678112  [INFO] [rbatis] [610755193673019022] exec  => `alter table rb_user add version BIGINT ;` []
    2024-03-14 16:36:17.682916  [INFO] [rbatis] [610755193673019022] exec  <= rows_affected={"rows_affected": 0, "last_insert_id": U64(0)}
    2024-03-14 16:36:17.682926  [INFO] [rbatis] [610755193673019026] exec  => `alter table rb_user add delete_flag INT ;` []
    2024-03-14 16:36:17.686065  [INFO] [rbatis] [610755193673019026] exec  <= rows_affected={"rows_affected": 0, "last_insert_id": U64(0)}
     */
}

// ******************************************** 拦截件  *********************************************

// use rbatis::{Error};
// use rbatis::executor::Executor;
// use rbatis::intercept::{Intercept, ResultType};
// use rbdc::db::ExecResult;
// use rbs::Value;
// #[derive(Debug)]
// pub struct MyInterceptor{}
//
// impl Intercept for MyInterceptor {
//     /// task_id maybe is conn_id or tx_id,
//     /// is_prepared_sql = !args.is_empty(),
//     /// if return Ok(false) will be return data. return Ok(true) will run next
//     fn before(
//         &self,
//         _task_id: i64,
//         _rb: &dyn Executor,
//         _sql: &mut String,
//         _args: &mut Vec<Value>,
//         _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
//     ) -> Result<bool, Error> {
//         Ok(true)
//     }
//
//     /// task_id maybe is conn_id or tx_id,
//     /// is_prepared_sql = !args.is_empty(),
//     /// if return Ok(false) will be return data. return Ok(true) will run next
//     fn after(
//         &self,
//         _task_id: i64,
//         _rb: &dyn Executor,
//         _sql: &mut String,
//         _args: &mut Vec<Value>,
//         _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
//     ) -> Result<bool, Error> {
//         Ok(true)
//     }
// }

// ******************************************** 拦截件  *********************************************

use rbatis::rbdc::Decimal;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<Decimal>,
    pub delete_flag: Option<i32>,
}

impl Default for BizActivity {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: None,
            remark: None,
            create_time: None,
            version: None,
            delete_flag: None,
        }
    }
}

async fn macros() {
    // let table = rbatis::make_table!(BizActivity{
    //           id:"1".to_string(),
    //     });
    // println!("{:#?}", table);
    /*
    BizActivity {
        id: Some(
            "1",
        ),
        name: None,
        pc_link: None,
        h5_link: None,
        pc_banner_img: None,
        h5_banner_img: None,
        sort: None,
        status: None,
        remark: None,
        create_time: None,
        version: None,
        delete_flag: None,
    }

         */

    // let table = rbatis::make_table!(BizActivity{
    //           id:"1".to_string(),
    //           name:"a".to_string()
    //     });
    // let table_vec = vec![table];
    // let map = rbatis::make_table_field_map!(&table_vec,name);
    // println!("{:#?}", map);
    /*
    {
        "a": BizActivity {
            id: Some(
                "1",
            ),
            name: Some(
                "a",
            ),
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: None,
            remark: None,
            create_time: None,
            version: None,
            delete_flag: None,
        },
    }

         */
    // assert_eq!(map.len(), table_vec.len());

    ///
    let table = rbatis::make_table!(BizActivity {
        id: "1".to_string(),
        name: "a".to_string()
    });
    let table_vec = vec![table];
    let names = rbatis::make_table_field_vec!(&table_vec, name);
    println!("{:#?}", names);
    /*
    [
        "a",
    ]

         */
    assert_eq!(names.len(), table_vec.len());
}
