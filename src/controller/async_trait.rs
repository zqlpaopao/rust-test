#![allow(unused)]
use std::boxed::Box;
use std::future::Future;
use std::pin::Pin;

/// https://mp.weixin.qq.com/s/PdDaduBrSmWk2mXUP1Mm2A
///
///  和 future_into_future.rs 是一样的
trait MyTrait {
    type Fut: Future<Output = ()> + Send;
    fn do_something(&self) -> Self::Fut;
}

struct MyStruct;

impl MyTrait for MyStruct {
    type Fut = Pin<Box<dyn Future<Output = ()> + Send>>;
    fn do_something(&self) -> Self::Fut {
        Box::pin(async move { () })
    }
}
