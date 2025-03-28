#![allow(dead_code)]
use std::pin::{ Pin};
use std::task::{Context, Poll};
use futures::{ StreamExt};
use futures_core::Stream;
use tokio::time::Sleep;
use std::marker::PhantomPinned;
use std::time::Duration;
use futures_core::Future;  // 关键：导入 Future trait

struct IntervalCounter {
    count: u32,
    max: u32,
    delay: Sleep,
    _pin: PhantomPinned,
}

impl IntervalCounter {
    fn new(max: u32) -> Self {
        let delay = tokio::time::sleep(Duration::from_secs(1));
        Self {
            count: 0,
            max,
            delay,
            _pin: PhantomPinned,
        }
    }
}

impl Stream for IntervalCounter {
    type Item = u32;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>> {
        if self.count >= self.max {
            return Poll::Ready(None);
        }

        // 安全地访问 delay 字段
        let delay = unsafe { self.as_mut().map_unchecked_mut(|s| &mut s.delay) };

        match Future::poll(delay, cx) {  // 显式调用 Future::poll
            Poll::Ready(_) => {
                // 安全地修改 count 和 delay
                unsafe {
                    let this = self.as_mut().get_unchecked_mut();
                    this.count += 1;
                    this.delay = tokio::time::sleep(Duration::from_secs(1));
                }
                Poll::Ready(Some(self.count))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub async fn test_stream_trait() {
    // 使用while let循环处理流
    let mut stream = Box::pin(IntervalCounter::new(5));
    while let Some(value) = stream.next().await {
        println!("value = {}", value);
    }

    // 使用适配器方法
    Box::pin(IntervalCounter::new(5))
        .map(|x| x * 2)
        .filter(|x| std::future::ready(x % 4 == 0))  // 使用 ready 包装 bool
        .for_each(|x| async move {
            println!("Processed value: {}", x);
        })
        .await;

    // value = 1
    // value = 2
    // value = 3
    // value = 4
    // value = 5
    // Processed value: 4
    // Processed value: 8

    // 使用 pin! 宏固定流
    // let mut stream_a = pin!(IntervalCounter::new(3));
    // let mut stream_b = futures::stream::iter(vec!['a', 'b', 'c']);
    //
    // loop {
    //     let next_a = stream_a.as_mut().next();
    //     let next_b = stream_b.next();
    //
    //     tokio::select! {
    //         Some(num) = next_a => {
    //             println!("Number: {}", num);
    //         }
    //         Some(ch) = next_b => {
    //             println!("Char: {}", ch);
    //         }
    //         else => break,
    //     }
    // }

}