//https://mp.weixin.qq.com/s/IVqM89BParM5RJhuMUhzgQ

use tokio::signal;

pub async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    println!("收到停机信号！");
}
