// 在 futures crate 中，IntoFuture 是一个 trait，用于将某个类型转换为一个 Future。
// 实现了这个 trait 的类型可以使用 into_future 方法来进行转换。
// 典型的用法是将某种类型（例如一个具体的结果类型）转换为一个 Future，以便在异步上下文中使用。

use std::future::{ready, IntoFuture, Ready};

struct MyType {
    num: u16,
    factor: u16,
}
impl MyType {
    /// 创建一个新的 `Multiply` 实例。
    pub fn new(num: u16, factor: u16) -> Self {
        Self { num, factor }
    }

    /// 设置要乘以因子的数字。
    pub fn number(mut self, num: u16) -> Self {
        self.num = num;
        self
    }

    /// 设置要与数字相乘的因子。
    pub fn factor(mut self, factor: u16) -> Self {
        self.factor = factor;
        self
    }
}

impl IntoFuture for MyType {
    type Output = u16;
    type IntoFuture = Ready<Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        ready(self.num * self.factor)
    }
}

pub async fn run() {
    let num = MyType::new(0, 0) // 将构建器初始化为数字: 0，因子: 0
        .number(2) // 将数字更改为 2
        .factor(2) // 将因子更改为 2
        .await; // 转换为 future 和 .await

    println!("{}", num);
    //4
}
