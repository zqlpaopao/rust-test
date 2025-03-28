#![allow(unused)]
use std::fmt::Debug;

///
pub fn test_as_ref_into() {
    test_into()
}

// **************************************** into  ************************************//
///Into<String> 是另一个 trait，它定义了一个方法 into(self) -> String。
/// 这个 trait 通常用于表示某种类型可以被转换为 String 类型
/// AsRef<str> 提供了一种查看给定类型的能力，但不会修改原始值或创建新的副本。
/// Into<String> 则涉及到所有权转移和数据复制，因此它会创建一个新的 String 实例，并可能消耗更多资源。
/// 当你只需要读取字符串内容而不打算修改它或拥有其所有权时，使用 AsRef<str> 更合适。
/// 当你需要对字符串进行修改或者获取一个拥有所有权的新实例时，使用 Into<String> 更合适。
fn test_into() {
    let num = "aa";
    //help: consider giving `num_str` an explicit type
    //    |
    // 19 |     let num_str: /* Type */ = num.into();
    //    |                ++++++++++++
    // ^^^^ the trait `std::convert::From<i64>` is not implemented for `&str`
    // let num_str : &str = num.into();

    let num_str: String = num.into();
    println!("{}", num_str)
}

// **************************************** as_ref() ************************************//
fn test_as_ref() {
    let str = String::from("test ARef");

    let str1: &str = str.as_ref();
    println!("{}", str1)
}

// **************************************** AsRef trait ************************************//
/// as ref trait
/// 它接受一个类型为 T 的参数，并要求 T 实现 AsRef<str> trait
fn is_hello<T: AsRef<str> + Debug>(s: T) {
    // assert_eq!("hello", s.as_ref());
    println!("{:#?}", s);
}

fn test_params_as_ref() {
    // let s = "hello";
    // is_hello(s);
    //
    // let s = "hello".to_string();
    // is_hello(s);

    //自定义类型
    let my_struct = MyStruct {
        value: "hello".to_string(),
    };

    is_hello(my_struct);
}

#[derive(Debug)]
struct MyStruct {
    value: String,
}

impl AsRef<str> for MyStruct {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
