#![allow(unused)]
use std;
use std::cell::Cell;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::marker::PhantomPinned;
#[warn(dead_code)]
use std::mem::size_of_val;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;
use std::sync::{mpsc, Arc, Barrier, Condvar, Mutex, Once, RwLock};
use std::time::Duration;
use std::{io, thread};
use thread_local::ThreadLocal;
use tokio::sync::Semaphore;

pub async fn test_rust() {
    test_pin()
}
/******************************************* Pin !Unpin   **********************************************/
#[derive(Debug)]
struct Rabbit {
    name: String,
    p: *const String,
    _marker: PhantomPinned,
}

impl Rabbit {
    fn new(txt: &str) -> Self {
        Rabbit {
            name: String::from(txt),
            p: std::ptr::null(),
            // 这个标记可以让Rabbit自动实现!Unpin
            _marker: PhantomPinned,
        }
    }
    fn init(self: Pin<&mut Self>) {
        let self_ref: *const String = &self.name;
        let this = unsafe { self.get_unchecked_mut() };
        this.p = self_ref;
    }

    fn name(self: Pin<&Self>) -> &str {
        &self.get_ref().name
    }

    fn p_value(&self) -> &String {
        assert!(
            !self.p.is_null(),
            "Rabbit::p_value called without Rabbit::init being called first"
        );
        unsafe { &*(self.p) }
    }
}
fn test_pin() {
    // let mut  rabbit1 = Rabbit::new("小白");
    // rabbit1.init();
    // let mut rabbit2 : Rabbit = Rabbit::new("小黑");
    // rabbit2.init();
    // println!("rabbit1 name: {}, rabbit1 p_value: {} rabbit1 name addr:{:p} rabbit1 p addr:{:p} rabbit1 addr:{:p}",
    //          rabbit1..name(), rabbit1.p_value(),&rabbit1.name,rabbit1.p,&rabbit1);
    //
    // println!("rabbit2 name: {}, rabbit2 p_value: {} rabbit2 name addr:{:p} rabbit2 p addr:{:p} rabbit2 addr:{:p}",
    //          rabbit2.name(), rabbit2.p_value(),&rabbit2.name,rabbit2.p,&rabbit2);
    // println!("-------------------------------------------------");
    // std::mem::swap(&mut rabbit1,&mut rabbit2);
    // println!("rabbit1 name: {}, rabbit1 p_value: {} rabbit1 name addr:{:p} rabbit1 p addr:{:p} rabbit1 addr:{:p}",
    //          rabbit1.name(), rabbit1.p_value(),&rabbit1.name,rabbit1.p,&rabbit1);
    //
    // println!("rabbit2 name: {}, rabbit2 p_value: {} rabbit2 name addr:{:p} rabbit2 p addr:{:p} rabbit2 addr:{:p}",
    //          rabbit2.name(), rabbit2.p_value(),&rabbit2.name,rabbit2.p,&rabbit2);

    // let mut rabbit1 = Rabbit::new("小白");
    // let mut pinned = std::pin::pin!(rabbit1);
    // let mut rabbit1 = unsafe { Pin::new_unchecked(&mut rabbit1) };
    // Rabbit::init(rabbit1.as_mut());
    // let mut rabbit2: Rabbit = Rabbit::new("小黑");
    // let mut rabbit2 = unsafe { Pin::new_unchecked(&mut rabbit2) };
    //
    // Rabbit::init(rabbit2.as_mut());
    // println!("rabbit1 name: {}, rabbit1 p_value: {} rabbit1 name addr:{:p} rabbit1 p addr:{:p} rabbit1 addr:{:p}",
    //          Rabbit::name(rabbit1.as_ref()), Rabbit::p_value(rabbit1.as_ref()),&rabbit1.name,rabbit1.p,&rabbit1);
    // println!("rabbit2 name: {}, rabbit2 p_value: {} rabbit2 name addr:{:p} rabbit2 p addr:{:p} rabbit2 addr:{:p}",
    //          Rabbit::name(rabbit2.as_ref()), Rabbit::p_value(rabbit2.as_ref()),&rabbit2.name,rabbit2.p,&rabbit2);
    // println!("-----------------------------------------");

    // std::mem::swap函数用于交换两个内存地址处的值
    // 由于rabbit1和rabbit2都被Pin住了，所有swap移动值直接编译报错
    //std::mem::swap(rabbit1.get_mut(), rabbit2.get_mut());
    //println!("rabbit1 name: {}, rabbit1 p_value: {} rabbit1 name addr:{:p} rabbit1 p addr:{:p} rabbit1 addr:{:p}",
    // Rabbit::name(rabbit1.as_ref()), Rabbit::p_value(rabbit1.as_ref()),&rabbit1.name,rabbit1.p,&rabbit1);
    //println!("rabbit2 name: {}, rabbit2 p_value: {} rabbit2 name addr:{:p} rabbit2 p addr:{:p} rabbit2 addr:{:p}",
    // Rabbit::name(rabbit2.as_ref()), Rabbit::p_value(rabbit2.as_ref()),&rabbit2.name,rabbit2.p,&rabbit2);
}
/******************************************* 裸指针   **********************************************/
///https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484367&idx=1&sn=0cfdbf35f9e874c0bdb8839df32aed7e&chksm=cfe11dfdf89694eb5c8cfa12fc0618cd7f4f25f1ba7eed8e24f91c49157570a28cf6447ee78c&scene=21#wechat_redirect
fn test_ptr() {
    let mut age = 30;
    //基于值的引用创建不可变和可变引用
    let raw_point1 = &age as *const i32;
    let raw_point2 = &mut age as *mut i32;
    println!("raw point1 value:{}", unsafe { *raw_point1 });
    println!("raw point2 value:{}", unsafe { *raw_point2 });

    let hi = "Hi 2024";
    // 获取“Hi 2024”的内存地址
    let hi_addr = hi.as_ptr() as usize;
    // 字符串长度
    let len = hi.len();
    // 从指定内存地址处读取指定长度的数据
    let data = unsafe { from_utf8_unchecked(from_raw_parts(hi_addr as *const u8, len)) };
    println!("data :{data}");
}
/******************************************* 裸指针 实现Send Sync  **********************************************/
/// Send
#[derive(Debug)]
struct SandBoxI(*mut i32);
unsafe impl Send for SandBoxI {}
unsafe impl Sync for SandBoxI {}

fn test_ptr_sync_send() {
    // let num = 100;
    // let p = SandBoxI(num as *mut i32);
    //
    // let handler = thread::spawn(move || {
    //     println!("{:?}",p);
    // });

    //打印
    // println!("{:?}",p);
    // ^ value borrowed here after move
    // handler.join().unwrap();

    //sync
    // let num = 123;
    // let sand_box = Arc::new(SandBox(num as *mut i32));
    // let sand_box_clone = sand_box.clone();
    // let handle = thread::spawn(move || {
    //     println!("sand box value:{:?}", sand_box_clone);
    // });
    // handle.join().unwrap();
    // println!("sand box value:{:?}", sand_box);
}
///Sync特征表示可以在多线程之间共享一个值，要在多个线程之间共享这个值，
/// 首先要确保这个值或者这个值的引用能够在多线程之间移动，因此还需要实现Send特征。

/******************************************* Semaphore  **********************************************/

async fn test_semaphore() {
    //初始化信号量3
    let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::with_capacity(5);
    //获取信号量 信号量减1
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    println!("start {:?}", permit);
    // drop(permit);

    for i in 0..=5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("i:{i}");
            thread::sleep(Duration::from_secs(10));
            println!("inner {:?}", permit);
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }

    let permit = semaphore.clone().acquire_owned().await.unwrap();
    println!("end {:?}", permit)
}
/******************************************* RWLock  **********************************************/
/// 它同一时刻允许多个读，同一时刻只允许一个写。
fn test_rw_lock() {
    let lock = RwLock::new(100);
    {
        let num1 = lock.read().unwrap();
        let num2 = lock.read().unwrap();
        assert_eq!(*num1, 100);
        assert_eq!(*num2, 100)
    }

    {
        let mut num3 = lock.write().unwrap();
        *num3 += 1;
        assert_eq!(*num3, 101);
    }
}
/******************************************* mpsc channel发送不同类型  **********************************************/
/*
   tokio::sync::mpsc
       类型: 异步
       特点:
           "mpsc"代表多生产者，单消费者。
           专为Tokio异步运行时设计，适用于异步任务间的通信。
           支持在异步环境中无阻塞地发送和接收消息。
   async-channel
       类型: 异步
       特点:
           提供了多生产者，单消费者的异步通道。
           运行时无关，可以与任何异步运行时（如Tokio, async-std等）一起使用。
           简单易用，功能与tokio::sync::mpsc类似，但更为灵活。
   std::sync::mpsc
       类型: 同步
       特点:
           "mpsc"代表多生产者，单消费者。
           适用于不需要异步处理的场景，主要用于线程间的通信。
           提供了阻塞和非阻塞的接收方式。
   crossbeam-channel
       类型: 同步
       特点:
           提供了多生产者，多消费者（MPMC）的支持。
           性能优于std::sync::mpsc，特别是在高并发场景下。
           提供了更丰富的API，如选择性接收（select），定时发送和接收等。

   Tokio的通道原语
       Tokio提供一些通道(channel)，每个通道都有不同的用途:
           mpsc：多生产者，单消费者通道。可以发送许多数值。
           oneshot：单生产者，单消费者通道。可以发送一个单一的值。
           broadcast: 多生产者，多消费者。可以发送许多值。每个接收者看到每个值。
           watch：单生产者，多消费者。可以发送许多值，但不保留历史。接收者只看到最近的值。
*/
fn test_mpsc_chan() {
    #[derive(Debug)]
    enum Animal {
        Dog(String, u8),
        Cat(String, u8),
        Fish(String, u8),
    }
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let dog = Animal::Dog(String::from("dog"), 3);
        let cat = Animal::Cat(String::from("cat"), 4);
        let fish = Animal::Fish(String::from("fish"), 5);
        sender.send(dog).unwrap();
        sender.send(cat).unwrap();
        sender.send(fish).unwrap();
    });
    for msg in receiver {
        println!("{:?}", msg)
    }
}

/******************************************* 只调用一次的函数  **********************************************/
/// 在多线程的环境下，有时我们希望某个函数只执行一次，例如全局变量的初始化。无论哪个线程先调用，都保证全局变量只会被初始化一次，随后其它的线程调用将被忽略。

fn test_once() {
    static mut VERSION: &str = "1.2.3";
    static INIT: Once = Once::new();

    let handler1 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        INIT.call_once(|| unsafe {
            VERSION = "1.2.5";
        });
    });

    let handler2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        INIT.call_once(|| unsafe {
            VERSION = "1.2.8";
        });
    });
    handler1.join().unwrap();
    handler2.join().unwrap();
    println!("VERSION {}", unsafe { VERSION });
}
/******************************************* 条件变量 Condition  **********************************************/
///条件变量Condition Variable经常和Mutex一起使用，可以让线程挂起，直到某个条件发生后再继续执行。
fn test_condition() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    println!("main 锁定值为:{:?}", pair.0.lock().unwrap());

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("子线程修改锁定的值为true");
        *started = true;
        //通知等在次条件上唤醒
        cvar.notify_one();
    });

    // thread::sleep(Duration::from_millis(10));
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    //main 线程等待所里面的变量变为true
    println!("main wait...");
    started = cvar.wait(started).unwrap();
    println!("main 线程{started}")
}

/******************************************* 线程局部变量  **********************************************/
///https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484340&idx=1&sn=5c626fd5d5bcdd453e6868fedf39bdfc&chksm=cfe11d86f8969490bba8c83c848c1b4ecbde307ab6f80290f86c072277366077bfdc67133b35&cur_album_id=3357418700156502025&scene=189#wechat_redirect
/// 标准库
/// 子线程中修改了INIT，但不影响主线程中的值。
fn test_std_thread_local() {
    //使用标准库的thread_local!宏创建线程局部变量
    //使用Cell提供内部可变性
    thread_local! {
        static INIT : Cell<i32> = Cell::new(100);
    }

    INIT.with(|i| {
        println!("main thread i = {}", i.get());
        i.set(i.get() - 1);
        println!("main thread i = {}", i.get());
    });

    println!("main thread INIT VALUE:{}", INIT.get());

    //在线程中使用局部变量
    let handle = thread::spawn(move || {
        INIT.with(|i| {
            println!("child thread i = {}", i.get());
            i.set(i.get() - 2);
            println!("child thread i = {}", i.get());
        })
    });

    handle.join().unwrap();
    println!("main thread INIT VALUE:{}", INIT.get());

    println!("结构体中使用--------------------<");
    test_thread_struct();
    println!("第三方库--------------------<");
}

/// 结构体中使用
struct Task;
impl Task {
    thread_local! {
        static INIT : Cell<i32> = Cell::new(100);
    }

    fn set_init(&self, num: i32) {
        Self::INIT.set(num);
    }

    fn get_init(&self) -> i32 {
        Self::INIT.get()
    }
}

fn test_thread_struct() {
    let task = Task;
    task.set_init(1000);
    println!("main struct thread INIT :{}", task.get_init());
    let handle = thread::spawn(move || {
        task.set_init(99);
        println!("child thread struct INIT:{}", task.get_init());
    });
    handle.join().unwrap();
    // 下面编译错误，因为task所有权已经移动到了子线程中
    // println!("main thread struct INIT:{}", task.get_init());
}

///三方库
fn test_sf_thread_local() {
    //使用第三方库创建本地变量
    let thread_local = Arc::new(ThreadLocal::new());
    //主线程初始化为99
    thread_local.get_or(|| Cell::new(99));
    let mut handlers = Vec::with_capacity(4);
    //创建多线程
    for i in 0..=4 {
        let ti_i = thread_local.clone();
        let handle = thread::spawn(move || {
            let cell = ti_i.get_or(|| Cell::new(100));
            cell.set(cell.get() + 1);
        });
        handlers.push(handle);
    }

    println!("引用计数:{}", Arc::strong_count(&thread_local));
    for handle in handlers {
        handle.join().unwrap();
    }
    println!("引用计数:{}", Arc::strong_count(&thread_local));
    println!("main thread tl:{}", thread_local.get().unwrap().get());
}

/******************************************* 线程屏蔽 Barrier **********************************************/
/// 线程屏蔽 Barrier
///所谓线程屏蔽 就是让多个线程都执行到某个点后才继续执行
fn test_barrier() {
    let cap: usize = 11;
    let mut handlers = Vec::with_capacity(cap);

    //通过Arc在堆上创建多线程安全且共享所有权的Barrier，屏蔽初始为11个线程
    let barrier: Arc<Barrier> = Arc::new(Barrier::new(cap));

    for i in 0..=10 {
        //clone 增加计数
        let b: Arc<Barrier> = barrier.clone();
        handlers.push(thread::spawn(move || {
            println!("thread {i} before wait");
            thread::sleep(Duration::from_secs(i));
            b.wait();
            println!("thread {i} after wait");
        }))
    }

    for handle in handlers {
        handle.join().unwrap();
    }
}

/******************************************* 自定义错误**********************************************/
#[derive(Debug)]
struct MyError {
    code: usize,
    message: String,
}
fn get_a_err1() -> Result<(), MyError> {
    Err(MyError {
        code: 404,
        message: String::from("err1"),
    })
}
fn get_a_err2() -> Result<(), MyError> {
    Err(MyError {
        code: 500,
        message: String::from("err2"),
    })
}

fn test_err() {
    println!("{:?}", get_a_err1());
    println!("{:?}", get_a_err2());

    //是心啊from 自定义错误
    let s = use_my_error();
    match s {
        Ok(_) => {
            println!("ok")
        }
        Err(err) => {
            println!("err :{:?}", err)
        }
    }

    //anyhow thiserror
    let s = test_anyhow();
    match s {
        Ok(_) => {
            println!("is ok")
        }
        Err(err) => {
            println!("err : {}", err)
        }
    }

    //Err(MyError { code: 404, message: "err1" })
    // Err(MyError { code: 500, message: "err2" })
    // err :MyError { code: 10000404, message: "No such file or directory (os error 2)" }
    // err : 没有环境变量
}
///借助From特征，我们可以将系统内部各式各样的错误类型统一为自定义错误类型：
impl From<io::Error> for MyError {
    fn from(value: Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => MyError {
                code: 10000404,
                message: value.to_string(),
            },
            ErrorKind::PermissionDenied => MyError {
                code: 10002,
                message: value.to_string(),
            },
            _ => MyError {
                code: 10003,
                message: value.to_string(),
            },
        }
    }
}

fn use_my_error() -> Result<(), MyError> {
    let s = File::open("./aaa")?;
    Ok(())
}

///anyhow thiserror

#[derive(Debug, thiserror::Error)]
enum MyError1 {
    #[error("没有环境变量")]
    EnvironmentNotFound(#[from] std::env::VarError),
    #[error("IO错误")]
    IOError(#[from] std::io::Error),
}

fn test_anyhow() -> Result<String, MyError1> {
    let data_path = std::env::var("DATA_PATH")?;
    let mut file = File::open(data_path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    println!("content:{}", buf);

    Ok(buf)
}
/******************************************* or else**********************************************/

fn test_or_and() {
    let s1 = Some("123");
    let s2 = Some("234");
    let s3: Option<&str> = None;

    assert_eq!(s1.or(s2), s1);
    assert_eq!(s1.or(s3), s1);
    //and
    assert_eq!(s1.and(s2), s2);
    assert_eq!(s1.and(s3), s3);

    //result
    let ok1: Result<&str, &str> = Ok("ok1");
    let ok2: Result<&str, &str> = Ok("ok2");
    let err1: Result<&str, &str> = Err("err1");
    let err2: Result<&str, &str> = Err("err2");

    assert_eq!(ok1.and(ok2), ok2);
    assert_eq!(ok1.or(ok2), ok1);

    assert_eq!(ok1.and(err1), err1);
    assert_eq!(ok1.or(err1), ok1);
    assert_eq!(err1.or(err2), err2);
    assert_eq!(err1.and(err2), err1);

    //or_else and_then
    //另外一组计算是or_else和and_then，
    // 他俩同or和and的区别仅仅在参数的传递上，or_else和and_then接收一个闭包函数。
    let fn_some = || Some("234");
    let fn_some2 = |_| Some("234");
    assert_eq!(s2.or_else(fn_some), s2);
    assert_eq!(s2.and_then(fn_some2), fn_some());

    // Result
    let fn_ok1 = |_| Ok("ok1");
    let fn_ok2 = |_| Ok("ok2");
    let fn_err1 = |_| Err("err1");
    let fn_err2 = |_| Err("err2");
    assert_eq!(ok1.or_else(fn_ok1), ok1);
    assert_eq!(ok2.and_then(fn_ok2), ok2);
    assert_eq!(err1.or_else(fn_err1), err1);
    assert_eq!(err2.and_then(fn_err2), err2);

    //filter过滤
    // Option支持过滤操作，传入一个闭包，可对特定类型的Option进行过滤。Result类型不支持过滤。
    let closure_fn = |x: &&str| x.starts_with("123");
    assert_eq!(s1.filter(closure_fn), Some("123"));
    assert_eq!(s2.filter(closure_fn), None);

    // map和map_err对结果进行转换
    // Option和Some类型都支持转换，进一步对数据进行加工，
    // 转换操作需要定义并使用闭包。map对正常值进行处理，map_err对错误值进行处理。
    // Option和Result map和map_err转换

    // map
    let char_count = |s: &str| -> usize { s.chars().count() };
    assert_eq!(s1.map(char_count), Some(3));
    assert_eq!(s3.map(char_count), None);
    assert_eq!(ok1.map(char_count), Ok(3));
    assert_eq!(err1.map(char_count), Err("err1"));

    // map_err
    let map_err_char_count = |s: &str| s.chars().count();
    let s = ok1.map_err(map_err_char_count);
    assert_eq!(s, Ok("ok1"));
    assert_eq!(err1.map_err(map_err_char_count), Err(4));

    //在map的基础上提供默认值的算子map_or和map_or_else
    //map_or和map_or_else都是在map的基础上提供默认值，区别是map_or_else通过闭包来实现。

    // Option和Result中map_or和map_or_else提供默认值
    // map_or
    const DEFAULT: usize = 1;
    let fn_closure = |s: &str| s.chars().count();
    assert_eq!(s1.map_or(DEFAULT, fn_closure), 3);
    assert_eq!(s3.map_or(DEFAULT, fn_closure), 1);
    assert_eq!(ok1.map_or(DEFAULT, fn_closure), 3);
    assert_eq!(err1.map_or(DEFAULT, fn_closure), 1);

    // map_or_else
    // 这个闭包可以对Err值进行处理
    let fn_closure_default_value = |s: &str| s.chars().count() + 100;
    assert_eq!(err1.map_or_else(fn_closure_default_value, fn_closure), 104);
    //ok_or和ok_or_else
    //ok_or和ok_or_else将Option转换为Result，其中ok_or接收一个默认的Err参数，ok_or_else接收一个闭包作为Err参数。

    // 将Option转换为Result
    // ok_or 和 ok_or_else
    assert_eq!(s1.ok_or("Error Message"), Ok("123"));
    assert_eq!(s3.ok_or("Error Message"), Err("Error Message"));
    let fn_closure = || "Error Message";
    assert_eq!(s1.ok_or_else(fn_closure), Ok("123"));
    assert_eq!(s3.ok_or_else(fn_closure), Err("Error Message"));
}

/******************************************* 流程控制**********************************************/

fn switch() {
    let num = 50;
    //let if 模式 将满足条件的返回值绑定到result变量
    let result = if num >= 50 { true } else { false };
    println!("result {}", result);

    //if let 模式 模式匹配:pattern = expr
    // 匹配到之后将运行{}中的代码，否则运行else中的代码
    if let 50 = num {
        println!("num == 50");
    } else {
        println!("num != 50");
    }

    // 单分支多模式匹配
    match num {
        // pat|pat 单分支多模式匹配
        20 | 30 => println!("num=20 or num=30"),
        // expr..=expr进行范围匹配
        40..=50 => println!("num>=40 and num<=50"),
        // 匹配守卫，提供额外的条件
        x if x >= 100 => println!("x>=100"),
        1 | 2 | 3 | 4 | 500 if num == 500 => print!("num = 500"),
        // 使用_表示其它的选项
        _ => println!("num not in 20 30 50"),
    }
}

/******************************************* Vec 结构体排序**********************************************/

#[derive(Debug)]
struct Animals(i32, String);

fn test_vec_sort() {
    let mut animals = vec![
        Animals(2, "dog2".to_string()),
        Animals(3, "dog3".to_string()),
        Animals(4, "dog4".to_string()),
        Animals(1, "dog".to_string()),
    ];
    println!("animals 排序前 {:?}", animals);
    animals.sort_by(|a, b| a.0.cmp(&b.0));
    println!("animals 排序后 {:?}", animals);
}

/******************************************* Vec 存储不同类型**********************************************/
#[derive(Debug)]
enum Pen {
    Pencil(String),
    ColorPen(String),
}
trait Write {
    fn write(&self);
}

struct Pencil {
    name: String,
}

impl Write for Pencil {
    fn write(&self) {
        println!("我用{}写字", self.name.as_str())
    }
}

struct ColorPen {
    name: String,
}

impl Write for ColorPen {
    fn write(&self) {
        println!("我用{}写字", self.name.as_str())
    }
}

fn test_vec() {
    // Vec存储不同类型的对象
    // 第一种：存储枚举值
    let mut pens = vec![];
    pens.push(Pen::Pencil("铅笔".to_string()));
    pens.push(Pen::ColorPen("彩色笔".to_string()));
    for pen in &pens {
        println!("{:?}", pen);
    }
    // 第二种：存储特征对象
    let pens: Vec<Box<dyn Write>> = vec![
        Box::new(Pencil {
            name: "铅笔".to_string(),
        }),
        Box::new(ColorPen {
            name: "彩色笔".to_string(),
        }),
    ];
    for pen in &pens {
        pen.write();
    }
}

/******************************************* Deref Drop **********************************************/
///智能指针都实现了Deref特征，使用时将自动解引用。下面是Box指针的简单使用示例：
fn test_zn_ptr() {
    let age_stack = 10;
    let age_heap = Box::new(age_stack);
    println!(
        "age stack:{age_stack}
    age_heap:{age_heap}
    age_stack_addr:{:p}
    age_heap_addr:{:p}",
        &age_stack,
        &(*age_heap)
    );
    // 使用*解引用堆上的数据
    println!("{} {} ", age_stack == *age_heap, age_heap.deref());
}

#[derive(Debug)]
struct SandBox<T>(T);

impl<T> SandBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

///实现Deref
impl<T> Deref for SandBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("解引用...");
        &self.0
    }
}

///实现Drop
impl<T> Drop for SandBox<T> {
    fn drop(&mut self) {
        println!("调用drop方法释放内存")
    }
}

fn test_deref_drop() {
    let sandbox = SandBox("Ruster");
    println!("sanbox:{:?}", sandbox);
    // 使用*解引用运算符获取结构体中的值(如果是指针就是获取指针指向的内存中的值)
    // 如果没有实现Deref接口，*sandbox是会报错的
    println!("sandbox value:{:?}", *sandbox);
}

/******************************************* 地址 **********************************************/

fn test() {
    // 分别打印出切片长度、切片变量地址、切片对应值的地址
    // 和切片对应值的大小，它和切片中的len属性大小一致

    let a: &str = "Rust";
    println!(
        "size of a:{} , address of a:{:p}, value address of a:{:p} , value is:{}, size of data:{}",
        a.len(),
        &a,
        &(*a),
        a,
        size_of_val(&(*a))
    );
    println!(
        "size of a:{} , address of a:{:p}, value address of a:{:p} , value is:{},size of data:{}",
        a.len(),    // 切片长度
        &a,         // 切片变量地址
        a.as_ptr(), // 切片对应值的地址
        a,
        size_of_val(a) // 切片对应值的大小，实际上是切片长度
    );
}
#[derive(Debug)]
struct Message<'a> {
    msg: &'a str,
}

impl<'a> Message<'a> {
    fn new(msg: &'a str) -> Self {
        Self { msg }
    }
}

/************************************ 类型转换 **************************************/
/// 标量类型转换
pub fn bl() {
    let x = 3i32;
    let y: u64 = x as u64;
    println!("x:{} y:{}", x, y);
}

///复杂类型转换
#[derive(Debug)]
struct Animal {
    age: u32,
}

#[derive(Debug)]
struct Long {
    age: u32,
}

impl From<Animal> for Long {
    fn from(value: Animal) -> Self {
        Long { age: value.age }
    }
}

impl Into<Animal> for Long {
    fn into(self) -> Animal {
        Animal { age: self.age }
    }
}
fn fz() {
    let loong = Long { age: 1000 };
    println!("龙：{:?}", loong);
    let long_is_animal: Animal = loong.into();
    println!("long -into->animal：{:?}", long_is_animal);

    let animal = Animal { age: 2000 };
    let long = Long::from(animal);
    println!("animal -from->long：{:?}", long);
}
