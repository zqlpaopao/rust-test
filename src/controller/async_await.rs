use chrono::{DateTime, Local};
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, SystemTime};

/// 封装了Future 且带有发送端，等待执行器去`poll`
struct Task {
    //进行中的Future，在未来的某个时间点会被完成
    //BoxFuture<'a,T> = Pin<alloc::boxes::Box<dyn Future<Output = T> + Send + 'a>>
    future: Mutex<BoxFuture<'static, ()>>,

    //可以将任务自身放入任务通道中，等待执行器的poll
    //Task需共享所有权，使用Arc智能指针包裹
    task_sender: Sender<Arc<Task>>,
    task_name: String,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("发送任务失败")
    }
}

///任务执行器 负责从通道中接受任务执行
struct Executor {
    task_receiver: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        //使用while let 循环从channel中去task
        while let Ok(task) = self.task_receiver.recv() {
            //使用waker_ref方法生成WakeRef,需要task是Arc<_>类型
            //需要task实现ArcWake trait
            let waker = waker_ref(&task);
            //构建context
            let context = &mut Context::from_waker(&*waker);
            let mut future_slot = task.future.lock().unwrap();
            let status = future_slot.as_mut().poll(context);
            let current_time = current_now();
            println!(
                "{:?}任务状态:{:?} 任务名称{:?}",
                current_time, status, &task.task_name
            );
            println!(
                "  id {:?} name {:?}",
                std::thread::current().id(),
                std::thread::current().name()
            );
        }
    }
}

/// 负责创建新的`Future` 然后将它发送到任务通道中
struct Spawner {
    task_sender: Sender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send, task_name: &str) {
        //boxed方法需要引入futures::future::FutureExt
        //Pin 住 future
        let future: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> = future.boxed();

        //使用task 封装future
        let task = Arc::new(Task {
            future: Mutex::new(future),
            task_sender: self.task_sender.clone(),
            task_name: String::from(task_name),
        });

        //发送task 到 channel
        self.task_sender.send(task).expect("发送任务失败");
    }
}
// 实现一个Future，调用wake方法通知executor执行
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shard_state = self.shared_state.lock().unwrap();
        if shard_state.completed {
            let current_time = current_now();
            println!("{:?} task ready", current_time);
            Poll::Ready(())
        } else {
            //poll 方法将waker传入Future中
            shard_state.waker = Some(cx.waker().clone());
            let current_time = current_now();
            println!("{:?} task pending", current_time);
            Poll::Pending
        }
    }
}
impl TimerFuture {
    /// 创建一个新的`TimerFuture`,在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        //创建新线程
        let thread_shard_state = shared_state.clone();
        thread::spawn(move || {
            let current_time = current_now();
            println!("{:?} 任务睡眠中。。。", current_time);
            thread::sleep(duration);
            let mut shared_state = thread_shard_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                let current_time = current_now();

                println!("{:?} 任务结束通知executor ...", current_time);

                waker.wake_by_ref()
            }
        });
        TimerFuture { shared_state }
    }
}

/// 在Future和等待的线程间共享状态
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}
fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (task_sender, task_receiver) = mpsc::channel::<Arc<Task>>();
    (Executor { task_receiver }, Spawner { task_sender })
}

fn current_now() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn test_async() {
    let (executor, spawner) = new_executor_and_spawner();
    async fn hello() {
        thread::sleep(Duration::from_secs(1));
        let current_time = current_now();
        let msg = format!("{:?} from async hello,do something......", current_time);
        println!("{:?}", msg);
    }

    spawner.spawn(
        async {
            hello().await;
        },
        "任务1",
    );

    spawner.spawn(
        async {
            hello().await;
            TimerFuture::new(Duration::new(1, 0)).await;
            hello().await;
        },
        "任务2",
    );

    spawner.spawn(
        async {
            hello().await;
        },
        "任务4",
    );
    drop(spawner);
    executor.run();
}
