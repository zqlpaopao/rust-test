#![allow(dead_code)]

// https://mp.weixin.qq.com/s/Bw960I993FXmuNyLHA8pcw

use tracing::{info, instrument, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::fs::File;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};


pub fn test_instrument() {
    init_log();
    my_function(32);
}

fn init_log(){
    // 创建一个文件 appender，将日志写入到文件中
    let file = File::create("output.log").expect("Failed to create log file");
    let (non_blocking_file, _guard) = non_blocking_file(file);

    // 配置控制台和文件的 Subscriber
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(fmt::Layer::new().with_writer(std::io::stdout)) // 输出到控制台
        .with(fmt::Layer::new().with_writer(non_blocking_file).json()); // 输出到文件

    // 设置全局的 Subscriber
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // 输出一些日志
    info!("This will be logged to both console and file");

    // 记录日志
    info!("This is an info log message");
}
fn non_blocking_file(file: File) -> (non_blocking::NonBlocking, WorkerGuard) {
    non_blocking::NonBlocking::new(file)
}


/********************************* 基本用法 ******************************/

#[instrument]
fn my_function(my_arg: i32) {
    info!("inside my_function! arg is {}", my_arg);
}

/*
    2025-02-19T02:27:22.522305Z  INFO my_test::controller::tracing::span_instrument: This will be logged to both console and file
    2025-02-19T02:27:22.522439Z  INFO my_test::controller::tracing::span_instrument: This is an info log message
    2025-02-19T02:27:22.522757Z  INFO my_function{my_arg=32}: my_test::controller::tracing::span_instrument: inside my_function! arg is 32


 */