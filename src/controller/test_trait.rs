#![allow(unused)]
use std::fmt::Display;

pub trait Summary {
    fn summary(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub context: String,
}

impl Summary for Post {
    fn summary(&self) -> String {
        format!("文章{},作者{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub context: String,
}

impl Summary for Weibo {
    fn summary(&self) -> String {
        format!("{}发表了weobi{}", self.username, self.context)
    }
}

pub fn test_trait() {
    // let post = Post{
    //     title: "rust语言检测".to_string(),
    //     author: "Sunface".to_string(),
    //     context: "rust很棒".to_string(),
    // };
    // let weibo= Weibo{ username: "zhangsan".to_string(), context: "没有微博账号啊".to_string() };
    // println!("{}",post.summary());
    // println!("{}",weibo.summary());

    test_traits()
}

// ************************************ 特征约束

pub fn notify(item: impl Summary, item2: impl Summary) {}

pub fn notify1<T: Summary>(item: T) {}

// 多重约束
pub fn notify2(item: &(impl Summary + Display)) {}

pub fn notify3<T: Summary + Display>(item: T) {}
pub fn notify4<T: Summary + Display>(item: &T) {}

// ************************************ where约束
fn notify5<T, U>(item: T, item1: &U)
where
    T: Summary,
    U: Display,
{
}
// ************************************ 函数返回
pub fn notify6() -> impl Summary {
    Weibo {
        username: "".to_string(),
        context: "".to_string(),
    }
}

// ************************************ trait 接受指的两种方式
trait Draw {
    fn draw(&self) -> String;
}
impl Draw for u8 {
    fn draw(&self) -> String {
        format!("us : {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64 : {}", *self)
    }
}

//若T实现了Draw特征 则调用该函数时传入的Box<T> 可以被隐式转换成函数参数签名中的Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    let res = x.draw();
    println!("{}", res);
}

fn draw2(s: &dyn Draw) {
    let res = s.draw();
    println!("{}", res);
}

fn test_traits() {
    let s = 1.1f64;
    let y = 8u8;

    draw1(Box::new(s));
    draw2(&y)
    //f64 : 1.1
    // us : 8
}

//

// pub struct Screen{
//     pub components : Vec<Box<dyn Draw>>
// }
//
// impl Screen {
//     pub fn run(&self){
//         for  component in  self.components{
//             component.draw();
//         }
//     }
// }

// 泛型实现
struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

// ************************************ Self 与 self
trait Draw1 {
    fn draw(&self) -> Self;
}

#[derive(Clone)]
struct Button;

impl Draw1 for Button {
    fn draw(&self) -> Self {
        return (*self).clone();
    }
}

fn test_self() {
    let button = Button;
    let _ = button.draw();
}

// ************************************ 关联类型
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
