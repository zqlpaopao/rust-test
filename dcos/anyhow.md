一个优秀的项目，错误处理的优雅性是至关重要的，而rust，anyhow creat是绕不过去的一个，今天我们来研究下，怎么使用它，帮助我们写出更优雅的代码


关注 vx golang技术实验室，获取更多golang、rust好文

# 一、anyhow

`anyhow::Error` 是这个 `crate` 中最重要的结构体，它是动态错误类型的包装器，能从所有实现了 `std::error::Error + Send + Sync + 'static` 的错误转换而来，也能转换成 `Box`，它有以下特点：

1. `anyhow::Error` 要求包裹的错误必须是 `Send + Sync + 'static`；
2. `anyhow::Error` 保证 `backtrace` 是可用的，就是底层的错误类型没有提供；
3. `anyhow::Error` 在内存中只占一个机器字而不是两个；

## 1.1 打印anyhow方式

1. 可以使用 `{}` 或者 `.to_string()`，但是仅仅打印最外层错误或者上下文，而不是内层的错误；
2. 可以使用 `{:#}` 打印外层和底层错误；
3. 可以使用 `{:?}` 在调试模式打印错误以及调用栈；
4. 可以使用 `{:#?}` 以结构体样式打印错误，例如：

```
Error {
  context: "Failed to read instrs from ./path/to/instrs.json",
  source: Os {
      code: 2,
      kind: NotFound,
      message: "No such file or directory",
  },
}
```



## 1.2 downcast_ref

`anyhow` 提供了 `downcast_ref` 方法，用于在运行时将 `anyhow::Error` 转换为其包含的具体错误类型的引用。这可以用于检查和处理特定类型的错误。

```
use anyhow::{anyhow,Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum  MyError {
    #[error("the data for key `{0}` is not available")]
    Redaction(String)
}

pub fn anyhow_use(){
    let error: Error = anyhow!(MyError::Redaction("keys".to_string()));

    if let Some(my_error) = error.downcast_ref::<MyError>() {
        println!("MyError is parse ok: {}", my_error);
        // Handle MyError specifically
    } else {
        println!("Unknown error: {:?}", error);
        // Handle other types of errors
    }
}
```



```
MyError is parse ok: the data for key `keys` is not available
```



## 1.3 anyhow!

使用 `anyhow!` 这个宏可以生成 `anyhow::Error`类型的值，它可以接受字符串，格式化字符串作为参数，或者实现 `std::error:Error` 的错误作为参数。

```
use anyhow::{anyhow,Error,Result};

fn anyhow()->Result<()>{

    return Err(anyhow!("this is return error"))
}
pub fn anyhow_use(){
    let err = anyhow();
    println!("{:#?}",err)
}
```



```
Err(
    "this is return error",
)
```



## 1.4 anyhow配合thiserror使用

项目更多用到的组合

```
use anyhow::{anyhow, Error, Result};
use thiserror::Error;

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

pub fn anyhow_use() {
    // let err = anyhow();
    // println!("{:#?}",err)

    let f = foo();
    println!("foo {:?}", f);
    let f1 = foo2();
    println!("foo2 {:?}",f1);

    let f2 = foo3();
    println!("foo3 {:?}",f2);

    let f3 = foo4();
    println!("foo4 {:?}",f3);
}

```



```
foo Err(the data for key `bar() std::result::Result` is not available)
foo2 Err(the data for key `foo2 (anyhow::Error::from(DataStoreError::Redaction ` is not available)
foo3 Err(the data for key `foo3 anyhow!(DataStoreError::Redaction` is not available)
foo4 Err(foo4 anyhow! f4)
```



## 1.5 bail!

**`anyhow::bail` 宏用于提前错误返回，它等价于 `return Err(anyhow!($args...))`，包含这个宏的函数的返回值必须是 `Result<_,anyhow::Error>`：**



```
use anyhow::{anyhow,bail, Error, Result};
use futures::future::err;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
}


fn bail(i:i16)->anyhow::Result<()>{

    if i < 16{
        bail!(DataStoreError::Redaction(i.to_string() + &" is less 16".to_string()))
    }
    Ok(())
}

```

直接返回

```
bail! error Err(the data for key `1 is less 16` is not available)
```



## 1.6 anyhow::Context

`anyhow::Context` 为 `anyhow::Result` 类型提供了 `context` 方法，能在错误发生时提供更多的上下文信息：



```
use anyhow::{anyhow, bail, Context, Error, Result};
use futures::future::err;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;


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
    let content =
        fs::read(path).with_context(|| format!("Failed to read instrs from {}", path.display()))?;

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

    let mut it = ImportantThing{
        path:PathBuf::new()
    };

    match do_it(&mut it) {
        Ok(_)=>(),
        Err(err)=>{
            for cause in err.chain(){
                println!("{}",cause)
            }
        }
    }

    match do_it1(&mut it) {
        Ok(_)=>(),
        Err(err)=>{
            for cause in err.chain(){
                println!("{}",cause)
            }
        }
    }

}

```



```
Failed to detach the important thing
detach failed

Failed to read instrs from 
No such file or directory (os error 2)
```