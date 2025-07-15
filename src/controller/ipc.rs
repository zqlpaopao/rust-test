// // https://mp.weixin.qq.com/s/x181rtEj06JSyd9EqfTTKQ
//
// use memmap2::{MmapMut, MmapOptions};
// use std::fs::OpenOptions;
// use std::os::fd::{AsRawFd, RawFd};
// use std::path::Path;
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::Arc;
//
// const HEADER_SIZE: usize = 16; // 头部大小：head 和 tail 各占 8 字节
// const RING_CAPACITY: usize = 1024 * 1024; // 环形缓冲区容量：1 MB
//
// pub struct RingBuffer {
//     m_map: MmapMut,
//     head: *mut AtomicUsize, // 读指针
//     tail: *mut AtomicUsize, // 写指针
//     buffer_start: *mut u8,  // 数据区起始地址
// }
//
// impl RingBuffer {
//     pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
//         // 创建或打开共享内存文件
//         let file = OpenOptions::new()
//             .read(true)
//             .write(true)
//             .create(true)
//             .open(path)?;
//
//         //设置文件大小
//         file.set_len((HEADER_SIZE + RING_CAPACITY) as u64)?;
//
//         //内存映射
//         let mut m_map = unsafe { MmapOptions::new().map_mut(&file)? };
//
//         //设置指针位置
//         let head = m_map.as_mut_ptr() as *mut AtomicUsize;
//         let tail = unsafe { m_map.as_mut_ptr().add(8) as *mut AtomicUsize };
//         let buffer_start = unsafe { m_map.as_mut_ptr().add(HEADER_SIZE) };
//
//         // 初始化 head 和 tail 为 0
//         unsafe {
//             (*head).store(0, Ordering::Relaxed);
//             (*tail).store(0, Ordering::Relaxed);
//         }
//
//         Ok(Self {
//             m_map,
//             head,
//             tail,
//             buffer_start,
//         })
//     }
//
//     // 无阻塞消息读取
//     pub fn pop(&self) -> Option<&Request> {
//         //使用 Acquire 语义读取 head 和 tail
//         let head = unsafe { (*self.head).load(Ordering::Acquire) };
//         let tail = unsafe { (*self.tail).load(Ordering::Acquire) };
//
//         //检查是否有消息
//         if head == tail {
//             return None;
//         }
//
//         // 直接将内存地址转换为 Request 引用
//         let ptr = unsafe { self.buffer_start.add(head) as *const Request };
//         let request = unsafe { &*ptr };
//
//         // 更新 head 指针
//         let next_head = (head + std::mem::size_of::<Request>()) % RING_CAPACITY;
//         unsafe { (*self.head).store(next_head, Ordering::Release) };
//
//         Some(request)
//     }
// }
//
// // 零拷贝消息结构
// #[repr(C)]
// pub struct Request {
//     request_id: u64,     //请求 ID
//     payload_length: u32, //有效载荷长度
//     _reserved: u32,      //保留字段，用于内存对齐
//     payload: [u8; 512],
// }
//
// impl Request {
//     pub fn new(request_id: u64, payload: &[u8]) -> Self {
//         let mut msg = Request {
//             request_id,
//             payload_length: payload.len() as u32,
//             _reserved: 0,
//             payload: [0u8; 512],
//         };
//
//         //复制数据到载荷数组
//         msg.payload[..payload.len()].copy_from_slice(payload);
//         msg
//     }
// }
//
// // #[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
// use nix::sys::eventfd::{EventFd, EfdFlags};
// use tokio::io::unix::AsyncFd;
//
// // 创建异步事件文件描述符
// // #[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
// fn create_eventfd() -> Result<OwnedFd> {
//     let res = unsafe { libc::eventfd(0, EfdFlags::empty().bits()) };
//     Errno::result(res).map(|r| unsafe { OwnedFd::from_raw_fd(r) })
// }
//
// // 异步消费者任务
// async fn start_consumer(ring: Arc<RingBuffer>, mut event_fd: AsyncFd<RawFd>) {
//     loop {
//         // 等待事件通知
//         let mut guard = event_fd.readable().await.unwrap();
//
//         // 读取 eventfd 清除事件
//         let mut buf = [0u8; 8];
//         let _ = nix::unistd::read(event_fd.as_raw_fd(), &mut buf);
//         guard.clear_ready();
//
//         // 批量处理所有待处理消息
//         while let Some(request) = ring.pop() {
//             // process_request(request).await;
//         }
//     }
// }
