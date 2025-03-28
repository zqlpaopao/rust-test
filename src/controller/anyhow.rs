#![allow(unused)]

use anyhow::{anyhow, bail, Context, Error, Result};
use std::path::PathBuf;
use std::{backtrace, fs};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
}

fn downcast_ref() {
    let error: Error = anyhow!(MyError::Redaction("keys".to_string()));

    if let Some(my_error) = error.downcast_ref::<MyError>() {
        println!("MyError is parse ok: {}", my_error);
        // Handle MyError specifically
    } else {
        println!("Unknown error: {:?}", error);
        // Handle other types of errors
    }
    //MyError is parse ok: the data for key `keys` is not available
}

fn anyhow() -> Result<()> {
    return Err(anyhow!("this is return error"));
}

//配合thiserror使用

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
}

fn bar() -> std::result::Result<(), DataStoreError> {
    Err(DataStoreError::Redaction(
        "bar() std::result::Result".to_string(),
    ))
}

fn foo() -> anyhow::Result<()> {
    let a = bar()?;
    Ok(())
}

fn foo2() -> anyhow::Result<()> {
    Err(anyhow::Error::from(DataStoreError::Redaction(
        "foo2 (anyhow::Error::from(DataStoreError::Redaction ".to_string(),
    )))
}

fn foo3() -> anyhow::Result<()> {
    Err(anyhow!(DataStoreError::Redaction(
        "foo3 anyhow!(DataStoreError::Redaction".to_string()
    )))
}

fn foo4() -> anyhow::Result<()> {
    Err(anyhow!("foo4 anyhow! {}", "f4"))
}

fn bail(i: i16) -> anyhow::Result<()> {
    if i < 16 {
        bail!(DataStoreError::Redaction(
            i.to_string() + &" is less 16".to_string()
        ))
    }
    Ok(())
}
/////////////////////////////////     context         ///////////////////////////////////

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    pub fn detach(&mut self) -> Result<()> {
        Err(anyhow!("detach failed"))
    }
}

pub fn do_it(mut it: &mut ImportantThing) -> Result<Vec<u8>> {
    it.detach()
        .context("Failed to detach the important thing")?;
    let path = &it.path;
    let context =
        fs::read(path).with_context(|| format!("Failed to read in str form {}", path.display()))?;
    Ok(context)
}

pub fn do_it1(it: &mut ImportantThing) -> Result<Vec<u8>> {
    let path = &it.path;
    let content = fs::read(path)
        .with_context(|| format!("Failed to read instrs from {} 。。。", path.display()))?;

    Ok(content)
}
pub fn anyhow_use() {
    // let err = anyhow();
    // println!("{:#?}",err)

    // let f = foo();
    // println!("foo {:?}", f);
    // let f1 = foo2();
    // println!("foo2 {:?}",f1);
    //
    // let f2 = foo3();
    // println!("foo3 {:?}",f2);
    //
    // let f3 = foo4();
    // println!("foo4 {:?}",f3);

    // let b = bail(1);
    // println!("bail! error {:?}", b)

    // let mut it = ImportantThing {
    //     path: PathBuf::new(),
    // };
    //
    // match do_it(&mut it) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         for cause in err.chain() {
    //             println!("{}", cause)
    //         }
    //     }
    // }
    //
    // match do_it1(&mut it) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         for cause in err.chain() {
    //             println!("{}", cause)
    //         }
    //     }
    // }

    //错误回溯
    if let Err(e) = process() {
        println!("Error: {:?}", e);
        println!("backtrace: {:?}", e.to_string());
        for cause in e.chain().skip(1) {
            println!("Caused by: {}", cause);
        }
    }
}

/////////////////////////////////     错误回溯         ///////////////////////////////////
fn step1() -> Result<()> {
    Err(anyhow!("Error in step 1"))
}

fn step2() -> Result<()> {
    step1().context("Failed during step 2")?;
    Ok(())
}

fn process() -> Result<()> {
    step2().context("Process failed")?;
    Ok(())
}
