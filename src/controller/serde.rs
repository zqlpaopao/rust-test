#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn test() {}
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Person8 {
//     #[serde(getter = "get_full_name")]
//     first_name: String,
//     last_name: String,
// }
//
// impl Person8 {
//     fn new(first_name: String, last_name: String) -> Person8 {
//         Person8 {
//             first_name,
//             last_name,
//         }
//     }
//
//     fn get_full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }
//
// fn getter() {
//     let person = Person8::new(String::from("John"), String::from("Doe"));
//
//     let serialized = serde_json::to_string(&person).unwrap();
//     println!("Serialized: {}", serialized);
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Data6 {
    VariantA { field1: String, field2: i32 },
    VariantB { field3: bool },
}

fn untagged1() {
    let variant_a = Data6::VariantA {
        field1: String::from("Hello"),
        field2: 42,
    };

    let variant_b = Data6::VariantB { field3: true };

    let serialized_a = serde_json::to_string(&variant_a).unwrap();
    let serialized_b = serde_json::to_string(&variant_b).unwrap();

    println!("Serialized variant A: {}", serialized_a);
    println!("Serialized variant B: {}", serialized_b);

    let deserialized_a: Data6 = serde_json::from_str(&serialized_a).unwrap();
    let deserialized_b: Data6 = serde_json::from_str(&serialized_b).unwrap();

    println!("Deserialized variant A: {:?}", deserialized_a);
    println!("Deserialized variant B: {:?}", deserialized_b);
}

// use serde_json::Value;
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Data1<'a> {
//     #[serde(other="'a")]
//     unknown_fields: Vec<&'a (String, Value)>,
// }
//
// fn other() {
//     let json_string = r#"
//         {
//             "name": "John",
//             "age": 30,
//             "city": "New York"
//         }
//     "#;
//
//     let deserialized_data: Data1 = serde_json::from_str(json_string).unwrap();
//     println!("{:?}", deserialized_data);
// }
//

#[derive(Debug, Serialize, Deserialize)]
struct Data<'a> {
    #[serde(borrow = "'a")]
    value: &'a str,
}

fn borrow() {
    let value = "Hello, World!";
    let data = Data { value };

    let json_string = serde_json::to_string(&data).unwrap();
    println!("{}", json_string);

    let deserialized_data: Data = serde_json::from_str(&json_string).unwrap();
    println!("{:?}", deserialized_data);
}

use serde::Serializer;
#[derive(Debug, Serialize)]
struct Person7 {
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

fn serialize_with() {
    let person = Person7 {
        name: "John Doe".to_owned(),
        age: 30,
        secret_info: "Some secret".to_owned(),
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("{}", json_string);
}

#[derive(Debug, Serialize, Deserialize)]
struct Person6 {
    name: String,
    age: u32,
    #[serde(skip_deserializing)]
    secret_info: String,
}

fn skip_deserializing() {
    let json = r#"{
        "name": "John Doe",
        "age": 30,
        "secret_info": "This is a secret"
    }"#;

    let deserialized_person: Person6 = serde_json::from_str(json).unwrap();
    println!("{:?}", deserialized_person);

    let person = Person6 {
        name: "Jane Smith".to_owned(),
        age: 25,
        secret_info: "This is a secret".to_owned(),
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("{}", json_string);
}

#[derive(Debug, Serialize, Deserialize)]
struct Person5 {
    name: String,
    age: u32,
    #[serde(skip_serializing)]
    secret_info: String,
}

fn skip_serializing() {
    let person = Person5 {
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

    let deserialized_person: Person5 = serde_json::from_str(json).unwrap();
    println!("{:?}", deserialized_person);
}

#[derive(Debug, Serialize, Deserialize)]
struct Person4 {
    name: String,
    age: u32,
    #[serde(skip)]
    secret_info: String,
}

fn skip() {
    let person = Person4 {
        name: "John Doe".to_owned(),
        age: 30,
        secret_info: "This is a secret".to_owned(),
    };

    let json_string = serde_json::to_string(&person).unwrap();
    println!("{}", json_string);
}

//***************************************** alias 别名   ******************************************* //

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

//
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct Person1 {
//     #[serde(expecting = "String")]
//     name: String,
//     #[serde(expecting = "u32")]
//     age: u32,
// }
//
// fn expecting() {
//     let json_string = r#"
//         {
//             "name": 42,
//             "age": "John"
//         }
//     "#;
//
//     let result: Result<Person1, serde_json::Error> = serde_json::from_str(json_string);
//     match result {
//         Ok(person) => println!("{:?}", person),
//         Err(error) => println!("{}", error),
//     }
// }

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

//***************************************** 无嵌套 直接   ******************************************* //
//直接序列化及反序列化
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Wrapper1 {
//     #[serde(transparent)]
//     value: i32,
// }
//
// fn transparents() {
//     // 反序列化
//     let deserialized: Wrapper1 = serde_json::from_str("42").unwrap();
//     println!("Deserialized: {:?}", deserialized);
//
//     // 序列化
//     let serialized = serde_json::to_string(&Wrapper1 { value: 42 }).unwrap();
//     println!("Serialized: {}", serialized);
// }
//

//***************************************** untagged   ******************************************* //
//

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(remote = "MyStruct3")]
// struct MyStructRemote {
//     value: i32,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(remote = "MyStructRemote")]
// struct MyStruct3 {
//     #[serde(default)]
//     value: i32,
// }
//
// fn remote() {
//     // 反序列化
//     let deserialized: MyStruct3 = serde_json::from_str(r#"{"value": 10}"#).unwrap();
//     println!("Deserialized: {:?}", deserialized);
//
//     // 序列化
//     let serialized = serde_json::to_string(&MyStruct3 { value: 42 }).unwrap();
//     println!("Serialized: {}", serialized);
// }

//***************************************** untagged   ******************************************* //
//缺失的时候调用默认函数进行值的补充
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

//***************************************** default   ******************************************* //
//确实字段自动添加默认值

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct1 {
    #[serde(default)]
    value: String,
    #[serde(default)]
    age: i8,
}

fn default() {
    let deserialized: MyStruct1 = serde_json::from_str(&r#"{}"#).unwrap();
    println!("deserialized {:?}", deserialized);

    let serialized = serde_json::to_string(&deserialized).unwrap();
    println!("serialized {}", serialized);
}

//***************************************** #[serde(bound = "T: MyTrait")]   ******************************************* //
//为泛型类型参数指定序列化和反序列化时的约束条件

trait MyTrait {
    fn do_something(&self);
}

#[derive(Serialize, Deserialize, Debug)]
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

//***************************************** untagged   ******************************************* //
//用于在序列化和反序列化过程中处理不带标签的变体类型
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

//***************************************** tag content  封装一层  ******************************************* //
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

//***************************************** type   ******************************************* //
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

fn r#type() {
    let circle = Shape::Circle {
        radius: 5.0,
        color: "red".to_string(),
    };

    let serialized = serde_json::to_string(&circle).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Shape = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

//***************************************** deny_unknown_fields   ******************************************* //
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct User {
    user_name: String,
    user_age: i32,
}
fn deny_unknown_fields() {
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

//***************************************** rename_all   ******************************************* //
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(rename(serialize = "ser_name", deserialize = "ser_name"))]
    z: Z,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "lowercase"))]
struct Z {
    user_name: String,
    user_age: i8,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "UPPERCASE"))]
struct Z1 {
    user_name: String,
    user_age: i8,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "lowercase"))]
struct Z2 {
    user_name: String,
    user_age: i8,
}

//容器的rename
fn container_rename() {
    let p = Z1 {
        user_name: "ZHangQL".to_string(),
        user_age: 18,
    };

    let ser = serde_json::to_string(&p).unwrap();
    println!("ser {}", ser);

    let des: Z2 = serde_json::from_str(&ser).unwrap();
    println!("des {:?}", des);
}

fn sample() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
