#![allow(unused)]
/// https://mp.weixin.qq.com/s/pMS3ac9sEF6w7_F8t9c-2g

// 📈 性能优化建议
// 1. 刷新率控制
// 使用pb.set_draw_delta(n)控制更新频率
// 避免过于频繁的刷新影响性能
// 2. 内存使用优化
// 合理设置缓冲区大小
// 及时清理不需要的进度条
// 3. 多线程场景
// 使用MultiProgress处理并发任务
// 注意线程安全性

pub fn test_progress() {
    //基础进度条
    // basic_progress_bar()

    //自定义样式
    // styled_progress_bar()

    //多样化展示
    // multi_progress_bars()

    //迭代器保障
    // iterator_wrapper()

    //高级特性 自定义状态展示
    // custom_state_display()

    //高级特性 嵌套进度显示
    // nested_progress()

    //实用技巧 下载进度
    // download_progress()

    //实用技巧 任务处理进度
    process_tasks()
}

/************************* 基础进度条 ***************************/
use indicatif::{ProgressBar, ProgressStyle};

fn basic_progress_bar() {
    let pb = ProgressBar::new(100);
    for _ in 0..100 {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    pb.finish_with_message("done");
    //██████████████████████████████████████████████░░░░░░░░░░░░░░░░░░ 86/100
}

/************************* 自定义样式 ***************************/

fn styled_progress_bar() {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    for i in 0..100 {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    // [00:00:00] [######>---------------------------------] 15/100 (4s)
}

/************************* 多样化显示 ***************************/
use indicatif::MultiProgress;

fn multi_progress_bars() {
    let m = MultiProgress::new();
    let pb1 = m.add(ProgressBar::new(100));
    let pb2 = m.add(ProgressBar::new(100));
    let pb3 = m.add(ProgressBar::new(100));
    std::thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                pb1.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });

        s.spawn(|| {
            for i in 0..100 {
                pb2.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        s.spawn(|| {
            for i in 0..100 {
                pb3.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(75));
            }
        });
    });

    //█████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 48/100
    //██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 25/100
    //██████████████████████████████████████░░░░░ 33/100^C
}

/************************* 迭代器包装 ***************************/
fn iterator_wrapper() {
    let items = vec!["1", "2", "3", "4", "5"];
    let pb = ProgressBar::new(items.len() as u64);

    for item in pb.wrap_iter(items.iter()) {
        std::thread::sleep(std::time::Duration::from_millis(500));
        // 处理item...
    }
    //███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2/5^C
}

/************************* 高级特性 专业级定制 ***************************/
// 1. 自定义状态展示
fn custom_state_display() {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"]),
    );
    for _ in 0..100 {
        pb.set_message(format!(
            "Processing... {}",
            chrono::Local::now().format("%H:%M:%S")
        ));
        std::thread::sleep(std::time::Duration::from_millis(100));
        pb.tick();
    }
    //▹▹▸▹▹ Processing... 15:51:09
}
// 2. 嵌套进度显示
fn nested_progress() {
    let main_pb = ProgressBar::new(3);
    main_pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40}] {pos}/{len}")
            .unwrap(),
    );
    for i in 0..3 {
        let sub_pb = ProgressBar::new(100);
        sub_pb.set_style(
            ProgressStyle::default_bar()
                .template("  ╰─▶ {spinner:.green} [{bar:40.cyan/blue}] {pos}/{len}")
                .unwrap(),
        );
        for j in 0..100 {
            sub_pb.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        sub_pb.finish_and_clear();
        main_pb.inc(1);
    }

    //[█████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░] 1/3
    //  [██████████████████████████░░░░░░░░░░░░░░] 2/3
    //    ╰─▶ ⠒ [███████████████████░░░░░░░░░░░░░░░░░░░░░] 49/100                                                                             ^C
}

/************************* 实用技巧 专业级定制 ***************************/

// 1. 下载进度显示
fn download_progress() {
    let pb = ProgressBar::new(1024);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("=>-"));
    // 模拟下载
    let chunk_size = 32;
    for _ in 0..1024 / chunk_size {
        pb.inc(chunk_size);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    //⠒ [00:00:01] [============================>-----------] 736 B/1.00 KiB (0s)
}
// 2. 任务处理进度
use std::collections::HashMap;

fn process_tasks() {
    let tasks: HashMap<&str, u64> = [("数据下载", 100), ("数据处理", 200), ("生成报告", 50)]
        .into_iter()
        .collect();
    let pb = ProgressBar::new(tasks.values().sum());
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap(),
    );
    for (task, size) in tasks {
        pb.set_message(format!("正在{}...", task));
        for _ in 0..size {
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    // 正在数据下载...
    // ⠖ [████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 77/350 (14s)
}
