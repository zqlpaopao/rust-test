#![allow(unused)]

use anyhow::anyhow;
use log::error;
use std::error::Error as _;
use std::io;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?},found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
////////////////////////////////////////////////// enum ///////////////////////////////////////////
/// 支持常量
/// 支持自动调用函数
/// 支持占位
pub fn first_char(s: &String) -> char {
    if s.len() == 0 {
        '-'
    } else {
        s.chars().next().unwrap_or('-')
    }
}

#[derive(Debug)]
pub struct Limits {
    lo: i16,
    hi: i16,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("invalid rdo_lookahead_frames {0} (expected < {})", i8::MAX)]
    InvalidLookahead(u32),
    #[error("first letter must be lowercase but was {:?}",first_char(.0))]
    WrongCase(String),
    #[error("invalid index.html {idx},expected at least {} and at most {}",.limits.lo,.limits.hi)]
    OutOfBounds { idx: usize, limits: Limits },
}

////////////////////////////////////////////////// 结构体 ///////////////////////////////////////////
#[derive(Error, Debug)]
#[error("something failed, msg is: {msg}")]
pub struct MyErrorStruct {
    msg: &'static str,
}

////////////////////////////////////////////////// from ///////////////////////////////////////////
#[derive(Error, Debug)]
#[error("some io error happened, {:?}", .source)]
pub struct MyFromError {
    #[from]
    source: io::Error,
}

////////////////////////////////////////////////// backtrace ///////////////////////////////////////////
// #[derive(thiserror::Error, Debug)]
// pub enum MyErrorBackTrace {
//     Io {
//         #[backtrace]
//         source: io::Error,
//     },
// }

////////////////////////////////////////////////// #[error[transparent)] ///////////////////////////////////////////
#[derive(Error, Debug)]
#[error(transparent)]
pub struct MyErrorTrans {
    #[from]
    source: anyhow::Error,
}

#[derive(Error, Debug)]
pub enum MyErrorTransEnum {
    #[error("file not found")]
    FileNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

pub fn error() {
    println!("这是没有参数的 Unknown {}", DataStoreError::Unknown);
    //这是没有参数的 Unknown unknown data store error

    // 2
    //  #[error("invalid header (expected {expected:?},found {found:?})")]
    //     InvalidHeader { expected: String, found: String },
    println!(
        "这是结构体参数的 InvalidHeader {}",
        DataStoreError::InvalidHeader {
            expected: String::from("expected"),
            found: String::from("found")
        }
    );
    //这是结构体参数的 InvalidHeader invalid header (expected "expected",found "found")

    // 3
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    println!(
        "这是有index参数的 Redaction {}",
        DataStoreError::Redaction(String::from("Redaction"))
    );
    // 这是有index参数的 Redaction the data for key `Redaction` is not available

    // 4
    //  #[error("data store disconnected")]
    //  Disconnect(#[from] std::io::Error),
    println!(
        "这是有from参数的 Disconnect {}",
        DataStoreError::Disconnect(io::Error::from(io::ErrorKind::TimedOut))
    );
    // 这是有from参数的 Disconnect data store disconnected

    // 5 enum error
    //  #[error("invalid rdo_lookahead_frames {0} (expected < {})", i8::MAX)]
    //  InvalidLookahead(u32),
    println!(
        "这是 enum 的InvalidLookahead {}",
        MyError::InvalidLookahead(3333)
    );
    // 这是 enum 的InvalidLookahead invalid rdo_lookahead_frames 3333 (expected < 127)

    // 6 自动调用函数进行比较
    // #[error("first letter must be lowercase but was {:?}",first_char(.0))]
    // WrongCase(String),
    println!(
        "这是 enum 的 WrongCase {}",
        MyError::WrongCase("kk".to_string())
    );
    // 这是 enum 的 WrongCase first letter must be lowercase but was 'k'

    // 7
    // #[error("invalid index.html {idx},expected at least {} and at most {}",.limits.lo,.limits.hi)]
    // OutOfBounds { idx: usize, limits: Limits },
    println!(
        "这是 enum 的 OutOfBounds {}",
        MyError::OutOfBounds {
            idx: 89,
            limits: Limits { lo: 12, hi: 11 }
        }
    );
    // 这是 enum 的 OutOfBounds invalid index.html 89,expected at least 12 and at most 11

    // 8 struct error
    // #[derive(Error, Debug)]
    // #[error("something failed, msg is: {msg}")]
    // pub struct MyErrorStruct {
    //     msg: &'static str,
    // }
    println!(
        "这是 struct 的msg  {}",
        MyErrorStruct {
            msg: "失败的msg"
        }
    );
    //这是 struct 的msg  something failed, msg is: 失败的msg

    //9 from
    // #[derive(Error, Debug)]
    // #[error("some io error happened, {:?}", .source)]
    // pub struct MyFromError {
    //     #[from]
    //     source: io::Error,
    // }
    println!(
        "这是 struct 的 from 的 {}",
        MyFromError::from(io::Error::from(io::ErrorKind::TimedOut))
    );
    let err = MyFromError::from(io::Error::from(io::ErrorKind::TimedOut));
    println!("{:?}", err.source());
    // 这是 struct 的 from 的 some io error happened, Kind(TimedOut)
    // Some(Kind(TimedOut))

    // 10 backtrace
    // #[derive(Error, Debug)]
    // pub enum MyErrorBackTrace {
    //     Io {
    //         #[backtrace]
    //         source: io::Error,
    //     },
    // }
    // println!("这是 backtrace的错误 {:?}",MyErrorBackTrace::Io {source:io::Error::from(io::ErrorKind::TimedOut)});

    //transparent
    // 当你使用 transparent 特性时，如果你有一个错误类型 MyError，
    // 它包含了一个 io::Error，那么在某些上下文中，MyError 可以被自动转换为 io::Error，
    // 而不需要显式调用 MyError::source() 方法。
    // #[derive(Error, Debug)]
    // #[error(transparent)]
    // pub struct MyErrorTrans {
    //     #[from]
    //     source: anyhow::Error,
    // }
    let err = MyErrorTrans::from(anyhow!("Missing attribute: {}", "field1"));
    println!("{}", err);
    println!("{:?}", err);
    // Missing attribute: field1
    // MyErrorTrans { source: Missing attribute: field1

    // #[derive(Error, Debug)]
    // pub enum MyErrorTransEnum {
    //     #[error("file not found")]
    //     FileNotFound,
    //     #[error(transparent)]
    //     Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
    // }
    let err = MyErrorTransEnum::from(anyhow!("Missing attribute: {}", "field1"));
    println!("---->{}", err);
    println!("{:?}", err);
    // ---->Missing attribute: field1
    // Other(Missing attribute: field1
}
