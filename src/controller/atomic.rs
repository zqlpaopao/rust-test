#![allow(dead_code)]
// use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// https://mp.weixin.qq.com/s/vNQor5cqGC8B38F6twHJJw

/// 包括AtomicBool、AtomicIsize、AtomicUsize、AtomicPtr等。
/// 这些类型提供了原子操作的方法，如load、store、fetch_add、fetch_sub等。

/// 内存顺序
/// SeqCst：顺序一致性（Sequential Consistency），确保所有线程看到相同的操作顺序。
//     用 SeqCst 在性能上可能会有一些代价，因为它需要在 CPU 和内存之间进行更多的同步。
/// Acquire：获取操作，确保后续读操作不会被重排序到当前操作之前。
//      任何在获取操作之后执行的读操作都能看到获取操作之前写入的值。
/// Release：释放操作，确保之前的写操作不会被重排序到当前操作之后。
//      任何在释放操作之前执行的写操作都能被后续的获取操作看到。
/// Relaxed：松散顺序，不保证操作的顺序和可见性。
//      使用 Relaxed 可以在某些情况下提高性能，但需要非常小心地使用，通常只在特定的情况下使用，例如计数器。
/// AcqRel 是 Rust 中的内存顺序之一，用于原子操作。它结合了 Acquire 和 Release 的特性，既可以用来加载数据（使用 Acquire 顺序）也可以用来存储数据（使用 Release 顺序）。这种顺序主要应用于那些既有加载又有存储操作的原子指令。
//      在 compare_and_swap 操作中，如果实际上没有执行存储操作，那么这个操作只具有 Acquire 顺序。然而，使用 AcqRel 顺序总是会避免使用 Relaxed 顺序。
//      以下是一些关键点：
//      对于加载操作，使用 Acquire 顺序。
//      对于存储操作，使用 Release 顺序。
//      在 compare_and_swap 操作中，如果没有实际执行存储操作，则只具有 Acquire 顺序。
//      AcqRel 顺序永远不会执行 Relaxed 访问。
//      这种顺序仅适用于同时包含加载和存储操作的原子指令。

pub fn test_atomic() {
    //
    //  test_atomics()
}

fn test_atomics() {
    println!("线程安全的计数器SeqCst..严格的执行顺序");

    let atomic_counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_i| {
            let counter = atomic_counter.clone();
            thread::spawn(move || {
                for _ in 0..1000 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {}", atomic_counter.load(Ordering::SeqCst));
    println!();

    println!("线程安全的标志位Acquire读取严格 Release写操作的严格性..严格的执行顺序");

    let atomic_flag = Arc::new(AtomicBool::new(false));

    let handles: Vec<_> = (0..10)
        .map(|_i| {
            let flag = atomic_flag.clone();
            thread::spawn(move || {
                //swap(false, ...)：该方法将原子布尔值设置为 false，并返回之前的值
                // if flag.swap(false, Ordering::SeqCst) {
                //
                // }
                let flags = flag.load(Ordering::Acquire);
                println!("flag = {}", flags);
                if flags {
                    flag.store(false, Ordering::Release);
                } else {
                    flag.store(true, Ordering::Release);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    println!("flag = {}", atomic_flag.load(Ordering::Acquire));
    println!();
}

/***************************************** 无锁队列 ************************************/
//
// struct Node<T>{
//     value: Option<T>,
//     next:AtomicPtr<Node<T>>,
// }
//
// pub struct LockFreeQueue<T>{
//     head: AtomicPtr<Node<T>>,
//     tail: AtomicPtr<Node<T>>,
// }
// impl<T> LockFreeQueue<T>{
//     pub fn new() -> Self{
//         let dummy_node = Box::into_raw(
//             Box::new(
//                 Node{value:None,next:AtomicPtr::new(null_mut())}
//             )
//         );
//         Self{ head: AtomicPtr::new(dummy_node), tail: AtomicPtr::new(dummy_node) }
//     }
//
//     pub fn enqueue(&self, value: T){
//         let new_node = Box::into_raw(
//             Box::new(
//                 Node{ value: Some(value), next: AtomicPtr::new(null_mut()) }
//             )
//         );
//         loop{
//             let tail  = self.tail.load(Ordering::Acquire);
//             let tail_next = unsafe { (*tail).next.load(Ordering::Acquire) };
//             if tail_next.is_null() {
//                 if unsafe{
//                     (*tail).next.compare_exchange(ptr::null_mut(),new_node,Ordering::Release,Ordering::Release) == ptr::null_mut()
//                 }{
//                     self.tail.compare_exchange(tail, new_node, Ordering::Release,Ordering::Release).unwrap();
//                     break;
//                 }else {
//                     self.tail.compare_exchange(tail, new_node, Ordering::AcqRel, Ordering::Acquire).unwrap();
//                 }
//             }
//         }
//     }
//
//     pub fn dequeue(&self) -> Option<T>{
//         loop{
//             let head = self.head.load(Ordering::Acquire);
//             let tail = self.tail.load(Ordering::Acquire);
//             let head_next = unsafe { (*head).next.load(Ordering::Acquire) };
//
//             if head == tail{
//                 if head_next.is_null() {
//                     return None;
//                 }
//                 self.tail.compare_exchange(head, head_next, Ordering::Release,Ordering::Release).unwrap();
//             }else {
//                 let value = unsafe { (*head_next).value.take().unwrap() };
//                 if self.head.compare_exchange(head, tail, Ordering::Release,Ordering::Release) ==   head {
//                     unsafe{Box::from_raw(head)};
//                     return Some(value);
//                 }
//             }
//         }
//     }
// }
//
// impl<T> Drop for LockFreeQueue<T>{
//     fn drop(&mut self){
//         let mut current = self.head.load(Ordering::Acquire);
//         while !current.is_null(){
//             let next = unsafe { (*current).next.load(Ordering::Acquire) };
//             unsafe{
//                 Box::from_raw(current)
//             }
//             current = next;
//
//         }
//     }
// }
