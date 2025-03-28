



# CancellationToken 任务取消

`tokio_util::sync::CancellationToken` 是一个用于异步任务取消的工具，允许你在需要时取消一个或多个任务。它可以用于在复杂的异步系统中实现优雅的任务终止。

### 主要功能

- **任务取消**：可以触发取消操作，影响所有与该令牌相关的任务。
- **嵌套取消**：支持层级结构，子令牌可以独立于父令牌取消。

### 使用场景

- **优雅关停**：在需要安全停止异步任务的情况下使用，比如应用程序关闭时。
- **任务协调**：在多个任务之间共享取消信号。

### 示例

```rust
use tokio::task;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();

    let cloned_token = token.clone();
    let task = task::spawn(async move {
        loop {
            tokio::select! {
                _ = cloned_token.cancelled() => {
                    println!("Task was cancelled");
                    break;
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    println!("Task is running");
                }
            }
        }
    });

    // 模拟一些工作
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // 取消任务
    token.cancel();
    
    // 等待任务完成
    let _ = task.await;
}
```

### 注意事项

- **同步取消**：`CancellationToken` 可以在任意线程中使用，适合多线程环境。
- **嵌套结构**：可以创建子令牌，使得取消操作更加灵活。

如果你有进一步的问题或需要更多信息，请告诉我！
