// use std::cell::UnsafeCell;
// use std::ptr::null_mut;
// use std::sync::atomic::{AtomicPtr, Ordering};
// use std::sync::Mutex;
// use std::task::{Poll, Waker};
//
//
// // https://mp.weixin.qq.com/s/Nt7ImBmhSqgUKei_5VK-Tw
// struct Node<T> {
//     data : UnsafeCell<Option<T>>,
//     next : AtomicPtr<Node<T>>,
// }
//
// pub struct LockFreeQueue<T> {
//     head: AtomicPtr<Node<T>>,
//     tail: AtomicPtr<Node<T>>,
// }
//
// //手动实现Send/Sync保证线程安全
// unsafe impl<T: Send> Send for Node<T> {}
// unsafe impl<T: Send> Sync for Node<T> {}
//
//
//
// // 队列初始化
//
// impl<T> LockFreeQueue<T> {
//     pub fn new() -> Self{
//         let dummy = Box::into_raw(
//             Box::new(Node{
//             data: UnsafeCell::new(None),
//             next: AtomicPtr::new(null_mut()),
//             }
//             ));
//
//         LockFreeQueue{
//             head: AtomicPtr::new(dummy),
//             tail: AtomicPtr::new(dummy),
//         }
//     }
//
//     //生产者入队操作
//     pub fn enqueue(&self, value: T) {
//         let new_node = Box::into_raw(
//             Box::new(
//                 Node{
//                     data: UnsafeCell::new(Some(value)),
//                     next: AtomicPtr::new(null_mut()),
//                 }
//             )
//         );
//
//         let prev = self.tail.swap(new_node,Ordering::AcqRel);
//         unsafe {
//             (*prev).next.store(new_node, Ordering::Release);
//         }
//     }
//
//     pub fn dequeue(&self) -> Option<T> {
//         let head_ptr = self.head.load(Ordering::Acquire);
//         let next_ptr = unsafe {(*head_ptr).next.load(Ordering::Acquire)};
//
//         if next_ptr.is_null() {
//             return None;
//         }
//
//         let value = unsafe{
//             (*next_ptr).data.get().replace(None).unwrap();
//         };
//         self.head.store(next_ptr, Ordering::Release);
//         unsafe {
//            let _ =  Box::from_raw(head_ptr);
//         }
//         Some(value)
//     }
// }
//
// pub struct AsyncReceiver<T>{
//     queue: LockFreeQueue<T>,
//     waker : Mutex<Option<Waker>>,
// }
//
// impl<T> AsyncReceiver<T> {
//     pub async fn recv(&self) -> T {
//         futures::future::poll_fn(|cx| {
//             if let Some(val) = self.queue.dequeue() {
//                 Poll::Ready(val)
//             } else {
//                 let mut guard = self.waker.lock().unwrap();
//                 *guard = Some(cx.waker().clone());
//                 Poll::Pending
//             }
//         }).await
//     }
// }
//
// // 增强的生产者接口
// impl<T> LockFreeQueue<T> {
//     pub fn enqueue_and_wake(&self, value: T, waker: &Mutex<Option<Waker>>) {
//         self.enqueue(value);
//         if let Some(w) = waker.lock().unwrap().take() {
//             w.wake();
//         }
//     }
// }