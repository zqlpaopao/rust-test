// https://mp.weixin.qq.com/s/gCqLB9uJ9YTuQWuzvjYZMw

use crate::controller::thread_pool;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 1..=size {
            let receiver = receiver.clone();
            workers.push(Worker::new(id, receiver));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    //     'lop :loop{
    //     break 'lop f();
    // }
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            let time = SystemTime::now();
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("{:?} shutdown worker:{}", time, worker.id);
            }
        }
        // println

        let time = SystemTime::now();
        println!("{:?} shutdown thread pool", time);
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {id} is working...");
                    job();
                }
                Err(err) => {
                    println!("Worker {id} disconnected {err};exit");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(handle),
        }
    }
}

pub fn test_thread() {
    let pool = thread_pool::ThreadPool::new(2);
    let f1 = || {
        let result = 1 + 1;
        println!("result:{result}");
    };
    pool.execute(f1);
    pool.execute(f1);
    pool.execute(f1);
}

#[cfg(test)]
mod test {
    use crate::controller::thread_pool;
    #[test]
    fn test_thread_pool() {
        let pool = thread_pool::ThreadPool::new(2);
        let f1 = || {
            let result = 1 + 1;
            println!("result:{result}");
        };
        pool.execute(f1);
        pool.execute(f1);
        pool.execute(f1);
    }
}
