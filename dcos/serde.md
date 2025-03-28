# 1 serde 序列&反序列

[-](https://serde.rs/)

 ## 1. 1 支持数据结构

数据格式

以下是社区已为 Serde 实现的部分数据格式列表。

- [JSON](https://github.com/serde-rs/json)是许多 HTTP API 使用的普遍存在的 JavaScript 对象表示法。
- [Postcard](https://github.com/jamesmunns/postcard)，一种 no_std 和嵌入式系统友好的紧凑二进制格式。
- [CBOR](https://github.com/enarx/ciborium)，一种简洁的二进制对象表示，专为小消息大小而设计，无需版本协商。
- [YAML](https://github.com/dtolnay/serde-yaml)，一种自称为人类友好的配置语言，但不是标记语言。
- [MessagePack](https://github.com/3Hren/msgpack-rust)，一种类似于紧凑 JSON 的高效二进制格式。
- [TOML ， ](https://docs.rs/toml)[Cargo](http://doc.crates.io/manifest.html)使用的最小配置格式。
- [Pickle](https://github.com/birkenfeld/serde-pickle)，Python 世界中常见的格式。
- [RON](https://github.com/ron-rs/ron)，一种 Rusty 对象表示法。
- [BSON](https://github.com/mongodb/bson-rust)，MongoDB 使用的数据存储和网络传输格式。
- [Avro](https://docs.rs/apache-avro)是 Apache Hadoop 中使用的一种二进制格式，支持架构定义。
- [JSON5](https://github.com/callum-oakley/json5-rs)，JSON 的超集，包括 ES5 的一些产品。
- [URL](https://docs.rs/serde_qs)查询字符串，采用 x-www-form-urlencoded 格式。
- [Starlark](https://github.com/dtolnay/serde-starlark)，Bazel 和 Buck 构建系统用于描述构建目标的格式。*（仅限序列化）*
- [Envy](https://github.com/softprops/envy)，一种将环境变量反序列化为 Rust 结构的方法。 *（仅反序列化）*
- [Envy Store](https://github.com/softprops/envy-store)，一种将[AWS Parameter Store](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html)参数反序列化为 Rust 结构的方法。*（仅反序列化）*
- [S-表达式](https://github.com/rotty/lexpr-rs)，Lisp 语言系列使用的代码和数据的文本表示。
- [D-Bus](https://docs.rs/zvariant)的二进制线路格式。
- [FlexBuffers](https://github.com/google/flatbuffers/tree/master/rust/flexbuffers)是 Google 的 FlatBuffers 零拷贝序列化格式的无模式表亲。
- [Bencode](https://github.com/P3KI/bendy)，BitTorrent 协议中使用的简单二进制格式。
- [令牌流](https://github.com/oxidecomputer/serde_tokenstream)，用于处理 Rust 程序宏输入。*（仅反序列化）*
- [DynamoDB Items ， ](https://docs.rs/serde_dynamo)[rusoto_dynamodb](https://docs.rs/rusoto_dynamodb)使用的格式与 DynamoDB 之间传输数据。
- [Hjson](https://github.com/Canop/deser-hjson)，围绕人类阅读和编辑设计的 JSON 语法扩展。 *（仅反序列化）*
- [CSV](https://docs.rs/csv)，逗号分隔值是一种表格文本文件格式。



## 1.2 简单使用

Serde 开箱即用，能够以上述任何格式序列化和反序列化常见 Rust 数据类型。例如`String`，`&str`、`usize`、 `Vec<T>`、`HashMap<K,V>`都支持。此外，Serde 还提供了派生宏来为您自己的程序中的结构生成序列化实现。使用派生宏的方式如下：

Cargo.toml



```
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.96"
```



```

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Point{
    x : i32,
    y : i32
}


pub fn test(){
    let point = Point{
        x : 1,
        y : 2
    };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}",serialized);

    let deserialized : Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}",deserialized);

}

serialized = {"x":1,"y":2}
deserialized = Point { x: 1, y: 2 }
```





## 1.33 serde features

serde 是一个用于序列化和反序列化数据的 Rust 库，它提供了一组功能强大的特性，可以根据不同的需求进行配置。以下是一些常用的 serde 特性：

- `derive` 特性：serde 提供了 `#[derive(Serialize, Deserialize)]` 宏，可以自动为结构体或枚举类型实现 `Serialize` 和 `Deserialize` trait，简化了序列化和反序列化的过程。
- `json` 特性：启用 `json` 特性后，serde 可以序列化和反序列化 JSON 格式的数据。需要在 Cargo.toml 文件中添加 `serde_json` 依赖并启用 `json` 特性。
- `yaml` 特性：启用 `yaml` 特性后，serde 可以序列化和反序列化 YAML 格式的数据。需要在 Cargo.toml 文件中添加 `serde_yaml` 依赖并启用 `yaml` 特性。
- `bincode` 特性：启用 `bincode` 特性后，serde 可以使用 bincode 格式进行高效的二进制序列化和反序列化。需要在 Cargo.toml 文件中添加 `serde` 和 `bincode` 依赖并启用 `bincode` 特性。
- `toml` 特性：启用 `toml` 特性后，serde 可以序列化和反序列化 TOML 格式的数据。需要在 Cargo.toml 文件中添加 `serde_toml` 依赖并启用 `toml` 特性。
- `xml` 特性：启用 `xml` 特性后，serde 可以序列化和反序列化 XML 格式的数据。需要在 Cargo.toml 文件中添加 `serde_xml_rs` 依赖并启用 `xml` 特性。
- `uuid` 特性：启用 `uuid` 特性后，serde 可以序列化和反序列化 UUID 类型。

这只是一些常见的 serde 特性示例，实际上 serde 还提供了更多的特性和扩展，可以根据具体需求进行选择和配置。



# 2 Serde 数据模型

Serde 数据模型的序列化部分由特征定义 [`Serializer`](https://docs.rs/serde/1/serde/trait.Serializer.html)，反序列化部分由特征定义 [`Deserializer`](https://docs.rs/serde/1/serde/trait.Deserializer.html)。这些是将每个 Rust 数据结构映射到 29 种可能类型之一的方法。特征的每个方法`Serializer`对应于数据模型的一种类型。



## 2.1 Serde 数据模型

- 14 基础类型

  - bool
  - i8, i16, i32, i64, i128
  - u8, u16, u32, u64, u128
  - f32, f64
  - char

- string

  - 具有长度且无空终止符的 UTF-8 字节。可能包含 0 字节。
  - 序列化时，所有字符串都会被同等处理。反序列化时，字符串分为三种类型：瞬态字符串、自有字符串和借用字符串。这种区别在[了解反序列化器生命周期](https://serde.rs/lifetimes.html)中进行了解释，并且是 Serde 实现高效零拷贝反序列化的关键方式。

- byte array

   

  \- [u8]

  - 与字符串类似，在反序列化期间，字节数组可以是瞬态的、拥有的或借用的。

- option

  - 要么没有，要么有一些值。

- unit

  - Rust 中的类型`()`。它表示不包含数据的匿名值。

- unit_struct

  - 例如`struct Unit`或`PhantomData<T>`. 它表示不包含数据的命名值。

- unit_variant

  - 例如中的`E::A`和。`E::B``enum E { A, B }`

- newtype_struct

  - 例如`struct Millimeters(u8)`。

- newtype_variant

  - 例如`E::N`在`enum E { N(u8) }`.

- seq

  - 大小可变的异构值序列，例如`Vec<T>`或 `HashSet<T>`。序列化时，在迭代所有数据之前可能知道也可能不知道长度。反序列化时，通过查看序列化数据来确定长度。请注意，同类 Rust 集合`vec![Value::Bool(true), Value::Char('c')]`可能会序列化为异构 Serde seq，在本例中包含 Serde bool 后跟 Serde char。

- tuple

  - 静态大小的异构值序列，其长度在反序列化时无需查看序列化数据即可知道，例如`(u8,)`或`(String, u64, Vec<T>)`或`[u64; 10]`。

- tuple_struct

  - 例如，命名元组`struct Rgb(u8, u8, u8)`。

- tuple_variant

  - 例如`E::T`在`enum E { T(u8, u8) }`.

- map

  - 例如，可变大小的异构键值对`BTreeMap<K, V>`。序列化时，在迭代所有条目之前可能知道也可能不知道长度。反序列化时，通过查看序列化数据来确定长度。

- struct

  - 静态大小的异构键值对，其中键是编译时常量字符串，并且在反序列化时无需查看序列化数据即可知道，例如`struct S { r: u8, g: u8, b: u8 }`。

- struct_variant

  - 例如`E::S`在`enum E { S { r: u8, g: u8, b: u8 } }`.



## 2.2 Serialize数据模型

对于大多数 Rust 类型，它们到 Serde 数据模型的映射非常简单。例如，Rust`bool`类型对应于 Serde 的 bool 类型。Rust 元组结构`Rgb(u8, u8, u8)`对应于 Serde 的元组结构类型。

但没有根本原因表明这些映射需要简单明了。和特征可以执行 Rust 类型和 Serde 数据模型之间适合用例的[`Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html)任何*映射。*[`Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html)

作为一个例子，考虑 Rust 的[`std::ffi::OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html)类型。该类型表示平台本机字符串。在 Unix 系统上，它们是任意非零字节，在 Windows 系统上，它们是任意非零 16 位值。`OsString`将Serde 数据模型映射为以下类型之一似乎很自然：

- 作为 Serde**字符串**。`OsString`不幸的是，序列化会很脆弱，因为不能保证an 可以用 UTF-8 表示，而反序列化也会很脆弱，因为 Serde 字符串允许包含 0 字节。
- 作为 Serde**字节数组**。这解决了使用 string 的两个问题，但现在如果我们`OsString`在 Unix 上序列化 an 并在 Windows 上反序列化它，我们最终会得到[错误的 string](https://www.joelonsoftware.com/2003/10/08/the-absolute-minimum-every-software-developer-absolutely-positively-must-know-about-unicode-and-character-sets-no-excuses/)。

相反，`Serialize`和`Deserialize`impls 通过将其视为Serde **enum**`OsString`来映射到 Serde 数据模型。实际上，它的行为就好像被定义为以下类型，即使这与任何单独平台上的定义都不匹配。`OsString``OsString`

```rust
enum OsString {
    Unix(Vec<u8>),
    Windows(Vec<u16>),
    // and other platforms
}
```

映射到 Serde 数据模型的灵活性是深刻而强大的。在实现`Serialize`和时`Deserialize`，请注意您的类型的更广泛的上下文，这可能会使最本能的映射不是最佳选择。



# 3 使用derive

Serde 提供了一个派生宏来生成您的板条箱中定义的数据结构的`Serialize`和 特征的实现，从而允许它们以所有 Serde 数据格式方便地表示。`Deserialize`

**仅当您的代码使用`#[derive(Serialize, Deserialize)]`.**

此功能基于 Rust 的`#[derive]`机制，就像您用来自动派生内置`Clone`、 `Copy`、`Debug`或其他特征的实现一样。它能够为大多数结构和枚举生成实现，包括具有复杂泛型类型或特征边界的结构和枚举。在极少数情况下，对于特别复杂的类型，您可能需要 [手动实现这些特征](https://serde.rs/custom-serialization.html)。

这些派生需要 Rust 编译器版本 1.31 或更高版本。

- 在 Cargo.toml 中添加`serde = { version = "1.0", features = ["derive"] }`为依赖项。
- 确保所有其他基于 Serde 的依赖项（例如 serde_json）均位于与 serde 1.0 兼容的版本上。
- 在要序列化的结构体和枚举上，在 `use serde::Serialize;`同一模块中导入派生宏并 `#[derive(Serialize)]`在结构体或枚举上写入。
- 同样导入`use serde::Deserialize;`并写入`#[derive(Deserialize)]` 要反序列化的结构和枚举。

这里是`Cargo.toml`：

Cargo.toml

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }

# serde_json is just for the example, not required in general
serde_json = "1.0"
```

现在`src/main.rs`使用Serde的自定义导出：

src/main.rs

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
```

这是输出：

```
$ cargo run
serialized = {"x":1,"y":2}
deserialized = Point { x: 1, y: 2 }
```

## 3.1 故障排除

有时您可能会看到编译时错误，告诉您：

```
the trait `serde::ser::Serialize` is not implemented for `...`
```

即使结构或枚举显然有`#[derive(Serialize)]`它。

这几乎总是意味着您正在使用依赖于不兼容版本的 Serde 的库。您可能在 Cargo.toml 中依赖于 serde 1.0，但使用了依赖于 serde 0.9 的其他库。因此， `Serialize`serde 1.0 中的特征可以实现，但库期望`Serialize`从 serde 0.9 中实现该特征。从 Rust 编译器的角度来看，这些是完全不同的特征。

修复方法是根据需要升级或降级库，直到 Serde 版本匹配。该`cargo tree -d`命令有助于查找所有重复依赖项被拉入的位置。



# 4 属性

[属性](https://doc.rust-lang.org/book/attributes.html)用于定制 Serde 派生生成的`Serialize`和实现。`Deserialize`它们需要 Rust 编译器版本 1.15 或更高版本。

属性分为三类：

- [**容器属性**](https://serde.rs/container-attrs.html)- 适用于结构或枚举声明。
- [**变体属性**](https://serde.rs/variant-attrs.html)——适用于枚举的变体。
- [**字段属性**](https://serde.rs/field-attrs.html)— 适用于结构体或枚举变体中的一个字段。

```rust
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // <-- this is a container attribute
struct S {
    #[serde(default)]  // <-- this is a field attribute
    f: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "e")]  // <-- this is also a container attribute
enum E {
    #[serde(rename = "a")]  // <-- this is a variant attribute
    A(String),
}
```

请注意，单个结构体、枚举、变体或字段可能具有多个属性。



## 4.1 容器属性

### 4.1.1 `#[serde(rename = "name")]`

**使用嵌套的格式，指定特定名称**

使用给定名称而不是 Rust 名称来序列化和反序列化此结构体或枚举。

允许为序列化与反序列化指定独立的名称：

- `#[serde(rename(serialize = "ser_name"))]`
- `#[serde(rename(deserialize = "de_name"))]`
- `#[serde(rename(serialize = "ser_name", deserialize = "de_name"))]`

我们看下sericlize

```

#[derive(Serialize,Deserialize,Debug)]
struct Person{
    #[serde(rename(serialize = "ser_name"))]
    z : Z

}
#[derive(Serialize,Deserialize,Debug)]
struct Z {
    name : String,
    age : i8
}

//容器的rename
fn container_rename(){
    let p = Person{
        z : Z{
            name :"ZHangQL".to_string(),
            age:18,
        }
    };

    let ser = serde_json::to_string(&p).unwrap();
    println!("ser {}",ser);

    let des : Person = serde_json::from_str(&ser).unwrap();
    println!("des {:?}",des);

}


ser {"ser_name":{"name":"ZHangQL","age":18}}
thread 'main' panicked at src/controller/serde.rs:42:51:
called `Result::unwrap()` on an `Err` value: Error("missing field `z`", line: 1, column: 40)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

为什么报错了，因为serde默认找不到全部rust的字段的时候就会报错



下面我们加上 看下效果

```

#[derive(Serialize,Deserialize,Debug)]
struct Person{
    #[serde(rename(serialize = "ser_name",deserialize="ser_name"))]
    z : Z

}
#[derive(Serialize,Deserialize,Debug)]
struct Z {
    name : String,
    age : i8
}

//容器的rename
fn container_rename(){
    let p = Person{
        z : Z{
            name :"ZHangQL".to_string(),
            age:18,
        }
    };

    let ser = serde_json::to_string(&p).unwrap();
    println!("ser {}",ser);

    let des : Person = serde_json::from_str(&ser).unwrap();
    println!("des {:?}",des);

}

ser {"ser_name":{"name":"ZHangQL","age":18}}
des Person { z: Z { name: "ZHangQL", age: 18 } }
```

报错消失



### 4.1.2  `#[serde(rename_all = "...")]`

`rename_all` 属性接受一个字符串参数，用于指定命名策略。常见的命名策略包括：

- `"lowercase"`：将字段名转换为小写形式。
- `"UPPERCASE"`：将字段名转换为大写形式。
- `"snake_case"`：将字段名转换为蛇形命名法（例如，`first_name`）。
- `"SCREAMING_SNAKE_CASE"`：将字段名转换为大写蛇形命名法（例如，`FIRST_NAME`）。
- `"camelCase"`：将字段名转换为驼峰命名法（例如，`firstName`）。
- `"PascalCase"`：将字段名转换为帕斯卡命名法（例如，`FirstName`）。

```

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all ( serialize = "UPPERCASE"))]
struct Z1 {
    user_name : String,
    user_age : i8
}


#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all ( deserialize = "lowercase"))]
struct Z2 {
    user_name : String,
    user_age : i8
}

//容器的rename
fn container_rename(){
    let p = Z1{
            user_name :"ZHangQL".to_string(),
            user_age:18,
        };

    let ser = serde_json::to_string(&p).unwrap();
    println!("ser {}",ser);

    let des : Z2 = serde_json::from_str(&ser).unwrap();
    println!("des {:?}",des);

}

ser {"user_name":"ZHangQL","user_age":18}
des Z2 { user_name: "ZHangQL", user_age: 18 }
```

des的时候转换为小写没生效



### 4.1.3  `#[serde(deny_unknown_fields)]`

`#[serde(deny_unknown_fields)]` 是 `serde` 库提供的一个属性，用于在序列化和反序列化过程中禁止处理未知字段。当使用该属性时，`serde` 将会在遇到未知字段时产生一个错误，而不是忽略这些字段。

```

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct User {
    user_name: String,
    user_age: i32,
}
fn deny_unknown_fields(){
    let json = r#"
        {
            "user_name": "John",
            "user_age": 25,
            "unknown_field": "value"
        }
    "#;

    let result: Result<User, serde_json::Error> = serde_json::from_str(json);
    match result {
        Ok(user) => println!("Deserialized: {:?}", user),
        Err(err) => println!("Error: {}", err),
    }
}
Error: unknown field `unknown_field`, expected `user_name` or `user_age` at line 5 column 27

如果没加 回忽略这个字段
Deserialized: User { user_name: "John", user_age: 25 }

```



### 4.1.4 `#[serde(tag = "type")]`

`#[serde(tag = "type")]` 是 `serde` 库提供的一个属性，用于在序列化和反序列化过程中使用标签字段来标识不同的变体类型。这种模式被称为"tagged"序列化和反序列化。

在使用 `#[serde(tag = "type")]` 属性时，你需要为包含不同变体的结构体定义一个标签字段，该字段的值将用于标识不同的变体类型。通常，标签字段是一个字符串，但也可以是其他可序列化的类型。

以下是一个示例代码，演示了如何使用 `#[serde(tag = "type")]` 属性：

```

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Shape {
    #[serde(rename = "circle")]
    Circle {
        radius: f64,
        #[serde(default)]
        color: String,
    },
    #[serde(rename = "rectangle")]
    Rectangle {
        width: f64,
        height: f64,
        #[serde(default)]
        color: String,
    },
}



fn r#type(){
    let circle = Shape::Circle {
        radius: 5.0,
        color: "red".to_string(),
    };

    let serialized = serde_json::to_string(&circle).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Shape = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

Serialized: {"type":"circle","radius":5.0,"color":"red"}
Deserialized: Circle { radius: 5.0, color: "red" }
```

在上述示例中，我们定义了一个 `Shape` 枚举，其中包含了两个变体：`Circle` 和 `Rectangle`。我们使用 `#[serde(tag = "type")]` 属性为枚举定义了一个标签字段 `type`，用于标识不同的变体类型。在序列化时，`serde` 将使用标签字段来标识变体类型；在反序列化时，`serde` 将根据标签字段的值来确定要创建的变体类型。



### 4.1.5 `#[serde(tag = "t", content = "c")]`

相当于在向上封装一层

`#[serde(tag = "t", content = "c")]` 是 `serde` 库提供的一个属性，用于在序列化和反序列化过程中使用标签字段和内容字段来标识和包装不同的变体类型。这种模式被称为"tagged"序列化和反序列化，其中标签字段用于标识变体类型，内容字段用于包装变体的数据。

在使用 `#[serde(tag = "t", content = "c")]` 属性时，你需要为包含不同变体的结构体或枚举定义一个标签字段和内容字段。标签字段的值将用于标识不同的变体类型，而内容字段将用于包装变体的数据。

```

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
enum Shapes {
    #[serde(rename = "circle")]
    Circle {
        #[serde(rename = "r")]
        radius: f64,
        #[serde(default)]
        color: String,
    },
    #[serde(rename = "rectangle")]
    Rectangle {
        #[serde(rename = "w")]
        width: f64,
        #[serde(rename = "h")]
        height: f64,
        #[serde(default)]
        color: String,
    },
}

fn tag_content() {
    let circle = Shapes::Circle {
        radius: 5.0,
        color: "red".to_string(),
    };

    let serialized = serde_json::to_string(&circle).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Shapes = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
Serialized: {"t":"circle","c":{"r":5.0,"color":"red"}}
Deserialized: Circle { radius: 5.0, color: "red" }
```





### 4.1.6 `#[serde(untagged)]`

`#[serde(untagged)]` 是 `serde` 库提供的一个属性，用于在序列化和反序列化过程中处理不带标签的变体类型。这种模式被称为"untagged"序列化和反序列化。

当你使用 `#[serde(untagged)]` 属性时，`serde` 将不会使用标签字段来标识不同的变体类型。相反，它会根据变体的结构来决定如何序列化和反序列化数据。

```
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Shape2 {
    Circle {
        radius: f64,
        #[serde(default)]
        color: String,
    },
    Rectangle {
        width: f64,
        height: f64,
        #[serde(default)]
        color: String,
    },
}

fn untagged() {
    let circle = Shape2::Circle {
        radius: 5.0,
        color: "red".to_string(),
    };

    let serialized = serde_json::to_string(&circle).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Shape2 = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

Serialized: {"radius":5.0,"color":"red"}
Deserialized: Circle { radius: 5.0, color: "red" }
```

在上述示例中，我们定义了一个 `Shape` 枚举，其中包含了两个变体：`Circle` 和 `Rectangle`。我们使用 `#[serde(untagged)]` 属性告诉 `serde` 在序列化和反序列化时不使用标签字段。相反，它会根据变体的结构来决定如何处理数据。

在序列化时，`serde` 将直接将变体的字段序列化为 JSON 对象的字段。在反序列化时，`serde` 将根据 JSON 对象的字段来确定要创建的变体类型。

和不加`tag`、`content`是一样的效果



### 4.1.7 `#[serde(bound = "T: MyTrait")]`

`#[serde(bound = "T: MyTrait")]` 是 `serde` 库提供的一个属性，用于为泛型类型参数指定序列化和反序列化时的约束条件。

在 Rust 中，泛型类型参数可以有约束条件，以限定它们必须满足某些特定的 trait。`#[serde(bound = "T: MyTrait")]` 属性允许你在序列化和反序列化过程中对泛型类型参数进行约束。

允许为序列化与反序列化指定独立的边界：

- `#[serde(bound(serialize = "T: MySerTrait"))]`
- `#[serde(bound(deserialize = "T: MyDeTrait"))]`
- `#[serde(bound(serialize = "T: MySerTrait", deserialize = "T: MyDeTrait"))]`

```

trait MyTrait {
    fn do_something(&self);
}

#[derive(Serialize, Deserialize,Debug)]
struct MyStruct<T>
    where
        T: MyTrait,
{
    #[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
    data: T,
}

impl<T> MyStruct<T>
    where
        T: MyTrait,
{
    fn new(data: T) -> MyStruct<T> {
        MyStruct { data }
    }
}

impl MyTrait for u32 {
    fn do_something(&self) {
        println!("Doing something with u32 {}", self);
    }
}

fn bound() {
    let my_struct = MyStruct::new(42);
    let serialized = serde_json::to_string(&my_struct).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: MyStruct<u32> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);
}
```



```
serialized: {"data":42}
deserialized: MyStruct { data: 42 }
```

其他类型会有问题



### 4.1.8 `#[serde(default)]`

反序列化时，任何缺失的字段都应该从结构体的实现中填充`Default`。只允许在结构上。



```

#[derive(Serialize,Deserialize,Debug)]
struct MyStruct1{
    #[serde(default)]
    value : String,
    #[serde(default)]
    age : i8
}

fn default(){
    let deserialized : MyStruct1 = serde_json::from_str(&r#"{}"#).unwrap();
    println!("deserialized {:?}",deserialized);

    let serialized = serde_json::to_string(&deserialized).unwrap();
    println!("serialized {}",serialized);
}
deserialized MyStruct1 { value: "", age: 0 }
serialized {"value":"","age":0}
```

此属性知能用在字段上，用在结构体是无效的



### 4.1.9 `#[serde(default = "path")]` 默认值调用函数

`#[serde(default = "path")]` 属性用于在序列化和反序列化过程中处理缺失的字段，并使用指定的函数 `path` 来提供默认值。

```
fn default_value() -> i32 {
    42
}

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct2 {
    #[serde(default = "default_value")]
    value: i32,
}

fn default_path() {
    // 反序列化
    let deserialized: MyStruct2 = serde_json::from_str(r#"{}"#).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // 序列化
    let serialized = serde_json::to_string(&MyStruct2 { value: 10 }).unwrap();
    println!("Serialized: {}", serialized);
}

Deserialized: MyStruct2 { value: 42 }
Serialized: {"value":10}
```



### 4.1.10 `#[serde(remote = "...")]`

`#[serde(remote = "...")]` 属性用于在序列化和反序列化过程中指定一个远程类型。这个属性通常与 `#[serde(with = "...")]` 属性一起使用，用于指定在序列化和反序列化时要使用的远程类型。

```

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "MyStruct3")]
struct MyStructRemote {
    value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "MyStructRemote")]
struct MyStruct3 {
    #[serde(default)]
    value: i32,
}

fn remote() {
    // 反序列化
    let deserialized: MyStruct3 = serde_json::from_str(r#"{"value": 10}"#).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // 序列化
    let serialized = serde_json::to_string(&MyStruct3 { value: 42 }).unwrap();
    println!("Serialized: {}", serialized);
}
Deserialized: MyStruct { value: 10 }
Serialized: {"value":42}
```



### 4.1.11 -------->`#[serde(transparent)]`

序列化和反序列化一个新类型结构体或带有一个字段的大括号结构体，就像其一个字段自行序列化和反序列化一样。类似于`#[repr(transparent)]`.

`#[serde(transparent)]` 是一个属性，用于告诉 Serde 序列化和反序列化过程中保持字段的透明性。当一个结构体或枚举只有一个字段时，可以使用 `#[serde(transparent)]` 属性来表示该字段应该直接与外部类型进行交互，而不会引入额外的嵌套层级。

这个属性在以下情况下特别有用：

1. 当您希望在序列化和反序列化过程中保持类型的透明性，即将类型视为其字段的直接代理。
2. 当您希望消除类型之间的嵌套层级，从而简化序列化和反序列化的代码。



### 4.1.12  `#[serde(from = "FromType")]`

`#[serde(from = "FromType")]` 是一个属性，用于指定在反序列化时将数据从另一个类型 `FromType` 转换为当前类型。这在某些情况下很有用，当您需要从一种数据表示形式转换为另一种数据表示形式时。

例如，假设您有两个类型 `FromType` 和 `MyStruct`，您希望从 `FromType` 反序列化到 `MyStruct`。您可以使用 `#[serde(from = "FromType")]` 属性来实现这个转换。

```

#[derive(Debug, Deserialize)]
struct FromType {
    value: i32,
}

#[derive(Debug)]
struct MyStruct4 {
    value: i32,
}

impl From<FromType> for MyStruct4 {
    fn from(from: FromType) -> Self {
        MyStruct4 { value: from.value }
    }
}

fn from() {
    let from_type: FromType = serde_json::from_str(r#"{"value": 42}"#).unwrap();
    let my_struct: MyStruct4 = from_type.into();
    println!("{:?}", my_struct);
}

MyStruct4 { value: 42 }
```

在上面的示例中，我们定义了 `FromType` 和 `MyStruct` 两个结构体。然后，我们实现了 `From<FromType> for MyStruct` trait，将 `FromType` 转换为 `MyStruct`。在 `main` 函数中，我们首先从 JSON 字符串反序列化为 `FromType`，然后通过 `into()` 方法将其转换为 `MyStruct`。



### 4.1.13 #[serde(try_from = "FromType")]

`#[serde(try_from = "FromType")]` 是一个属性，用于指定在反序列化时尝试将数据从另一个类型 `FromType` 转换为当前类型。与 `#[serde(from = "FromType")]` 不同的是，`try_from` 属性表示转换可能会失败，因此返回一个 `Result` 类型。

```
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Deserialize)]
struct FromType {
    value: i32,
}

#[derive(Debug)]
struct MyStruct {
    value: i32,
}

impl TryFrom<FromType> for MyStruct {
    type Error = ();

    fn try_from(from: FromType) -> Result<Self, Self::Error> {
        if from.value >= 0 {
            Ok(MyStruct { value: from.value })
        } else {
            Err(())
        }
    }
}

fn main() {
    let from_type: FromType = serde_json::from_str(r#"{"value": 42}"#).unwrap();
    let my_struct: Result<MyStruct, ()> = MyStruct::try_from(from_type);
    
    match my_struct {
        Ok(my_struct) => println!("{:?}", my_struct),
        Err(()) => println!("Conversion failed"),
    }
}

```

在上面的示例中，我们定义了 `FromType` 和 `MyStruct` 两个结构体。然后，我们实现了 `TryFrom<FromType> for MyStruct` trait，将 `FromType` 转换为 `MyStruct`。在 `try_from` 方法中，我们检查 `FromType` 的值是否大于等于 0，如果是，则返回 `Ok` 包装的 `MyStruct`，否则返回 `Err`。

在 `main` 函数中，我们首先从 JSON 字符串反序列化为 `FromType`，然后使用 `MyStruct::try_from` 方法尝试将其转换为 `MyStruct`。由于值为 42，转换成功，因此打印出 `MyStruct` 的调试输出。如果值为负数，转换将失败，打印出 "Conversion failed"。



### 4.1.14  `#[serde(into = "IntoType")]`

`#[serde(into = "IntoType")]` 是一个属性，用于指定在序列化时将当前类型转换为另一个类型 `IntoType`。这在某些情况下很有用，当您希望将数据从一种类型转换为另一种类型以便进行序列化时。

```
use serde::Serialize;

#[derive(Debug, Serialize)]
struct IntoType {
    value: i32,
}

#[derive(Debug)]
struct MyStruct {
    value: i32,
}

impl Into<IntoType> for MyStruct {
    fn into(self) -> IntoType {
        IntoType { value: self.value }
    }
}

fn main() {
    let my_struct = MyStruct { value: 42 };
    let into_type: IntoType = my_struct.into();
    let json_string = serde_json::to_string(&into_type).unwrap();
    println!("{}", json_string);
}

```

在上面的示例中，我们定义了 `IntoType` 和 `MyStruct` 两个结构体。然后，我们实现了 `Into<IntoType> for MyStruct` trait，将 `MyStruct` 转换为 `IntoType`。在 `into()` 方法中，我们创建一个新的 `IntoType` 实例，并将 `MyStruct` 的值复制到其中。

在 `main` 函数中，我们创建了一个 `MyStruct` 实例，然后通过 `into()` 方法将其转换为 `IntoType`。然后，我们使用 `serde_json::to_string` 将 `IntoType` 序列化为 JSON 字符串，并打印出结果。



### 4.1.15  `#[serde(crate = "...")]`

`#[serde(crate = "...")]` 是一个属性，用于指定在哪个 crate 中查找 `serde` 宏和相关的依赖项。这对于在使用不同版本的 `serde` 或自定义的 `serde` 衍生宏时非常有用。

```
#[serde(crate = "my_serde")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct MyStruct {
    value: i32,
}

fn main() {
    let my_struct = MyStruct { value: 42 };
    let json_string = my_serde::to_string(&my_struct).unwrap();
    println!("{}", json_string);
}

```



在上面的示例中，我们使用 `#[serde(crate = "my_serde")]` 属性指定了自定义的 `serde` crate。然后，我们在 `MyStruct` 结构体上使用 `serde::Serialize` 和 `serde::Deserialize` trait 来自动实现序列化和反序列化。

在 `main` 函数中，我们创建了一个 `MyStruct` 实例，并使用自定义的 `my_serde::to_string` 函数将其序列化为 JSON 字符串。请注意，这里使用的是自定义的 `my_serde` crate 中的 `to_string` 函数。

要使此示例工作，您需要确保在 Cargo.toml 文件中添加了对自定义 `serde` crate 的依赖，并指定正确的版本。请根据您自己的情况进行适当的修改。

```toml
[dependencies]
my_serde = { version = "1.0", path = "/path/to/my_serde" }
```

这样，您就可以成功地在代码中使用自定义的 `serde` crate，并使用 `#[serde(crate = "...")]` 属性来指定其位置。



### 4.1.16 `#[serde(expecting = "...")]`

`#[serde(expecting = "...")]` 是一个属性，用于在反序列化时指定期望的数据格式。它可以用于在遇到不匹配的数据时提供更具体的错误消息。

以下是一个示例：

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    #[serde(expecting = "String")]
    name: String,
    #[serde(expecting = "u32")]
    age: u32,
}

fn main() {
    let json_string = r#"
        {
            "name": 42,
            "age": "John"
        }
    "#;

    let result: Result<Person, serde_json::Error> = serde_json::from_str(json_string);
    match result {
        Ok(person) => println!("{:?}", person),
        Err(error) => println!("{}", error),
    }
}
```

在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `name` 和 `age` 字段。在 `name` 字段上，我们使用 `#[serde(expecting = "String")]` 属性指定了期望的数据类型为字符串。同样，在 `age` 字段上，我们使用 `#[serde(expecting = "u32")]` 属性指定了期望的数据类型为无符号整数。

然后，我们使用 `serde_json::from_str` 函数将 JSON 字符串反序列化为 `Person` 结构体。如果遇到不匹配的数据类型，将会返回一个错误。我们在错误处理中打印出错误消息。

在上述示例中，JSON 字符串中的 `name` 字段的值是一个数字，而不是字符串。因此，反序列化过程会失败，并打印出相应的错误消息。

通过使用 `#[serde(expecting = "...")]` 属性，您可以提供更具体的期望数据类型，以帮助识别和调试反序列化错误。





## 4.2 变体属性

### 4.2.1 `#[serde(rename = "name")]`

如上

使用给定名称而不是 Rust 名称序列化和反序列化此变体。

允许为序列化与反序列化指定独立的名称：

- `#[serde(rename(serialize = "ser_name"))]`
- `#[serde(rename(deserialize = "de_name"))]`
- `#[serde(rename(serialize = "ser_name", deserialize = "de_name"))]`



### 4.2.2 `#[serde(alias = "name")]`

`#[serde(alias = "name")]` 是一个属性，用于在序列化和反序列化时指定字段的别名。它可以用于将 Rust 结构体字段与 JSON 键或其他数据格式中的不同名称进行映射。

```

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person2 {
    #[serde(alias = "name")]
    full_name: String,
    age: u32,
}

fn alias() {
    let person = Person2 {
        full_name: "John Doe".to_owned(),
        age: 30,
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("{}", json_string);

    let json = r#"{
        "name": "Jane Smith",
        "age": 25
    }"#;

    let deserialized_person: Person2 = serde_json::from_str(json).unwrap();
    println!("{:?}", deserialized_person);
}

{"fullName":"John Doe","age":30}
Person2 { full_name: "Jane Smith", age: 25 }
```

在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `full_name` 和 `age` 字段。在 `full_name` 字段上，我们使用 `#[serde(alias = "name")]` 属性指定了一个别名为 "name"。这意味着在序列化和反序列化时，我们可以使用 "name" 作为键来表示 `full_name` 字段。

### 4.2.3 `#[serde(rename_all = "...")]`

根据给定的大小写约定重命名此结构体变体的所有字段。可能的值为`"lowercase"`, `"UPPERCASE"`, `"PascalCase"`, `"camelCase"`, `"snake_case"`, `"SCREAMING_SNAKE_CASE"`, `"kebab-case"`, `"SCREAMING-KEBAB-CASE"`。

允许为序列化与反序列化指定独立的情况：

- `#[serde(rename_all(serialize = "..."))]`
- `#[serde(rename_all(deserialize = "..."))]`
- `#[serde(rename_all(serialize = "...", deserialize = "..."))]`



### 4.2.4 `#[serde(skip)]`

切勿序列化或反序列化此变体。

`#[serde(skip)]` 是一个属性，用于在序列化和反序列化时跳过特定的字段。通过使用 `#[serde(skip)]` 属性，您可以指示 Serde 库在处理序列化和反序列化时忽略该字段。

以下是一个示例：

```rust
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    #[serde(skip)]
    secret_info: String,
}

fn main() {
    let person = Person {
        name: "John Doe".to_owned(),
        age: 30,
        secret_info: "This is a secret".to_owned(),
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("{}", json_string);
}
{"name":"John Doe","age":30}
```

在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `name`、`age` 和 `secret_info` 字段。在 `secret_info` 字段上，我们使用 `#[serde(skip)]` 属性来指示 Serde 库在序列化和反序列化时跳过该字段。

在 `main` 函数中，我们创建了一个 `Person` 结构体的实例，并将其序列化为 JSON 字符串。由于我们使用了 `#[serde(skip)]` 属性，所以在序列化过程中，`secret_info` 字段将被忽略，不会出现在生成的 JSON 字符串中。

通过使用 `#[serde(skip)]` 属性，您可以选择性地排除某些字段，以便在序列化和反序列化过程中忽略它们。这对于保护敏感信息或排除不必要的字段很有用。



### 4.2.5 `#[serde(skip_serializing)]`

`#[serde(skip_serializing)]` 是一个属性，用于在序列化时跳过特定的字段。通过使用 `#[serde(skip_serializing)]` 属性，您可以指示 Serde 库在处理序列化时忽略该字段，但在反序列化时仍会使用该字段。

以下是一个示例：

```rust
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    #[serde(skip_serializing)]
    secret_info: String,
}

fn main() {
    let person = Person {
        name: "John Doe".to_owned(),
        age: 30,
        secret_info: "This is a secret".to_owned(),
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("{}", json_string);

    let json = r#"{
        "name": "Jane Smith",
        "age": 25,
        "secret_info": "This is a secret"
    }"#;

    let deserialized_person: Person = serde_json::from_str(json).unwrap();
    println!("{:?}", deserialized_person);
}

{"name":"John Doe","age":30}
Person { name: "Jane Smith", age: 25, secret_info: "This is a secret" }
```

在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `name`、`age` 和 `secret_info` 字段。在 `secret_info` 字段上，我们使用 `#[serde(skip_serializing)]` 属性来指示 Serde 库在序列化时跳过该字段。

在 `main` 函数中，我们创建了一个 `Person` 结构体的实例，并将其序列化为 JSON 字符串。由于我们使用了 `#[serde(skip_serializing)]` 属性，所以在序列化过程中，`secret_info` 字段将被忽略，不会出现在生成的 JSON 字符串中。

然而，在反序列化时，`secret_info` 字段仍然会被使用。在示例中，我们使用包含 "secret_info" 键的 JSON 字符串进行反序列化，并成功地将其转换为 `Person` 结构体。

通过使用 `#[serde(skip_serializing)]` 属性，您可以选择性地在序列化过程中排除某些字段，但仍然保留它们在反序列化过程中的使用。这对于在序列化时隐藏某些字段的值，但在反序列化时仍然需要使用它们很有用。



### 4.2.6 `#[serde(skip_deserializing)]`

`#[serde(skip_deserializing)]` 是一个属性，用于在反序列化时跳过特定的字段。通过使用 `#[serde(skip_deserializing)]` 属性，您可以指示 Serde 库在处理反序列化时忽略该字段，但在序列化时仍会使用该字段。

以下是一个示例：

```rust
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    #[serde(skip_deserializing)]
    secret_info: String,
}

fn main() {
    let json = r#"{
        "name": "John Doe",
        "age": 30,
        "secret_info": "This is a secret"
    }"#;

    let deserialized_person: Person = serde_json::from_str(json).unwrap();
    println!("{:?}", deserialized_person);

    let person = Person {
        name: "Jane Smith".to_owned(),
        age: 25,
        secret_info: "This is a secret".to_owned(),
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("{}", json_string);
}

Person6 { name: "John Doe", age: 30, secret_info: "" }
{"name":"Jane Smith","age":25,"secret_info":"This is a secret"}
```

在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `name`、`age` 和 `secret_info` 字段。在 `secret_info` 字段上，我们使用 `#[serde(skip_deserializing)]` 属性来指示 Serde 库在反序列化时跳过该字段。

在 `main` 函数中，我们使用包含 "secret_info" 键的 JSON 字符串进行反序列化，并成功地将其转换为 `Person` 结构体。由于我们使用了 `#[serde(skip_deserializing)]` 属性，所以在反序列化过程中，`secret_info` 字段将被忽略。

然而，在序列化时，`secret_info` 字段仍然会被使用。我们创建了一个 `Person` 结构体的实例，并将其序列化为 JSON 字符串。生成的 JSON 字符串中包含了 `secret_info` 字段，即使在反序列化时它被跳过了。

通过使用 `#[serde(skip_deserializing)]` 属性，您可以选择性地在反序列化过程中排除某些字段，但仍然保留它们在序列化过程中的使用。这对于在反序列化时忽略某些字段的值，但在序列化时仍然需要使用它们很有用。



### 4.2.7 `#[serde(serialize_with = "path")]`

- `#[serde(serialize_with = "path")]` 是一个属性，用于指定在序列化过程中使用自定义的序列化函数。通过使用 `#[serde(serialize_with = "path")]` 属性，您可以告诉 Serde 库在序列化时使用指定的函数来处理特定字段。

  在 `#[serde(serialize_with = "path")]` 中，`path` 是一个函数路径，指定了要用于序列化的自定义函数。该函数应接受一个值作为参数，并返回一个实现了 `serde::ser::Serialize` trait 的类型。

  以下是一个示例：

  ```rust
  use serde::Serialize;
  use serde::Serializer;
  
  #[derive(Debug, Serialize)]
  struct Person {
      name: String,
      age: u32,
      #[serde(serialize_with = "serialize_secret_info")]
      secret_info: String,
  }
  
  fn serialize_secret_info<S>(secret_info: &String, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      serializer.serialize_str("This is a secret")
  }
  
  fn main() {
      let person = Person {
          name: "John Doe".to_owned(),
          age: 30,
          secret_info: "Some secret".to_owned(),
      };
  
      let json_string = serde_json::to_string(&person).unwrap();
      println!("{}", json_string);
  }
  ```

  在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `name`、`age` 和 `secret_info` 字段。在 `secret_info` 字段上，我们使用 `#[serde(serialize_with = "serialize_secret_info")]` 属性来指示 Serde 库在序列化时使用 `serialize_secret_info` 函数来处理该字段。

  `serialize_secret_info` 函数接受一个 `secret_info` 字符串和一个 `Serializer` 参数，并使用 `serializer.serialize_str` 方法将固定的字符串 "This is a secret" 序列化为 JSON 字符串。

  在 `main` 函数中，我们创建了一个 `Person` 结构体的实例，并将其序列化为 JSON 字符串。由于我们使用了 `#[serde(serialize_with = "serialize_secret_info")]` 属性，所以在序列化过程中，`secret_info` 字段将被替换为固定的字符串 "This is a secret"。

  通过使用 `#[serde(serialize_with = "path")]` 属性，您可以自定义特定字段的序列化过程，以便根据自己的需求进行处理。这对于对字段进行特定的转换或处理非常有用。

  

  ### 4.2. 8 `#[serde(deserialize_with = "path")]`

  `#[serde(serialize_with = "path")]` 是一个属性，用于指定在序列化过程中使用自定义的序列化函数。通过使用 `#[serde(serialize_with = "path")]` 属性，您可以告诉 Serde 库在序列化时使用指定的函数来处理特定字段。

  在 `#[serde(serialize_with = "path")]` 中，`path` 是一个函数路径，指定了要用于序列化的自定义函数。该函数应接受一个值作为参数，并返回一个实现了 `serde::ser::Serialize` trait 的类型。

  以下是一个示例：

  ```rust
  use serde::Serialize;
  use serde::Serializer;
  
  #[derive(Debug, Serialize)]
  struct Person {
      name: String,
      age: u32,
      #[serde(serialize_with = "serialize_secret_info")]
      secret_info: String,
  }
  
  fn serialize_secret_info<S>(secret_info: &String, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      serializer.serialize_str("This is a secret")
  }
  
  fn main() {
      let person = Person {
          name: "John Doe".to_owned(),
          age: 30,
          secret_info: "Some secret".to_owned(),
      };
  
      let json_string = serde_json::to_string(&person).unwrap();
      println!("{}", json_string);
  }
  {"name":"John Doe","age":30,"secret_info":"This is a secret"}
  ```

  在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `name`、`age` 和 `secret_info` 字段。在 `secret_info` 字段上，我们使用 `#[serde(serialize_with = "serialize_secret_info")]` 属性来指示 Serde 库在序列化时使用 `serialize_secret_info` 函数来处理该字段。

  `serialize_secret_info` 函数接受一个 `secret_info` 字符串和一个 `Serializer` 参数，并使用 `serializer.serialize_str` 方法将固定的字符串 "This is a secret" 序列化为 JSON 字符串。

  在 `main` 函数中，我们创建了一个 `Person` 结构体的实例，并将其序列化为 JSON 字符串。由于我们使用了 `#[serde(serialize_with = "serialize_secret_info")]` 属性，所以在序列化过程中，`secret_info` 字段将被替换为固定的字符串 "This is a secret"。

  通过使用 `#[serde(serialize_with = "path")]` 属性，您可以自定义特定字段的序列化过程，以便根据自己的需求进行处理。这对于对字段进行特定的转换或处理非常有用。

  

  ### 4.2.9 `#[serde(with = "module")]`

  `#[serde(with = "module")]` 是一个属性，用于指定在序列化和反序列化过程中使用自定义的序列化和反序列化函数。通过使用 `#[serde(with = "module")]` 属性，您可以告诉 Serde 库在处理特定字段时使用指定的模块中的函数来进行序列化和反序列化操作。

  在 `#[serde(with = "module")]` 中，`module` 是一个模块路径，指定了包含自定义序列化和反序列化函数的模块。

  以下是一个示例：

  ```rust
  use serde::{Deserialize, Serialize};
  
  mod custom_serialization {
      pub fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer,
      {
          serializer.serialize_str(&value.to_string())
      }
  
      pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
      where
          D: serde::Deserializer<'de>,
      {
          let s: &str = serde::Deserialize::deserialize(deserializer)?;
          s.parse().map_err(serde::de::Error::custom)
      }
  }
  
  #[derive(Debug, Serialize, Deserialize)]
  struct Data {
      #[serde(with = "custom_serialization")]
      value: u32,
  }
  
  fn main() {
      let data = Data { value: 42 };
  
      let json_string = serde_json::to_string(&data).unwrap();
      println!("{}", json_string);
  
      let deserialized_data: Data = serde_json::from_str(&json_string).unwrap();
      println!("{:?}", deserialized_data);
  }
  ```

  在上面的示例中，我们定义了一个 `Data` 结构体，其中包含一个 `value` 字段。在 `value` 字段上，我们使用 `#[serde(with = "custom_serialization")]` 属性来指示 Serde 库在序列化和反序列化时使用 `custom_serialization` 模块中的函数。

  `custom_serialization` 模块中包含了 `serialize` 和 `deserialize` 函数，用于自定义 `value` 字段的序列化和反序列化操作。在 `serialize` 函数中，我们将 `value` 转换为字符串并序列化为 JSON 字符串。在 `deserialize` 函数中，我们将接收到的字符串解析为 `u32` 类型。

  在 `main` 函数中，我们创建了一个 `Data` 结构体的实例，并将其序列化为 JSON 字符串。由于我们使用了 `#[serde(with = "custom_serialization")]` 属性，所以在序列化过程中，`value` 字段将使用 `custom_serialization` 模块中的 `serialize` 函数进行处理。

  同样，我们还可以从 JSON 字符串中反序列化 `Data` 结构体。在反序列化过程中，`value` 字段将使用 `custom_serialization` 模块中的 `deserialize` 函数进行处理。

  通过使用 `#[serde(with = "module")]` 属性，您可以自定义特定字段的序列化和反序列化过程，以便根据自己的需求进行处理。这对于对字段进行特定的转换或处理非常有用。

  

  ### 4.2.10 `#[serde(bound = "T: MyTrait")]`

  如上

  `Serialize`和/或impls的 where 子句`Deserialize`。这将替换 Serde 为当前变体推断的任何特征范围。

  允许为序列化与反序列化指定独立的边界：

  - `#[serde(bound(serialize = "T: MySerTrait"))]`
  - `#[serde(bound(deserialize = "T: MyDeTrait"))]`
  - `#[serde(bound(serialize = "T: MySerTrait", deserialize = "T: MyDeTrait"))]`

  

  

  ### 4.2.11 `#[serde(borrow)]`和`#[serde(borrow = "'a + 'b + ...")]`

  `#[serde(borrow)]` 和 `#[serde(borrow = "'a + 'b + ...")]` 是 Serde 库中的属性，用于指定在序列化和反序列化过程中字段的借用行为。

  `#[serde(borrow)]` 属性用于指定字段在序列化和反序列化时应该以借用的方式进行处理。这意味着字段的类型在序列化和反序列化期间将被借用，而不是所有权转移。

  以下是一个示例：

  ```rust
  use serde::{Deserialize, Serialize};
  
  #[derive(Debug, Serialize, Deserialize)]
  struct Data {
      #[serde(borrow)]
      value: String,
  }
  
  fn main() {
      let data = Data {
          value: String::from("Hello, World!"),
      };
  
      let json_string = serde_json::to_string(&data).unwrap();
      println!("{}", json_string);
  
      let deserialized_data: Data = serde_json::from_str(&json_string).unwrap();
      println!("{:?}", deserialized_data);
  }
  ```

  在上面的示例中，我们定义了一个 `Data` 结构体，其中包含一个 `value` 字段。在 `value` 字段上，我们使用 `#[serde(borrow)]` 属性来指示 Serde 库在序列化和反序列化时以借用的方式处理该字段。

  在 `main` 函数中，我们创建了一个 `Data` 结构体的实例，并将其序列化为 JSON 字符串。由于我们使用了 `#[serde(borrow)]` 属性，所以在序列化过程中，`value` 字段将以借用的方式处理，而不是转移所有权。

  同样，我们可以从 JSON 字符串中反序列化 `Data` 结构体。在反序列化过程中，`value` 字段将以借用的方式处理。

  `#[serde(borrow = "'a + 'b + ...")]` 属性用于指定字段在序列化和反序列化时的借用约束。通过使用 `'a + 'b + ...` 语法，您可以指定字段的借用生命周期，并确保序列化和反序列化过程中的借用符合指定的生命周期约束。

  以下是一个示例：

  ```rust
  use serde::{Deserialize, Serialize};
  
  #[derive(Debug, Serialize, Deserialize)]
  struct Data<'a> {
      #[serde(borrow = "'a")]
      value: &'a str,
  }
  
  fn main() {
      let value = "Hello, World!";
      let data = Data { value };
  
      let json_string = serde_json::to_string(&data).unwrap();
      println!("{}", json_string);
  
      let deserialized_data: Data = serde_json::from_str(&json_string).unwrap();
      println!("{:?}", deserialized_data);
  }
  
  {"value":"Hello, World!"}
  Data { value: "Hello, World!" }
  ```

  在上面的示例中，我们定义了一个 `Data` 结构体，其中包含一个 `'a` 生命周期的引用字段 `value`。在 `value` 字段上，我们使用 `#[serde(borrow = "'a")]` 属性来指示 Serde 库在序列化和反序列化时应该将字段的借用生命周期限制为 `'a`。

  在 `main` 函数中，我们创建了一个 `Data` 结构体的实例，并将其序列化为 JSON 字符串。由于我们使用了 `#[serde(borrow = "'a")]` 属性，所以在序列化过程中，`value` 字段的借用生命周期限制为 `'a`。

  同样，我们可以从 JSON 字符串中反序列化 `Data` 结构体。在反序列化过程中，`value` 字段的借用生命周期也必须符合 `'a` 的约束。

  通过使用 `#[serde(borrow)]` 和 `#[serde(borrow = "'a + 'b + ...")]` 属性，您可以控制字段在序列化和反序列化过程中的借用行为，并指定借用的生命周期约束。这对于处理字段的所有权和借用非常有用，以满足特定的需求和场景。

  

  ### 4.2.12 `#[serde(other)]`

  `#[serde(other)]` 是 Serde 库中的一个属性，用于指定在反序列化时将未知字段存储为一个特定类型的结构体字段。

  当你使用 `#[serde(other)]` 属性时，Serde 将会将未知的字段序列化为指定类型的字段。这个字段将包含未知字段的名称和值。

  以下是一个示例：

  ```rust
  use serde::{Deserialize, Serialize};
  use serde_json::Value;
  
  #[derive(Debug, Serialize, Deserialize)]
  struct Data {
      #[serde(other)]
      unknown_fields: Vec<(String, Value)>,
  }
  
  fn main() {
      let json_string = r#"
          {
              "name": "John",
              "age": 30,
              "city": "New York"
          }
      "#;
  
      let deserialized_data: Data = serde_json::from_str(json_string).unwrap();
      println!("{:?}", deserialized_data);
  }
  ```

  在上面的示例中，我们定义了一个 `Data` 结构体，其中包含一个 `unknown_fields` 字段。在 `unknown_fields` 字段上，我们使用了 `#[serde(other)]` 属性来指示 Serde 库将未知字段存储为一个 `Vec<(String, Value)>` 类型的字段。

  在 `main` 函数中，我们使用 JSON 字符串进行反序列化，并将结果存储在 `Data` 结构体的实例中。由于我们使用了 `#[serde(other)]` 属性，所以在反序列化过程中，未知字段将被存储在 `unknown_fields` 字段中，以 `(String, Value)` 的形式表示，其中包含了未知字段的名称和值。

  通过使用 `#[serde(other)]` 属性，您可以处理那些在结构体中未定义的字段，以便在反序列化过程中保留它们的信息。这对于处理包含动态字段的数据非常有用，以便在后续的处理中进行进一步的分析或转换。

  

  ### 4.2.13 `#[serde(untagged)]`

  `#[serde(untagged)]` 是 Serde 库中的一个属性，用于指定在序列化和反序列化时如何处理无标记的变体类型（untagged variants）。

  当你使用 `#[serde(untagged)]` 属性时，Serde 将会以无标记的方式处理变体类型。这意味着在序列化时，变体类型的字段将被展开为外部结构体的字段，并且在反序列化时，可以接受多个字段来构造变体类型的实例。

  以下是一个示例：

  ```rust
  use serde::{Deserialize, Serialize};
  
  #[derive(Debug, Serialize, Deserialize)]
  #[serde(untagged)]
  enum Data {
      VariantA {
          field1: String,
          field2: i32,
      },
      VariantB {
          field3: bool,
      },
  }
  
  fn main() {
      let variant_a = Data::VariantA {
          field1: String::from("Hello"),
          field2: 42,
      };
  
      let variant_b = Data::VariantB {
          field3: true,
      };
  
      let serialized_a = serde_json::to_string(&variant_a).unwrap();
      let serialized_b = serde_json::to_string(&variant_b).unwrap();
  
      println!("Serialized variant A: {}", serialized_a);
      println!("Serialized variant B: {}", serialized_b);
  
      let deserialized_a: Data = serde_json::from_str(&serialized_a).unwrap();
      let deserialized_b: Data = serde_json::from_str(&serialized_b).unwrap();
  
      println!("Deserialized variant A: {:?}", deserialized_a);
      println!("Deserialized variant B: {:?}", deserialized_b);
  }
  
  Serialized variant A: {"field1":"Hello","field2":42}
  Serialized variant B: {"field3":true}
  Deserialized variant A: VariantA { field1: "Hello", field2: 42 }
  Deserialized variant B: VariantB { field3: true }
  
  ```

  在上面的示例中，我们定义了一个 `Data` 枚举类型，其中包含两个变体：`VariantA` 和 `VariantB`。在 `Data` 枚举上，我们使用了 `#[serde(untagged)]` 属性来指示 Serde 库以无标记的方式处理这两个变体。

  在 `main` 函数中，我们创建了一个 `VariantA` 和一个 `VariantB` 的实例，并将它们分别序列化为 JSON 字符串。由于我们使用了 `#[serde(untagged)]` 属性，所以在序列化过程中，变体类型的字段将被展开为外部结构体的字段。

  我们还从 JSON 字符串中反序列化了 `VariantA` 和 `VariantB`。由于我们使用了 `#[serde(untagged)]` 属性，所以在反序列化过程中，可以接受多个字段来构造变体类型的实例。

  通过使用 `#[serde(untagged)]` 属性，您可以灵活地序列化和反序列化无标记的变体类型，以适应不同的数据结构和格式。这对于处理具有不确定字段结构的数据非常有用。

## 4.3 字段属性

- ##### `#[serde(rename = "name")]`

  使用给定名称而不是 Rust 名称序列化和反序列化该字段。[这对于将字段序列化为驼峰命名法](https://serde.rs/attr-rename.html)或序列化名称为 Rust 保留关键字的字段非常有用。

  允许为序列化与反序列化指定独立的名称：

  - `#[serde(rename(serialize = "ser_name"))]`
  - `#[serde(rename(deserialize = "de_name"))]`
  - `#[serde(rename(serialize = "ser_name", deserialize = "de_name"))]`

- ##### `#[serde(alias = "name")]`

  从给定名称*或其*Rust 名称反序列化此字段。可以重复指定同一字段的多个可能的名称。

- ##### `#[serde(default)]`

  如果反序列化时该值不存在，请使用`Default::default()`.

- ##### `#[serde(default = "path")]`

  如果反序列化时该值不存在，则调用函数来获取默认值。给定的函数必须可调用为`fn() -> T`. 例如 `default = "empty_value"`，将调用`empty_value()`和`default = "SomeTrait::some_default"`将调用`SomeTrait::some_default()`.

- ##### `#[serde(flatten)]`

  将此字段的内容平铺到定义它的容器中。

  这删除了序列化表示和 Rust 数据结构表示之间的一层结构。它可用于将公共键分解为共享结构，或将剩余字段捕获到具有任意字符串键的映射中。结构[扁平化](https://serde.rs/attr-flatten.html)页面提供了一些示例。

  *注意：*不支持此属性与使用 的结构结合使用 [`deny_unknown_fields`](https://serde.rs/container-attrs.html#deny_unknown_fields)。外部扁平结构和内部扁平结构都不应该使用该属性。

- ##### `#[serde(skip)]`

  跳过此字段：不要序列化或反序列化它。

  反序列化时，Serde 将使用`Default::default()`或 给出的函数`default = "..."`来获取该字段的默认值。

- ##### `#[serde(skip_serializing)]`

  序列化时跳过此字段，但反序列化时不跳过。

- ##### `#[serde(skip_deserializing)]`

  反序列化时跳过该字段，但序列化时则不跳过。

  反序列化时，Serde 将使用`Default::default()`或 给出的函数`default = "..."`来获取该字段的默认值。

- ##### `#[serde(skip_serializing_if = "path")]`

  调用函数来确定是否跳过序列化该字段。给定的函数必须可调用为`fn(&T) -> bool`，尽管它可能是泛型的 `T`。例如，`skip_serializing_if = "Option::is_none"`将跳过 None 的选项。

- ##### `#[serde(serialize_with = "path")]`

  使用与其 的实现不同的函数序列化该字段`Serialize`。给定的函数必须是可调用的 `fn<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer`，尽管它也可以是泛型的`T`。与 一起使用的字段`serialize_with`不需要实现`Serialize`。

- ##### `#[serde(deserialize_with = "path")]`

  使用与其 的实现不同的函数反序列化该字段`Deserialize`。给定的函数必须是可调用的 `fn<'de, D>(D) -> Result<T, D::Error> where D: Deserializer<'de>`，尽管它也可以是泛型的`T`。与 一起使用的字段`deserialize_with`不需要实现`Deserialize`。

- ##### `#[serde(with = "module")]`

  `serialize_with`和的组合`deserialize_with`。Serde 将使用 `$module::serialize`as`serialize_with`函数和 `$module::deserialize`as`deserialize_with`函数。

- ##### `#[serde(borrow)]`和`#[serde(borrow = "'a + 'b + ...")]`

  使用零拷贝反序列化从反序列化器借用该字段的数据。请参阅[此示例](https://serde.rs/lifetimes.html#borrowing-data-in-a-derived-impl)。

- ##### `#[serde(bound = "T: MyTrait")]`

  `Serialize`the和impls的 where 子句`Deserialize`。这将替换 Serde 为当前字段推断的任何特征边界。

  允许为序列化与反序列化指定独立的边界：

  - `#[serde(bound(serialize = "T: MySerTrait"))]`
  - `#[serde(bound(deserialize = "T: MyDeTrait"))]`
  - `#[serde(bound(serialize = "T: MySerTrait", deserialize = "T: MyDeTrait"))]`

  ### 4.3.1 `#[serde(getter = "...")]`

  `#[serde(getter = "...")]` 是 Serde 库中的一个属性，用于指定在序列化时使用的 getter 方法的名称。

  当你使用 `#[serde(getter = "...")]` 属性时，Serde 将会调用指定名称的 getter 方法来获取字段的值进行序列化。

  以下是一个示例：

  ```rust
  use serde::{Serialize, Deserialize};
  
  #[derive(Debug, Serialize, Deserialize)]
  struct Person {
      #[serde(getter = "get_full_name")]
      first_name: String,
      last_name: String,
  }
  
  impl Person {
      fn new(first_name: String, last_name: String) -> Person {
          Person {
              first_name,
              last_name,
          }
      }
  
      fn get_full_name(&self) -> String {
          format!("{} {}", self.first_name, self.last_name)
      }
  }
  
  fn main() {
      let person = Person::new(String::from("John"), String::from("Doe"));
  
      let serialized = serde_json::to_string(&person).unwrap();
      println!("Serialized: {}", serialized);
  }
  ```

  在上面的示例中，我们定义了一个 `Person` 结构体，其中包含 `first_name` 和 `last_name` 字段。在 `first_name` 字段上，我们使用了 `#[serde(getter = "get_full_name")]` 属性来指定在序列化时使用的 getter 方法的名称。

  在 `Person` 结构体的实现中，我们定义了 `get_full_name` 方法，它返回一个包含完整姓名的字符串。在 `main` 函数中，我们创建了一个 `Person` 实例，并将其序列化为 JSON 字符串。

  由于我们使用了 `#[serde(getter = "get_full_name")]` 属性，所以在序列化过程中，Serde 将调用 `get_full_name` 方法来获取 `first_name` 字段的值，并将其作为序列化结果的一部分。

  通过使用 `#[serde(getter = "...")]` 属性，您可以控制在序列化过程中使用的 getter 方法的名称，以便在需要自定义字段值的序列化逻辑时进行灵活处理。



# 5 自定义序列化

Serde 的[派生宏](https://serde.rs/derive.html)通过`#[derive(Serialize, Deserialize)]` 为结构和枚举提供合理的默认序列化行为，并且可以使用[属性](https://serde.rs/attributes.html)在某种程度上进行自定义。对于不寻常的需求，Serde 允许通过手动实现类型的特征[`Serialize`](https://docs.rs/serde/1/serde/ser/trait.Serialize.html)来完全自定义序列化行为。[`Deserialize`](https://docs.rs/serde/1/serde/de/trait.Deserialize.html)

每个特征都有一个方法：

```rust
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

这些方法在序列化格式上是通用的，由 [`Serializer`](https://docs.rs/serde/1/serde/ser/trait.Serializer.html)和[`Deserializer`](https://docs.rs/serde/1/serde/de/trait.Deserializer.html)特征表示。例如，JSON 有一种序列化器类型，而明信片有另一种序列化器类型。

- [实施`Serialize`](https://serde.rs/impl-serialize.html)
- [实施`Deserialize`](https://serde.rs/impl-deserialize.html)



## 5.1 实现序列化

实现序列化

该[`Serialize`](https://docs.rs/serde/1/serde/ser/trait.Serialize.html)特征看起来像这样：

```rust
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
```

此方法的工作是获取您的类型 ( ) 并通过调用给定 上的方法之一`&self`将其映射到[Serde 数据模型](https://serde.rs/data-model.html)[`Serializer`](https://docs.rs/serde/1/serde/ser/trait.Serializer.html)。

在大多数情况下，Serde 的[派生](https://serde.rs/derive.html)能够`Serialize`为您的板条箱中定义的结构和枚举生成适当的实现。如果您需要以derive不支持的方式自定义类型的序列化行为，您可以`Serialize`自己实现。

## 5.1 序列化原语

作为最简单的示例，这里是`Serialize`原语的内置实现 `i32`。

```rust
impl Serialize for i32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(*self)
    }
}
```

Serde 为 Rust 的所有[基本类型](https://doc.rust-lang.org/book/primitive-types.html)提供了这样的实现，因此您不必负责自己实现它们，但是`serialize_i32`如果您有一个类型需要以其序列化形式表示为基本类型，则类似的方法可能会很有用。例如，您可以将[类似 C 的枚举序列化为原始数字](https://serde.rs/enum-number.html)。

## 5.2 序列化序列或映射

复合类型遵循初始化、元素、结束三步过程。

```rust
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

impl<K, V> Serialize for MyMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
```

## 5.3 序列化元组

方法`serialize_tuple`很像`serialize_seq`。Serde 的区别在于，`serialize_tuple`序列的长度不需要序列化，因为它在反序列化时是已知的。常见的例子是 Rust[元组](https://doc.rust-lang.org/std/primitive.tuple.html)和[数组](https://doc.rust-lang.org/std/primitive.array.html)。在非自描述格式中， `Vec<T>`需要对其长度进行序列化，以便能够反序列化`Vec<T>`返回。但是可以使用 a`[T; 16]`进行序列化 `serialize_tuple`，因为在反序列化时就可以知道长度，而无需查看序列化的字节。

## 5.4 序列化结构体

Serde 区分四种类型的结构。[普通结构体](https://doc.rust-lang.org/book/structs.html)和[元组结构体](https://doc.rust-lang.org/book/structs.html#tuple-structs)就像序列或映射一样遵循 init、elements、end 三步过程。[新类型结构](https://doc.rust-lang.org/book/structs.html#tuple-structs)和[单元结构](https://doc.rust-lang.org/book/structs.html#unit-like-structs)更像原语。

```rust
// An ordinary struct. Use three-step process:
//   1. serialize_struct
//   2. serialize_field
//   3. end
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// A tuple struct. Use three-step process:
//   1. serialize_tuple_struct
//   2. serialize_field
//   3. end
struct Point2D(f64, f64);

// A newtype struct. Use serialize_newtype_struct.
struct Inches(u64);

// A unit struct. Use serialize_unit_struct.
struct Instance;
```

结构和映射在某些格式（包括 JSON）中可能看起来相似。Serde 的区别在于，结构体的键是编译时常量字符串，并且在反序列化时无需查看序列化数据即可获知。这种情况使得某些数据格式能够比映射更高效、更紧凑地处理结构。

鼓励数据格式将新类型结构视为内部值的无关紧要的包装器，仅序列化内部值。例如，请参阅 [JSON 对 newtype structs 的处理](https://serde.rs/json.html)。

```rust
use serde::ser::{Serialize, Serializer, SerializeStruct};

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}
```

## 5.5 序列化枚举

序列化枚举变体与序列化结构非常相似。

```rust
enum E {
    // Use three-step process:
    //   1. serialize_struct_variant
    //   2. serialize_field
    //   3. end
    Color { r: u8, g: u8, b: u8 },

    // Use three-step process:
    //   1. serialize_tuple_variant
    //   2. serialize_field
    //   3. end
    Point2D(f64, f64),

    // Use serialize_newtype_variant.
    Inches(u64),

    // Use serialize_unit_variant.
    Instance,
}
```

## 5.6 其他特殊情况

还有两种特殊情况是序列化器特征的一部分。

有一种方法`serialize_bytes`可以序列化`&[u8]`. 某些格式像任何其他序列一样对待字节，但某些格式能够更紧凑地序列化字节。目前 Serde 不在impl for或`serialize_bytes`中 使用，但一旦[专业化](https://github.com/rust-lang/rust/issues/31844)进入稳定的 Rust，我们将开始使用它。目前，该板条箱可用于实现高效处理和通过 。`Serialize``&[u8]``Vec<u8>`[`serde_bytes`](https://docs.rs/serde_bytes)`&[u8]``Vec<u8>``serialize_bytes`

最后，`serialize_some`和`serialize_none`对应于`Option::Some`和 `Option::None`。`Option` 与其他枚举相比，用户往往对枚举有不同的期望。Serde JSON 将序列化为`Option::None`所`null` 包含`Option::Some`的值。



# 6 实施反序列化

该[`Deserialize`](https://docs.rs/serde/1/serde/de/trait.Deserialize.html)特征看起来像这样：

```rust
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

此方法的工作是通过提供可由 驱动的 来 构造类型的实例，从而将类型映射到[Serde 数据模型。](https://serde.rs/data-model.html)[`Deserializer`](https://docs.rs/serde/1/serde/trait.Deserializer.html)[`Visitor`](https://docs.rs/serde/1/serde/de/trait.Visitor.html)`Deserializer`

在大多数情况下，Serde 的[派生](https://serde.rs/derive.html)能够`Deserialize`为您的板条箱中定义的结构和枚举生成适当的实现。如果您需要以derive不支持的方式自定义类型的反序列化行为，您可以`Deserialize`自己实现。实现`Deserialize` 类型往往比实现更复杂`Serialize`。

该`Deserializer`特征支持两种入口点样式，可以实现不同类型的反序列化。

1. 方法`deserialize_any`。像 JSON 这样的自描述数据格式能够查看序列化数据并告诉它代表什么。例如，JSON 反序列化器可能会看到一个左花括号 ( `{`) 并知道它正在看到一个地图。如果数据格式支持`Deserializer::deserialize_any`，它将使用在输入中看到的任何类型来驱动访问者。JSON 在反序列化时使用这种方法，`serde_json::Value`它是一个可以表示任何 JSON 文档的枚举。在不知道 JSON 文档内容的情况下，我们可以`serde_json::Value`通过 `Deserializer::deserialize_any`.
2. 其他各种`deserialize_*`方法。像明信片这样的非自描述格式需要被告知输入内容才能反序列化。这些 `deserialize_*`方法是解串器如何解释下一个输入的提示。非自描述格式无法反序列化依赖`serde_json::Value`于 `Deserializer::deserialize_any`.

实现时`Deserialize`，您应该避免依赖 ， `Deserializer::deserialize_any`除非您需要解串器告诉您输入的类型。要知道，依赖`Deserializer::deserialize_any` 意味着您的数据类型将只能从自描述格式进行反序列化，排除明信片和许多其他格式。

## 6.1 访客特征

A[`Visitor`](https://docs.rs/serde/1/serde/de/trait.Visitor.html)由`Deserialize`impl 实例化并传递给 a `Deserializer`。然后`Deserializer`调用 的方法来`Visitor`构造所需的类型。

这是`Visitor`能够`i32`从各种类型反序列化原语的。

```rust
use std::fmt;

use serde::de::{self, Visitor};

struct I32Visitor;

impl<'de> Visitor<'de> for I32Visitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(i32::from(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::i32;
        if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
            Ok(value as i32)
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }

    // Similar for other methods:
    //   - visit_i16
    //   - visit_u8
    //   - visit_u16
    //   - visit_u32
    //   - visit_u64
}
```

该`Visitor`特征还有很多未针对 实现的方法 `I32Visitor`。不实现它们意味着如果调用它们，则会返回[类型错误。](https://docs.rs/serde/1/serde/de/trait.Error.html#method.invalid_type)例如，`I32Visitor`未实现 `Visitor::visit_map`，因此当输入包含映射时尝试反序列化 i32 是类型错误。

## 6.2 开车送访客

`Visitor`通过将 a 传递给给定的 来反序列化一个值`Deserializer`。将 根据输入数据调用`Deserializer`其中一种`Visitor`方法，这称为“驱动” `Visitor`.

```rust
impl<'de> Deserialize<'de> for i32 {
    fn deserialize<D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(I32Visitor)
    }
}
```

请注意， a`Deserializer`不一定遵循类型提示，因此调用 to`deserialize_i32`并不一定意味着`Deserializer`will 调用 `I32Visitor::visit_i32`。例如，JSON 对所有有符号整数类型都一视同仁。JSON`Deserializer`将调用`visit_i64`任何有符号整数和 `visit_u64`任何无符号整数，即使暗示不同的类型。

## 6.3 其他例子

- [反序列化地图](https://serde.rs/deserialize-map.html)
- [反序列化一个结构体](https://serde.rs/deserialize-struct.html)



# 7 源码实现

## 7.1 序列化实现

```
use serde::{ser, Serialize};

use error::{Error, Result};

pub struct Serializer {
    // This string starts empty and JSON is appended as values are serialized.
    output: String,
}

// By convention, the public API of a Serde serializer is one or more `to_abc`
// functions such as `to_string`, `to_bytes`, or `to_writer` depending on what
// Rust types the serializer is able to produce as output.
//
// This basic serializer supports only `to_string`.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    // JSON does not distinguish between different sizes of integers, so all
    // signed integers will be serialized the same and all unsigned integers
    // will be serialized the same. Other formats, especially compact binary
    // formats, may need independent logic for the different sizes.
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += "\"";
        self.output += v;
        self.output += "\"";
        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // JSON as `null`.
    fn serialize_unit(self) -> Result<()> {
        self.output += "null";
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":";
        value.serialize(&mut *self)?;
        self.output += "}";
        Ok(())
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output += "[";
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":[";
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output += "{";
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":{";
        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]}";
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}}";
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq: Vec<&'static str>,
    }

    let test = Test {
        int: 1,
        seq: vec!["a", "b"],
    };
    let expected = r#"{"int":1,"seq":["a","b"]}"#;
    assert_eq!(to_string(&test).unwrap(), expected);
}

#[test]
fn test_enum() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let u = E::Unit;
    let expected = r#""Unit""#;
    assert_eq!(to_string(&u).unwrap(), expected);

    let n = E::Newtype(1);
    let expected = r#"{"Newtype":1}"#;
    assert_eq!(to_string(&n).unwrap(), expected);

    let t = E::Tuple(1, 2);
    let expected = r#"{"Tuple":[1,2]}"#;
    assert_eq!(to_string(&t).unwrap(), expected);

    let s = E::Struct { a: 1 };
    let expected = r#"{"Struct":{"a":1}}"#;
    assert_eq!(to_string(&s).unwrap(), expected);
}
```

## 7.2 反序列化

```
use std::ops::{AddAssign, MulAssign, Neg};

use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use error::{Error, Result};

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> Deserializer<'de> {
    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    // Parse the JSON identifier `true` or `false`.
    fn parse_bool(&mut self) -> Result<bool> {
        if self.input.starts_with("true") {
            self.input = &self.input["true".len()..];
            Ok(true)
        } else if self.input.starts_with("false") {
            self.input = &self.input["false".len()..];
            Ok(false)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    // Parse a group of decimal digits as an unsigned integer of type T.
    //
    // This implementation is a bit too lenient, for example `001` is not
    // allowed in JSON. Also the various arithmetic operations can overflow and
    // panic or return bogus data. But it is good enough for example code!
    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    // Parse a possible minus sign followed by a group of decimal digits as a
    // signed integer of type T.
    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
    {
        // Optional minus sign, delegate to `parse_unsigned`, negate if negative.
        unimplemented!()
    }

    // Parse a string until the next '"' character.
    //
    // Makes no attempt to handle escape sequences. What did you expect? This is
    // example code!
    fn parse_string(&mut self) -> Result<&'de str> {
        if self.next_char()? != '"' {
            return Err(Error::ExpectedString);
        }
        match self.input.find('"') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => Err(Error::Eof),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_char()? {
            'n' => self.deserialize_unit(visitor),
            't' | 'f' => self.deserialize_bool(visitor),
            '"' => self.deserialize_str(visitor),
            '0'..='9' => self.deserialize_u64(visitor),
            '-' => self.deserialize_i64(visitor),
            '[' => self.deserialize_seq(visitor),
            '{' => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    // Uses the `parse_bool` parsing function defined above to read the JSON
    // identifier `true` or `false` from the input.
    //
    // Parsing refers to looking at the input and deciding that it contains the
    // JSON value `true` or `false`.
    //
    // Deserialization refers to mapping that JSON value into Serde's data
    // model by invoking one of the `Visitor` methods. In the case of JSON and
    // bool that mapping is straightforward so the distinction may seem silly,
    // but in other cases Deserializers sometimes perform non-obvious mappings.
    // For example the TOML format has a Datetime type and Serde's data model
    // does not. In the `toml` crate, a Datetime in the input is deserialized by
    // mapping it to a Serde data model "struct" type with a special name and a
    // single field containing the Datetime represented as a string.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // The `Serializer` implementation on the previous page serialized chars as
    // single-character strings so handle that representation here.
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse a string, check that it is one character, call `visit_char`.
        unimplemented!()
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with("null") {
            self.input = &self.input["null".len()..];
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with("null") {
            self.input = &self.input["null".len()..];
            visitor.visit_unit()
        } else {
            Err(Error::ExpectedNull)
        }
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening bracket of the sequence.
        if self.next_char()? == '[' {
            // Give the visitor access to each element of the sequence.
            let value = visitor.visit_seq(CommaSeparated::new(self))?;
            // Parse the closing bracket of the sequence.
            if self.next_char()? == ']' {
                Ok(value)
            } else {
                Err(Error::ExpectedArrayEnd)
            }
        } else {
            Err(Error::ExpectedArray)
        }
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening brace of the map.
        if self.next_char()? == '{' {
            // Give the visitor access to each entry of the map.
            let value = visitor.visit_map(CommaSeparated::new(self))?;
            // Parse the closing brace of the map.
            if self.next_char()? == '}' {
                Ok(value)
            } else {
                Err(Error::ExpectedMapEnd)
            }
        } else {
            Err(Error::ExpectedMap)
        }
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.peek_char()? == '"' {
            // Visit a unit variant.
            visitor.visit_enum(self.parse_string()?.into_deserializer())
        } else if self.next_char()? == '{' {
            // Visit a newtype variant, tuple variant, or struct variant.
            let value = visitor.visit_enum(Enum::new(self))?;
            // Parse the matching close brace.
            if self.next_char()? == '}' {
                Ok(value)
            } else {
                Err(Error::ExpectedMapEnd)
            }
        } else {
            Err(Error::ExpectedEnum)
        }
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        CommaSeparated {
            de,
            first: true,
        }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.peek_char()? == ']' {
            return Ok(None);
        }
        // Comma is required before every element except the first.
        if !self.first && self.de.next_char()? != ',' {
            return Err(Error::ExpectedArrayComma);
        }
        self.first = false;
        // Deserialize an array element.
        seed.deserialize(&mut *self.de).map(Some)
    }
}

// `MapAccess` is provided to the `Visitor` to give it the ability to iterate
// through entries of the map.
impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.de.peek_char()? == '}' {
            return Ok(None);
        }
        // Comma is required before every entry except the first.
        if !self.first && self.de.next_char()? != ',' {
            return Err(Error::ExpectedMapComma);
        }
        self.first = false;
        // Deserialize a map key.
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // It doesn't make a difference whether the colon is parsed at the end
        // of `next_key_seed` or at the beginning of `next_value_seed`. In this
        // case the code is a bit simpler having it here.
        if self.de.next_char()? != ':' {
            return Err(Error::ExpectedMapColon);
        }
        // Deserialize a map value.
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        // The `deserialize_enum` method parsed a `{` character so we are
        // currently inside of a map. The seed will be deserializing itself from
        // the key of the map.
        let val = seed.deserialize(&mut *self.de)?;
        // Parse the colon separating map key from value.
        if self.de.next_char()? == ':' {
            Ok((val, self))
        } else {
            Err(Error::ExpectedMapColon)
        }
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<()> {
        Err(Error::ExpectedString)
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
        int: u32,
        seq: Vec<String>,
    }

    let j = r#"{"int":1,"seq":["a","b"]}"#;
    let expected = Test {
        int: 1,
        seq: vec!["a".to_owned(), "b".to_owned()],
    };
    assert_eq!(expected, from_str(j).unwrap());
}

#[test]
fn test_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let j = r#""Unit""#;
    let expected = E::Unit;
    assert_eq!(expected, from_str(j).unwrap());

    let j = r#"{"Newtype":1}"#;
    let expected = E::Newtype(1);
    assert_eq!(expected, from_str(j).unwrap());

    let j = r#"{"Tuple":[1,2]}"#;
    let expected = E::Tuple(1, 2);
    assert_eq!(expected, from_str(j).unwrap());

    let j = r#"{"Struct":{"a":1}}"#;
    let expected = E::Struct { a: 1 };
    assert_eq!(expected, from_str(j).unwrap());
}
```



# 8 例子

**[JSON 中的结构和枚举](https://serde.rs/json.html)**：为结构和枚举选择的表示形式 [`serde_json`](https://github.com/serde-rs/json)。鼓励其他人类可读的数据格式尽可能遵循类似的方法。

**[枚举表示](https://serde.rs/enum-representations.html)**：以自描述格式表示枚举的外部标记、内部标记、相邻标记和未标记方式。

**[字段的默认值](https://serde.rs/attr-default.html)**： `#[serde(default)]`属性的一些示例。

**[手写泛型类型界限](https://serde.rs/attr-bound.html)**：一些不寻常的场景，其中 Serde 的派生推断出错误的泛型类型界限。可以使用该属性将 impl 边界替换为手写边界`#[serde(bound)]`。

**[反序列化自定义地图类型](https://serde.rs/deserialize-map.html)**：详细解释反序列化地图所涉及的每个步骤。

**[不带缓冲的值数组](https://serde.rs/stream-array.html)**：反序列化整数数组的最大值，而不将整个数组立即保存在内存中。这种方法可以适应处理各种其他情况，在这些情况下，数据需要在反序列化时而不是之后进行处理。

**[Serialize enum as number](https://serde.rs/enum-number.html)**：用于实现的宏`Serialize`，用于 `Deserialize`类似 C 的枚举，以跨所有数据格式将其表示为 a 的方式`u64`。

**[将字段序列化为驼峰命名法](https://serde.rs/attr-rename.html)**：该属性的一种常见应用`#[serde(rename)]`。

**[跳过序列化字段](https://serde.rs/attr-skip-serializing.html)**： `#[serde(skip_serializing)]`和`#[serde(skip_serializing_if)]`属性的一些示例。

**[派生远程板条箱](https://serde.rs/remote-derive.html)**：派生`Serialize`并 `Deserialize`实现其他人的板条箱中的类型。

**[手动反序列化 struct](https://serde.rs/deserialize-struct.html)**`Deserialize` ：由derive 为简单结构生成的 impl的长形式

**[丢弃数据](https://serde.rs/ignored-any.html)**：用于`IgnoredAny`有效地丢弃来自解串器的数据。

**[将一种格式转码为另一种格式](https://serde.rs/transcode.html)**：使用 [serde-transcode](https://github.com/sfackler/serde-transcode) crate 以流式传输一种格式的输入，以有效地以另一种格式输出。

**[反序列化字符串或结构](https://serde.rs/string-or-struct.html)**： [`docker-compose.yml`](https://docs.docker.com/compose/compose-file/#/build) 配置文件有一个“build”键，可以是字符串或结构。

**[转换错误类型](https://serde.rs/convert-error.html)**：将 Serde 错误从某种格式映射为其他格式的 Serde 错误，使用`Error::custom`.

**[自定义格式的日期](https://serde.rs/custom-date-format.html)**：处理 使用自定义字符串表示形式的格式化。[`chrono`](https://github.com/chronotope/chrono) `DateTime`



# 9 自定义日期

```
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
    // some custom logic to make it use our desired format.
    #[serde(with = "my_date_format")]
    pub timestamp: DateTime<Utc>,

    // Any other fields in the struct.
    pub bidder: String,
}

mod my_date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

fn main() {
    let json_str = r#"
      {
        "timestamp": "2017-02-16 21:54:30",
        "bidder": "Skrillex"
      }
    "#;

    let data: StructWithCustomDate = serde_json::from_str(json_str).unwrap();
    println!("{:#?}", data);

    let serialized = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", serialized);
}
```



```
StructWithCustomDate {
    timestamp: 2017-02-16T21:54:30Z,
    bidder: "Skrillex",
}
{
  "timestamp": "2017-02-16 21:54:30",
  "bidder": "Skrillex"
}
```



















































































































































































































































