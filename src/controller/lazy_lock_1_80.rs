// https://mp.weixin.qq.com/s/WzvB66m_zGyd6aP39qIoJg

use std::ops::Sub;
use std::sync::LazyLock;
use std::time::Instant;
static LAZY_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

//在 Rust 1.63.0 中引入的 std::thread::scope 是一个用于创建作用域线程的 API。
// 这种作用域线程允许你在一个有限的作用域内启动线程，
// 并确保这些线程在作用域结束之前完成。这种方式可以避免一些常见的并发问题，比如线程生命周期和数据竞态条件。

pub fn lazy_lock() {
    let start = Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("子线程 lazy time is {:?}", LAZY_TIME.sub(start));
        });

        println!("线程 lazy time is {:?}", LAZY_TIME.sub(start));
    });
}
