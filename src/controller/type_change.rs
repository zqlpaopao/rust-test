#![allow(unused)]

use std::num::TryFromIntError;

/// https://mp.weixin.qq.com/s/CyNClBlsYBF2eyQgDglegw
///
/// https://mp.weixin.qq.com/s/VW0umde3VVJ2HBatc45mGQ
pub fn test_type_change() {
    //转换为f64
    // to_f64()

    //自定义类型
    // test_zdy()

    //错误链用法
    // process_data()

    //内存地址转换
    // mem_address_change_point()

    //try 适合从大类型转换为小类型 是否有精度丢失的
    try_into()
}

/***************************** try_into 返回错误***********************************/

fn try_into() {
    let value: u32 = 10;
    let result: Result<i32, TryFromIntError> = value.try_into();
    match result {
        Ok(num) => println!("转换成功: {}", num),
        Err(e) => println!("转换失败: {}", e),
    }

    let value: u32 = 99999999;
    let result: Result<i8, TryFromIntError> = value.try_into();
    match result {
        Ok(num) => println!("转换成功: {}", num),
        Err(e) => println!("转换失败: {}", e),
    }
    // 转换成功: 10
    // 转换失败: out of range integral type conversion attempted
}

/***************************** 内存地址转化为指针***********************************/
fn mem_address_change_point() {
    let mut values: [i32; 2] = [1, 3];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    println!("{:?}", values);
    //[1, 4]
}

/***************************** 错误处理链***********************************/
#[derive(Debug)]
enum DatabaseError {
    ConnectionFailed(String),
    QueryFailed(String),
}

#[derive(Debug)]
enum ApplicationError {
    DataBaseError(DatabaseError),
    Validation(String),
}

impl From<DatabaseError> for ApplicationError {
    fn from(err: DatabaseError) -> Self {
        ApplicationError::DataBaseError(err)
    }
}

fn query_database() -> Result<(), DatabaseError> {
    Err(DatabaseError::ConnectionFailed(String::from(
        "Connection timeout",
    )))
}

fn process_data() {
    let err = query_database();
    println!("{:?}", err)
}

/***************************** 自定义类型转换***********************************/
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl From<&str> for Person {
    fn from(name: &str) -> Person {
        Person {
            name: "name".to_string(),
            age: 0,
        }
    }
}

impl From<(String, u32)> for Person {
    fn from(tuple: (String, u32)) -> Person {
        Person {
            name: tuple.0,
            age: tuple.1,
        }
    }
}

fn test_zdy() {
    let person1 = Person::from("John");
    let person2: Person = "Alice".into();
    let person3: Person = Person::from(("Bob".to_string(), 25));

    println!(
        "person1: {:?}\n person2:{:?}\n person3:{:?}",
        person1, person2, person3
    );
    //person1: Person { name: "name", age: 0 }
    //  person2:Person { name: "name", age: 0 }
    //  person3:Person { name: "Bob", age: 25 }
}

/***************************** 基础类型***********************************/

// 转换为f64
fn to_f64() {
    let num = 42i32;

    // 使用From
    let float_from = f64::from(num);

    //into
    let float_into: f64 = num.into();

    let num_str = "43";

    //string to f64
    let f64_str = num_str.parse::<f64>().unwrap();

    println!(
        "from: {}, into: {},f64_str:{}",
        float_from, float_into, f64_str
    );
    // from: 42, into: 42,f64_str:43
}
