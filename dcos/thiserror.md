一个优秀的项目，错误处理的优雅性是至关重要的，而rust，anyhow creat是绕不过去的一个，今天我们来研究下，怎么使用它，帮助我们写出更优雅的代码


关注 vx golang技术实验室，获取更多golang、rust好文


# 一、thiserror初体验

可以使用命令 `cargo add thiserror` 将它添加到自己的项目中，或者在 `Cargo.toml` 中添加如下的配置：

```
[dependencies]
thiserror = "1.0"
```

`thiserror` 可以用于枚举或者结构体，例如，我们来看一个基本的例子：

```
use std::io;
use log::error;
use thiserror::Error;
#[derive(Error,Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?},found {found:?})")]
    InvalidHeader {expected:String,found:String},
    #[error("unknown data store error")]
    Unknown
}

pub fn error(){
    ///error
    println!("这是没有参数的 Unknown {}",DataStoreError::Unknown);
    println!("这是结构体参数的 InvalidHeader {}",DataStoreError::InvalidHeader {
        expected : String::from("expected"),
        found : String::from("found")
    });
    println!("这是有index参数的 Redaction {}",DataStoreError::Redaction(String::from("Redaction")));
    println!("这是有from参数的 Disconnect {}",DataStoreError::Disconnect(io::Error::from(io::ErrorKind::TimedOut)));

}
```



```
这是没有参数的 Unknown unknown data store error
这是结构体参数的 InvalidHeader invalid header (expected "expected",found "found")
这是有index参数的 Redaction the data for key `Redaction` is not available
这是有from参数的 Disconnect data store disconnected
```



然后我们来仔细分析下各种用法

# 二、#[error]

如果使用 `#[error(...)]` 为结构体或者枚举生成自定义错误消息，这将为它们实现 `Display`：

## 2.1 Enum

```rust
#[derive(Debug)]
pub struct Limits{
    lo : i16,
    hi : i16
}

#[derive(Error,Debug)]
pub enum MyError{
    #[error("invalid rdo_lookahead_frames {0} (expected < {})",i8::MAX)]
    InvalidLookahead(u32),
    #[error("first letter must be lowercase but was {:?}",first_char(.0))]
    WrongCase(String),
    #[error("invalid index.html {idx},expected at least {} and at most {}",.limits.lo,.limits.hi)]
    OutOfBounds{idx:usize,limits:Limits}
}

```



```rust
pub fn error(){
    println!("这是 enum 的InvalidLookahead {}",MyError::InvalidLookahead(3333));
    //自动调用函数进行比较
    println!("这是 enum 的 WrongCase {}",MyError::WrongCase("kk".to_string()));
    println!("这是 enum 的 OutOfBounds {}",MyError::OutOfBounds{idx : 89,limits:Limits{
        lo:12,
        hi:11
    }});

}

这是 enum 的InvalidLookahead invalid rdo_lookahead_frames 3333 (expected < 127)
这是 enum 的 WrongCase first letter must be lowercase but was 'k'
这是 enum 的 OutOfBounds invalid index 89,expected at least 12 and at most 11

```

## 2.2 struct

```
#[derive(Error, Debug)]
#[error("something failed, msg is: {msg}")]
pub struct MyErrorStruct {
    msg: &'static str,
}
```



```
println!("这是 struct 的msg  {}",MyErrorStruct{msg:"失败的msg"});


这是 struct 的msg  something failed, msg is: 失败的msg
```



## 2.3 其他结构

其他结构也是支持的，例如 tuple、空struct 等等



# 三、#[from]

可以使用 `#[from]` 注解为错误类型实现 `From`，可以从其他错误生成：

```
#[derive(Error, Debug)]
#[error("some io error happened, {:?}", .source)]
pub struct MyFromError {
    #[from]
    source: io::Error,
}
```



```
println!("这是 struct 的 from 的 {}",MyFromError::from(io::Error::from(io::ErrorKind::TimedOut)));

这是 struct 的 from 的 some io error happened, Kind(TimedOut)
```



# 四、#[source]

可以使用 `#[source]` 属性，或者将字段命名为 `source`，可为自定义错误实现 `source` 方法，返回底层的错误类型：

```
use std::error::Error;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
  #[error("some io error happened, {:?}", .source)]
  IO { source: io::Error },
}

fn main() {
  let err = MyError::IO {
      source: io::Error::from(io::ErrorKind::TimedOut),
  };
  println!("{:?}", err.source());
}


```

或者使用 `#[source]` 属性标记非 `source` 的字段，例如：这里是 `err` 字段：

```
use std::error::Error;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
  #[error("some io error happened, {:?}", .err)]
  IO {
      #[source]
      err: io::Error,
  },
}

fn main() {
  let err = MyError::IO {
      err: io::Error::from(io::ErrorKind::TimedOut),
  };
  println!("{:?}", err.source());
}
```

`#[from]` 和 `#[source]` 二选一即可，`#[from]` 也会为类型生成 `.source()` 方法，例如：

```
#![allow(unused)]
#![feature(backtrace)]

use std::backtrace;
use std::error::Error as _;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("some io error happened, {:?}", .source)]
pub struct MyError {
  #[from]
  source: io::Error,
  backtrace: backtrace::Backtrace,
}

fn main() {
  let err = MyError::from(io::Error::from(io::ErrorKind::TimedOut));
  println!("{:?}", err.source());
}
```



# 五、#[backtrace]

只要在我们的错误结构体里面放个类型为 `std::backtrace::Backtrace` 的字段，就会自动实现 `backtrace()` 方法，可以看 `#[from]`。

另外，如果使用 `#[backtrace]` 标记 `source`（`source` 字段，或者 `#[source]`，或者 `#[from]`），那么 `backtrace()` 方法会转发到 `source` 的 `backtrace`。



# 六、#[error(transparent)]

可以通过 `#[error(transparent)]` 让 `source` 和 `Display` 直接使用底层的错误，这对于那些想处理任何的类型来说是很有用的：

```
use std::io;
use log::error;
use thiserror::Error;
use anyhow::anyhow;
use std::error::Error as _;
#[derive(Error,

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

```



```
//transparent
let err = MyErrorTrans::from(anyhow!("Missing attribute: {}", "field1"));
println!("{}", err);
println!("{:?}", err);

let err = MyErrorTransEnum::from(anyhow!("Missing attribute: {}", "field1"));
println!("{}", err);
println!("{:?}", err);

Missing attribute: field1
MyErrorTrans { source: Missing attribute: field1 }
Missing attribute: field1
Other(Missing attribute: field1)
```