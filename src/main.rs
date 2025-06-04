#![allow(unused)]

use my_test::controller::async_std;
use my_test::controller::chrono;
use my_test::controller::clap_args;
use my_test::controller::clickhouse_tcp;
use my_test::controller::config;
use my_test::controller::dashmap;
use my_test::controller::fast_log;
use my_test::controller::for_range::test_range;
use my_test::controller::futures_rs;
use my_test::controller::hashmap;
use my_test::controller::kafka_rd;
use my_test::controller::log;
use my_test::controller::memory_stats;
use my_test::controller::mongodb;
use my_test::controller::mpsc;
use my_test::controller::mysql;
use my_test::controller::num_cpus;
use my_test::controller::phantom_data;
use my_test::controller::rayon;
use my_test::controller::rbatis;
use my_test::controller::redis;
use my_test::controller::select_join;
use my_test::controller::serde;
use my_test::controller::sync_async;
use my_test::controller::test_rust;
use my_test::controller::test_tokio;
use my_test::controller::thiserror;
use my_test::controller::thread_pool;
use my_test::controller::tk_log;
use my_test::controller::tokio_console;
use my_test::controller::tokio_signal;
use my_test::controller::tokio_time;
use my_test::controller::tokio_util;
use my_test::controller::vec;
use my_test::controller::wait_group;
use my_test::controller::websocket;
use my_test::controller::{
    actix_web as my, cow, nebula, rbatis_sqlite, s_f_list, tokio_cancel_watch,
};
use my_test::controller::{anyhow, lazy_lock_1_80, reqwest_http, tonic_rpc};
use my_test::controller::{as_ref_into, code_model, future_into_future};
use my_test::controller::{async_await, progress_bar_jin_du_tiao, web_assembly};
use std::collections::HashMap;
// use my_test::controller::clickhouse_http;
use my_test::controller::actix_web_sse::actix_web_sse::test_sses;
use my_test::controller::tracing::span_instrument::test_instrument;
use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};
use sqlx::MySql;

use actix_web::rt::System;
use futures::future;
use std::thread;
use std::time::Duration;
use tokio::task::spawn;
use tokio::time::sleep;

// use chrono::Local;
use tokio::sync::Semaphore;
use tokio::{self, runtime::Runtime, task, time};

use my_test::controller::grpc_protos_tonic::test_proto;

// fn now() -> String {
//     Local::now().format("%F %T").to_string()
// }

#[tokio::main]
async fn main() {
    // test_file_watcher().await

    //多生产者 多消费者
    // test_crossbeam_1().await;

    //
    // test_phan_tom_data()

    //大文件读取优化
    // test_file_reader();

    // 测试arroyo 流处理 查询kafka
    // test_arroyo_to_kafka().await;

    //kafka
    // test_rd_kafka().await;

    //channel 测试
    // test_channels().await;

    //redis
    // test_redis().await;

    // /自定义神经网络
    // test_work()

    //tracing instrument
    // test_instrument1()

    //stream trait
    // test_streams().await;

    //axum及中间件
    // test_axum_code().await;

    //Rust 大文件处理对比：标准库、Tokio 与内存映射的性能分析
    // test_read_big_file();

    //编程模式-继承
    // test_code();

    //测试 as ref
    // test_as_red_deref()

    // tokio mpmc
    test_tokio_mp_mc().await;

}


//测试 as ref
fn test_as_red_deref() {
    // test_as_ref_deref()

    test_as_ref()

}


// tokio mpmc
async  fn test_tokio_mp_mc(){
    test_tokio_mpmc().await;
}


//编程模式-继承
fn test_code(){
    test_jc();
}

//Rust 大文件处理对比：标准库、Tokio 与内存映射的性能分析
fn test_read_big_file() {
    test_read()
}

//axum
async fn test_axum_code() {
    test_axum().await.unwrap();
}

//stream
async fn test_streams() {
    test_stream_trait().await
}

//自定义神经网络
fn test_work() {
    //
    // test_network()
    //
    test_stream()
}

//tracing instrument
fn test_instrument1() {
    test_instrument()
}

// channel 测试
async fn test_channels() {
    test_channel().await;
}

// 测试arroyo 流处理 查询kafka
async fn test_arroyo_to_kafka() {
    test_arroyo_kafka().await;
}

// crossbeam
async fn test_crossbeam_1() {
    test_crossbeam().await
}

//几种遍历方式
fn test_range_n() {
    test_range()
}

// 文件监控变化
async fn test_file_watcher() {
    test_file_watch_sync().await
}

//类型转换
fn test_change_type() {
    test_type_change()
}

//grpc proto
async fn test_grpc() {
    test_proto().await
}

// 进度条样式
fn test_progress() {
    progress_bar_jin_du_tiao::test_progress()
}

//测试 web assembly
async fn test_web_assembly() {
    // web_assembly::test_web_assembly().await;
}

//内存布局
async fn test_reprs() {
    test_repr();
}

// 测试nebula
async fn test_nebula() {
    nebula::test_nebula().await
}

//测试。std::future::INtoFuture
async fn test_into_future() {
    future_into_future::run().await;
}

// 策略
fn test_code_model() {
    //策略模式
    // code_model::celue::test();

    //观察者模式
    // code_model::guanchazhe::test()

    //装饰器
    code_model::zhaungshiqi::test()
}

//test tokio util
async fn test_tokio_util() {
    tokio_util::test_tokio_util().await
}

//点到点的p2p网络
async fn websocket() {
    websocket::test_websocket().await;
}

/// 测试原子自增
fn atomic() {
    test_atomic()
}

// 测试 asref deref  & *


// uuid
fn test_uuids() {
    test_uuid()
}

//rpc
async fn test_tonic() {
    reqwest_http::test_middleware().await
}

//lazyLock
fn test_lazy_lock() {
    lazy_lock_1_80::lazy_lock()
}

//测试trait
fn test_traits() {
    fast_log()
}

// sqlite
async fn test_sqlite() {
    rbatis_sqlite::test_sqlite().await
}

// tokio 的 cancel 和上下文传播 watch

async fn test_tokio_context() {
    tokio_cancel_watch::test_context().await;
}

//算法
async fn s_test_f() {
    // s_f()

    //list
    s_f_list::test_list()
}

use std::cmp::Ordering;
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0_i32, (nums.len() - 1) as i32);
    while left <= right {
        let mut middle = (left + right) / 2;
        match nums[middle as usize].cmp(&target) {
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle - 1,
            Ordering::Equal => return middle as i32,
        }
    }
    -1
}

fn sum_two(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(4);
    for (k, v) in arr.iter().enumerate() {
        let comp = target - v;
        if let Some(index) = map.get(&comp) {
            return vec![*index as i32, k as i32];
        }
        map.insert(v, k);
    }
    vec![]
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)] // 使用这个属性来拒绝未知字段
struct MyStruct {
    pub known_field: String,
    // 这里可以定义其他字段
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct {
            known_field: "".to_string(),
        }
    }
}

//test cow
fn test_cow() {
    cow::test_cow()
}

use colored::*;
use diesel::IntoSql;
use my_test::controller::ai::stream_data::test_stream;
use my_test::controller::ai::zi_ding_yi_ai::test_network;
use my_test::controller::arroyo_stream_msg::test_arroyo_kafka;
use my_test::controller::as_ref::test_as_ref;
// use my_test::controller::asref_deref::test_as_ref_deref;
use my_test::controller::atomic::test_atomic;
use my_test::controller::axum::axum::test_axum;
use my_test::controller::channel::test_channel;
use my_test::controller::code_mode::jc::test_jc;
use my_test::controller::crossbeam::test_crossbeam;
use my_test::controller::file_reader::test_file_reader;
use my_test::controller::file_watch_sync::test_file_watch_sync;
use my_test::controller::grpc_protos_tonic::test_proto::test_proto;
use my_test::controller::read_big_file::test_read;
use my_test::controller::repr::test_repr;
use my_test::controller::s_f_array::s_f;
use my_test::controller::stream_trait::test_stream_trait;
use my_test::controller::test_trait::test_trait;
use my_test::controller::tokio_mpmc::test_tokio_mpmc;
use my_test::controller::type_change::test_type_change;
use my_test::controller::uuid::test_uuid;

//测试颜色
fn test_color() {
    // 直接在字符串上使用扩展方法来设置颜色
    println!("This is in red: {}", "a red string".red());

    // 你也可以设置背景色
    println!("With a blue background: {}", "a string".on_blue());

    // 添加样式，比如加粗
    println!("Bold and underlined: {}", "a string".bold().underline());

    // 甚至可以组合多种属性
    println!(
        "Yellow on blue and italic: {}",
        "a string".yellow().on_blue().italic()
    );
}

//redis rs
async fn test_redis() {
    redis::test_redis().await;
}

//rd kafka
async fn test_rd_kafka() {
    kafka_rd::test_kafka_rd().await
}

// 内存使用
fn test_memory() {
    memory_stats::test_memory_stats()
}

//测试tokio time
async fn test_tokio_time() {
    tokio_time::test_tokio_time().await;
}

//测试future rs
async fn test_futures_rs() {
    futures_rs::test_futures_rs().await
}

//测试tk log
fn test_tk_log() {
    tk_log::log_init()
}

//测试actix_web sse
async fn test_sse() {
    test_sses().await.unwrap()
}

//测试mongo
async fn test_mongodb() {
    mongodb::test_mongodb().await;
}

//测试chrono 时间处理
fn test_chrono() {
    chrono::test_chrono()
}

// 并发执行future
async fn test_futures() {
    select_join::test_more_future().await
}

//测试ck
async fn test_ck() {
    let res = clickhouse_tcp::test_ck().await;
}

//测试ck http
async fn test_ck_http() {
    // clickhouse_http::test_clickhouse_http().await
}
//测试rust语法
async fn test_rust() {
    // test_rust::test_rust().await;
    vec::test_vec()
}

//wait group
fn test_wait_group() {
    wait_group::test_wait_group()
}

//fast_log
fn fast_log() {
    fast_log::test_fast_log()
}

//测试async pool
fn test_async_pool() {
    async_await::test_async()
}

//测试线程池
fn test_thread_pool() {
    thread_pool::test_thread()
}

async fn test_tokio_signal() {
    tokio_signal::shutdown_signal().await
}
async fn test_tokio_console() {
    // tokio_console::test_tokio_console().await;
}

fn test_phan_tom_data() {
    phantom_data::test_phan_tom_data()
}

fn test_dash_map() {
    dashmap::dash_map_test();
}

async fn test_async() {
    sync_async::test_async().await;
}

async fn rbatis_test() {
    rbatis::init_rb().await;
}

fn as_ref_into() {
    as_ref_into::test_as_ref_into()
}

fn hashmap() {
    hashmap::test_hashmap();
}

fn test_serde() {
    serde::test();
}

async fn test_rayon() {
    rayon::test_rayon().await;
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, sqlx::FromRow)]
pub struct LldP {
    pub local_ip: String,
}

///actix_web
async fn actix_web() {
    my::web().await;
}

///clap
fn clap() {
    clap_args::claps()
}

/// 获取cpus 及物理cpus数量
fn cpus() {
    num_cpus::cpus()
}

///anyhow
fn anyhow() {
    anyhow::anyhow_use()
}

/// 如何使用this error
fn error() {
    thiserror::error();
}

///配置文件 toml
fn config() {
    config::read_toml();
}

/// 日志的使用
async fn log() {
    log::log().await;
}

///初始化全局化mysql cli
async fn mysql() {
    //初始化 全局mysql 连接池
    mysql::MyPool::default();
    let cli = mysql::MyPool::get();
    let res = sqlx::query_as::<MySql, LldP>(
        r#"
        select
        *
        from test where
        local_ip = ? and
        local_if_name= ?
    "#,
    )
    .bind("ip1")
    .bind("ports")
    .fetch_all(cli)
    .await
    .unwrap();
    println!("{:#?}", res)
}
