

# 1 多发送者，单接收者

标准库提供了通道`std::sync::mpsc`，其中`mpsc`是*multiple producer, single consumer*的缩写，代表了该通道支持多个发送者，**但是只支持唯一的接收者。** 当然，支持多个发送者也意味着支持单个发送者，我们先来看看单发送者、单接收者的简单例子:

```
use std::sync::mpsc;
use std::thread;

pub fn test_mpsc(){
    mpsc_channel()
}


//多发送者 单接受者
fn mpsc_channel(){
    let ( tx,rx) = mpsc::channel();

    for i in 0..10{
        let t = tx.clone();
        thread::spawn(move ||{
            t.send(i).unwrap();
        });
    }

    for re in rx{
        println!("receive {}",re);

    }

}

receive 0
receive 2
receive 1
receive 5
receive 6
receive 3
receive 7
receive 4
receive 8
receive 9
```

