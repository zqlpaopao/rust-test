一个项目初始化，总是有几个元素是必不可少的、框架、日志、配置文件等等基本元素。
今天我们主要介绍下怎么获取配置并在全局使用

更多好文。vx. golang技术实验室
专注分享 golang、rust等多语言、中间件及大数据相关内容

# 一、读取cargo.toml文件内容

`Cargo.toml` 文件配置如下：

```
[package]
name = "my_test"
version = "0.1.0"
edition = "2021"
authors = ["zhangql"]
```

代码:

```
pub fn cargo_file(){
    let name = env!("CARGO_PKG_NAME");
    let version  = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    println!("{name}\n {version}\n {authors}")
}

my_test
 0.1.0
 zhangql

```



# 二、读取.env文件

项目[根目录](https://so.csdn.net/so/search?q=根目录&spm=1001.2101.3001.7020)新建 `.env` 文件，写入如下代码：

```
DATABASE_URL=mysql://postgres:123456@localhost:3306/test
```

Cargo.toml 文件配置导入以下第三方库：

```
[dependencies]
dotenv = "0.15.0"

```

main.rs

```
use dotenv::dotenv;
use std::env;

fn main() {
    // 在访问环境变量之前检查一下，防止因读取环境变量失败导致程序恐慌。
    // 先把 dotenv 导入，然后在程序开始的地方执行 dotenv() 函数即可，这就会从当前目录或父目录中的 .env 文件中加载环境变量。
    // 如果你想指定其它路径，可以使用 crate 中提供的 from_filename 或 from_path 这两个函数。
    // 好，那么调用 dotenv() 之后为什么还要调用 ok() 方法？
    // 首先，dotenv() 返回的是 Result<PathBuf> 类型，如果返回值不使用的话，就会发出一个警告：
    // 调用 ok() 之后，会把 Result 转化为 Option，而 Option 就不会产生未使用 Result 的警告了。
    // 那么，为什么不使用 unwrap()？
    // 因为在生产环境中，你不会使用 .env 这个文件，你应该使用真实的环境变量，这时 dotenv() 函数就会加载失败，如果使用 unwrap()，那么你的程序就会停止运行。
    // 所以这里使用 ok() 的目的就是当加载 dotenv 环境文件失败的时候可以忽略错误。
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    
    print!("{:?}", database_url);

}
```

运行结果：

```
"mysql://postgres:123456@localhost:3306/test"
```



# 三、读取自定义toml文件

项目根目录新建 config.toml 文件，写入如下代码：

```
[database]
url = "tcp://mysql:123456@localhost/test"

[log]
debug = true
debug_sql = false
log_root = "/tmp"
```


Cargo.toml 文件配置导入以下第三方库：

```
[dependencies]
config = "0.13.1"
toml = "0.5.9"
lazy_static = "1.4"
serde = "1.0"
serde_derive = "1.0"
```



```
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
#[macro_use]
extern crate lazy_static;


pub fn read_toml(){
    let setting = Setting::default();
    println!("{:#?}",setting);
    let c = Setting::get();
    println!("{:#?}",c);

}

#[derive(Debug,Deserialize)]
pub struct Log{
    pub debug : bool,
    pub debug_sql : bool,
    pub log_root : String
}
#[derive(Debug,Deserialize)]
pub struct Database{
    pub url : String
}

#[derive(Debug,Deserialize)]
pub struct Setting{
    pub database : Database,
    pub log : Log
}

impl Default for Setting{
    fn default() -> Self {
        let file_path = "/Users/zhangql/Desktop/rust/my_test/config.toml";

        let mut file = match File::open(file_path){
            Ok(f)=> f,
            Err(e)=>panic!("error is op en config {e}")
        };

        let mut str = String::new();
        println!("{}",str);
        match file.read_to_string(&mut str) {
            Ok(s)=>s,
            Err(e)=>panic!("error read str {}",e)
        };

        toml::from_str(&str).expect("Parsing the configuration file failed")
    }
}

impl Setting{
    pub fn get<'a>()->&'a Self{
        lazy_static!{
            static ref CACHE : Setting = Setting::default();
        }
        &CACHE
    }
}
```



````
Setting {
    database: Database {
        url: "tcp://mysql:123456@localhost/test",
    },
    log: Log {
        debug: true,
        debug_sql: false,
        log_root: "/tmp",
    },
}

Setting {
    database: Database {
        url: "tcp://mysql:123456@localhost/test",
    },
    log: Log {
        debug: true,
        debug_sql: false,
        log_root: "/tmp",
    },
}
````

现在，`CACHE`变量将在首次访问`Setting::get()`方法时进行初始化，并且后续的调用将重复使用已初始化的值。

请注意，为了使`lazy_static`宏正常工作，您需要在代码的顶部添加`#[macro_use] extern crate lazy_static;`。这使得宏在编译时能够正确展开。

使用这种方式，您可以通过调用`Setting::get()`方法来获取全局的`Setting`实例，而无需将其作为全局变量直接暴露出来。

参考文章:
https://blog.csdn.net/xiaohuihui1400/article/details/130801880