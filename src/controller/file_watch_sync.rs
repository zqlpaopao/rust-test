#![allow(unused)]
use futures::{
    channel::mpsc::{channel as fc, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, RecommendedWatcher, Watcher};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;
use tokio::sync::mpsc;
use tokio::time::sleep;

/// 温馨提示：要是想让文件监控更稳定，记得处理一下重命名事件，不然重命名后的文件可能监控不到。
fn watch_directory(path: &Path) -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            tx.send(event).unwrap();
        }
    })?;

    // Recursive 同时监视所有子目录，包括安装监视后创建的目录
    // NonRecursive 仅查看提供的目录
    watcher.watch(path, notify::RecursiveMode::NonRecursive)?;

    for event in rx {
        println!("{:?}", event);
    }
    Ok(())
}

/// 文件复制
async fn sync_file(source: PathBuf, target: PathBuf) -> anyhow::Result<()> {
    if source.is_file() {
        fs::copy(source, target).await?;
    } else if source.is_dir() {
        fs::create_dir_all(target).await?;
    }
    Ok(())
}

// 这段代码看着简单，其实坑不少。比如源文件正在被写入的时候去复制，搞不好就复制了个半成品。咱们得加个延迟：
async fn sync_file_delay(source: PathBuf, target: PathBuf) -> anyhow::Result<()> {
    sleep(Duration::from_millis(100)).await;
    sync_file(source, target).await
}

/// 要是文件变化太频繁，一个接一个地同步容易把机器累趴下。整个队列，一个一个来：
struct SyncTask {
    source: PathBuf,
    target: PathBuf,
}

/// 温馨提示：队列要是太长，内存可能会爆。给队列加个长度限制，比如 1000 个任务。
async fn process_queue(mut rx: mpsc::Receiver<SyncTask>) {
    while let Some(task) = rx.recv().await {
        if let Err(e) = sync_file_delay(task.source, task.target).await {
            eprintln!("{:?}", e);
        }
    }
}

pub async fn test_file_watch_sync() {
    let (mut tx, mut rx) = fc(10);
    let source_dir = Path::new("./log.log");
    // let target_dir = Path::new("./target");

    // tokio::spawn(process_queue(rx));

    //监控文件夹
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )
    .unwrap();

    watcher
        .watch(source_dir, notify::RecursiveMode::NonRecursive)
        .unwrap();

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                println!("changed: {:?}", event.info());
                println!("changed: {:?}", json!(event));
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    //
    // loop{
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }
}
