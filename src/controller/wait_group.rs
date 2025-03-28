#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;
use wg::WaitGroup;

/// https://mp.weixin.qq.com/s/7-J4usEmiC-4AyQDgEYI5g

pub fn test_wait_group() {
    // thread_test()
    sync_wait_group_test()
}

/// 同步中使用
fn thread_test() {
    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
        let ctrs = ctr.clone();
        let t_wg = wg.add(1);
        spawn(move || {
            sleep(Duration::from_millis(50));
            ctrs.fetch_add(1, Ordering::Relaxed);

            //
            t_wg.done();
        });
    }

    wg.wait();
    println!("{}", ctr.load(Ordering::Relaxed));
}

/// 异步中使用
fn sync_wait_group_test() {
    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
        let ctrx = ctr.clone();
        let w_g = wg.add(1);
        let _ = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            ctrx.fetch_add(1, Ordering::Relaxed);
            w_g.done()
        });
    }

    wg.wait();
    println!("sync {}", ctr.load(Ordering::Relaxed));
}
