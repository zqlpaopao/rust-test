#![allow(unused)]
use rayon::prelude::*;
use std::sync::mpsc::channel;

pub async fn test_rayon() {
    take_any().await;
}

async fn take_any() {
    let result: Vec<_> = (0..100)
        .into_par_iter()
        .skip_any_while(|x| {
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            if *x < 50 {
                Some(()).is_some()
            } else {
                println!("x > 50 {} x", x);

                None::<bool>.is_some()
            }
        })
        .collect();
    println!("{:?}", result);

    assert!(result.len() >= 50);
    assert!(result.windows(2).all(|w| w[0] < w[1]));
}

async fn panic_fuse() {
    use rayon::prelude::*;
    use std::{thread, time};

    (0..1_000_000).into_par_iter().panic_fuse().for_each(|i| {
        println!(
            " num {:?} id {:?} name {:?}",
            i,
            std::thread::current().id(),
            std::thread::current().name()
        );
        // simulate some work
        thread::sleep(time::Duration::from_secs(10));
        assert!(i > 0); // oops!
    });
}

async fn while_some() {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let counter = AtomicUsize::new(0);
    let value = (0_i32..2048)
        .into_par_iter()
        .map(|x| {
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            counter.fetch_add(1, Ordering::SeqCst);
            if x < 1024 {
                Some(x)
            } else {
                None
            }
        })
        .while_some()
        .max();

    println!(" value {:?}", value);
    println!(" counter {:?}", counter.load(Ordering::SeqCst));
    assert!(value < Some(1024));
    assert!(counter.load(Ordering::SeqCst) < 2048); // should not have visited every single one
}

async fn any() {
    let a = [0, 12, 3, 4, 0, 23, 0];

    let is_valid = a.par_iter().all(|&x| {
        println!("{}", x);
        x > 10
    });
    println!("{}", is_valid)
}

async fn find_map_any() {
    let c = ["lol", "NaN", "5", "5"];

    let found_number = c.par_iter().find_map_any(|s| s.parse().ok());
    println!("{:#?}", found_number);

    assert_eq!(found_number, Some(5));
}

async fn fold() {
    let s = ['a', 'b', 'c', 'd', 'e']
        .par_iter()
        .fold(
            || String::new(),
            |mut s: String, c: &char| {
                s.push(*c);
                s
            },
        )
        .reduce(
            || String::new(),
            |mut a: String, b: String| {
                a.push_str(&b);
                a
            },
        );
    println!("s {}", s);
}

async fn try_reduce() {
    println!("{:?} {:?}", sum_squares(0..5), Some(0 + 1 + 4 + 9 + 16));

    // The sum might overflow
    println!("{:?} ", sum_squares(0..10_000));

    // Or the squares might overflow before it even reaches `try_reduce`
    println!("{:?} ", sum_squares(1_000_000..1_000_001));
}
// Compute the sum of squares, being careful about overflow.
fn sum_squares<I: IntoParallelIterator<Item = i32>>(iter: I) -> Option<i32> {
    iter.into_par_iter()
        .map(|i| i.checked_mul(i)) // square each item,
        .try_reduce(|| 0, i32::checked_add) // and add them up!
}

async fn reduce_with() {
    let sums = [(0, 1), (5, 6), (16, 2), (8, 9)]
        .par_iter() // iterating over &(i32, i32)
        .cloned() // iterating over (i32, i32)
        .reduce_with(|a, b| {
            println!(
                " num {:?} id {:?} name {:?}",
                b,
                std::thread::current().id(),
                std::thread::current().name()
            );
            (a.0 + b.0, a.1 + b.1)
        })
        .unwrap();
    println!(" {:?}", sums);
}

//
async fn reduce() {
    let sums = [(0, 1), (5, 6), (16, 2), (8, 9)]
        .par_iter()
        .cloned()
        .reduce(
            || (0, 0),
            |a, b| {
                println!(
                    " num {:?} id {:?} name {:?}",
                    b,
                    std::thread::current().id(),
                    std::thread::current().name()
                );
                (a.0 + b.0, a.1 + b.1)
            },
        );

    println!("sums {:?}", sums);
}

async fn flatten_iter() {
    let x: Vec<Vec<_>> = vec![vec![1, 2], vec![3, 4]];
    let iters: Vec<_> = x.into_iter().map(Vec::into_iter).collect();
    let y: Vec<_> = iters.clone().into_par_iter().flatten_iter().collect();
    println!("{:?} {:?}", iters.clone(), y)
}

async fn flatten() {
    let x: Vec<Vec<_>> = vec![vec![1, 2], vec![3, 4]];
    let y: Vec<_> = x.into_par_iter().flatten().collect();
    println!("y {:?}", y)
}

async fn flat_map_iter() {
    let a = [[1, 2], [3, 4], [5, 6], [7, 8]];
    use std::cell::RefCell;
    let par_iter = a.par_iter().flat_map_iter(|a| {
        // The serial iterator doesn't have to be thread-safe, just its items.
        let cell_iter = RefCell::new(a.iter().cloned());
        std::iter::from_fn(move || cell_iter.borrow_mut().next())
    });

    let vec: Vec<_> = par_iter.collect();
    println!("vec {:?}", vec);
}

async fn flat_map() {
    let words = [[1, 2], [3, 4], [5, 6], [7, 8]];

    let letters: Vec<_> = words
        .par_iter()
        .cloned()
        .flat_map(|word| {
            println!(
                " num {:?} id {:?} name {:?}",
                word,
                std::thread::current().id(),
                std::thread::current().name()
            );
            let t = word.to_vec();
            println!("t {:?}", t);
            t
        })
        .collect();

    println!("{:?}", words);
    println!("{:?}", letters);
}

async fn filter_map() {
    let mut par_iter: Vec<_> = (0..10)
        .into_par_iter()
        .filter_map(|x| {
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            if x % 2 == 0 {
                Some(x)
            } else {
                None
            }
        })
        .collect();
    println!("par_iter {:?}", par_iter);
}

async fn filter() {
    // let    mut par_iter : Vec<_> = (0..10)//可变 也可以
    let par_iter: Vec<_> = (0..10).into_par_iter().filter(|x| x % 2 == 0).collect();

    println!(" {:?}", par_iter);
}

//根据原值进行操作，然后新的操作结果
async fn update() {
    let double: Vec<_> = (0..5)
        .into_par_iter()
        .update(|x| {
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            *x *= 2
        })
        .collect();

    println!("init double {:?}", double);
}

//inspect() 方法接受一个闭包函数作为参数，该闭包函数会接收迭代器的每个元素作为参数，
// 并在每次迭代时执行。闭包函数可以用于执行任意操作，例如打印元素、记录日志或进行其他的副作用操作。
async fn inspect() {
    let a = [1, 4, 2, 3];

    // this iterator sequence is complex.
    let sum = a
        .par_iter()
        .cloned()
        .filter(|&x| x % 2 == 0)
        .reduce(|| 0, |sum, i| sum + i);

    println!("{}", sum);

    // let's add some inspect() calls to investigate what's happening
    let sum = a
        .par_iter()
        .cloned()
        .inspect(|x| println!("about to filter: {}", x))
        .filter(|&x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {}", x))
        .reduce(|| 0, |sum, i| sum + i);

    println!("{}", sum);
}

async fn copied() {
    let a = vec![1, 2, 3];

    let v_cloned: Vec<_> = a.par_iter().copied().collect();

    // let vp : Vec<_> = a.par_iter().map(|&x|{x}).collect();//x shi i32
    let vp: Vec<_> = a.par_iter().map(|x| x).collect(); //&x shi &i32

    println!(
        "a {:?}  vp {:?},a address {:p},vp address {:p}",
        a, vp, &a, &vp
    );
}

async fn cloned() {
    let a = vec![1, 2, 3];

    let v_cloned: Vec<_> = a.par_iter().cloned().collect();

    // let vp : Vec<_> = a.par_iter().map(|&x|{x}).collect();//x shi i32
    let vp: Vec<_> = a.par_iter().map(|x| x).collect(); //&x shi &i32

    println!(
        "a {:?}  vp {:?},a address {:p},vp address {:p}",
        a, vp, &a, &vp
    );
}

async fn map_init() {
    let a: Vec<_> = (1i32..10)
        .into_par_iter()
        .map_init(
            || 10,
            |r, x| {
                println!(
                    " num {:?} id {:?} name {:?}",
                    x,
                    std::thread::current().id(),
                    std::thread::current().name()
                );
                *r += x;
                *r
            },
        )
        .collect();
    println!(" a {:?}", a);
}

//可传递 初始化的
//where
//         F: Fn(&mut T, Self::Item) -> R + Sync + Send,
//         T: Send + Clone,
//         R: Send,
async fn map_with() {
    let (sender, receiver) = channel();

    let mut res: Vec<_> = (0..5)
        .into_par_iter()
        .map_with(sender, |s, v| {
            println!(
                " num {:?} id {:?} name {:?}",
                v,
                std::thread::current().id(),
                std::thread::current().name()
            );
            s.send(v).unwrap();
            v
        })
        .collect();

    println!("init res  {:?}", res);
    res.sort();
    println!("res  {:?}", res);
    let r_rec: Vec<_> = receiver.iter().collect();
    println!("init r_rec  {:?}", r_rec);
}

//map 迭代器执行 返回新的迭代器
async fn map() {
    let mut par_iter = (0..5).into_par_iter().map(|x| {
        println!(
            " num {:?} id {:?} name {:?}",
            x,
            std::thread::current().id(),
            std::thread::current().name()
        );
        x * 2
    });
    let double: Vec<_> = par_iter.collect();
    println!("double {:?}", double);
}

//
async fn try_for_each_init() {
    let mut v = vec![0u8; 1_0];
    use rand::Rng;

    v.par_chunks_mut(10)
        .try_for_each_init(
            || rand::thread_rng(),
            |rng, chunk| {
                println!(
                    " num {:?} id {:?} name {:?}",
                    chunk,
                    std::thread::current().id(),
                    std::thread::current().name()
                );
                rng.try_fill(chunk)
            },
        )
        .expect("expected no rand errors");

    println!("v {:?}", v);
    // There's a remote chance that this will fail...
}

async fn try_for_each_with2() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    use std::sync::Mutex;
    // 使用 Mutex 包装可变的累加器
    let sum = Mutex::new(0);

    // 使用 try_for_each_with 在并行环境中对每个数字执行操作
    let result = numbers.par_iter().try_for_each_with(&sum, |acc, &num| {
        // 执行操作，这里尝试将数字累加到累加器中
        let mut acc = acc.lock().unwrap();
        *acc += num;
        println!(
            " num {} id {:?} name {:?}",
            acc,
            std::thread::current().id(),
            std::thread::current().name()
        );
        // 模拟可能的错误，如果数字为3，则返回一个错误
        if num == 3 {
            Err("Encountered an error!")
        } else {
            Ok(())
        }
    });

    // 检查操作是否成功
    match result {
        Ok(()) => println!(
            "All operations completed successfully. Sum: {}",
            *sum.lock().unwrap()
        ),
        Err(err) => println!("Error occurred: {}", err),
    }
    println!("numbers {:?}", numbers);
    println!("numbers {:?}", sum);
}

//传入可变元素并操作
async fn try_for_each_with() {
    let number = vec![1, 2, 3, 4, 5];
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();

    let res = (0..10).into_par_iter().try_for_each_with(sender, |s, x| {
        println!(
            " num {} id {:?} name {:?}",
            x,
            std::thread::current().id(),
            std::thread::current().name()
        );
        if x == 3 {
            Err("Encountered an error!")
        } else {
            s.send(x).unwrap();
            Ok(())
        }
    });
    println!("init res {:?}", res);

    let mut data: Vec<_> = receiver.iter().collect();
    println!("init data {:?}", data);
    data.sort();
    println!("sort data {:?}", data);
}

// for_each 迭代元素并返回结果
async fn try_for_each() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 使用 try_for_each 在并行环境中对每个数字执行操作
    let result = numbers.par_iter().try_for_each(|&num| {
        // 执行操作，这里尝试将数字打印出来
        println!(
            " num {} id {:?} name {:?}",
            num,
            std::thread::current().id(),
            std::thread::current().name()
        );
        // 模拟可能的错误，如果数字为3，则返回一个错误
        if num == 3 {
            Err("Encountered an error!")
        } else {
            Ok(())
        }
    });

    // 检查操作是否成功
    match result {
        Ok(()) => println!("All operations completed successfully."),
        Err(err) => println!("Error occurred: {}", err),
    }
}

//可传递一个初始值，变为可变的参与活动
async fn for_each_init() {
    use rand::Rng;
    let mut v = vec![0u8; 1_0];

    v.par_chunks_mut(1000).for_each_init(
        || rand::thread_rng(),
        |rng, chunk| {
            println!(
                " rng {:?},num {:?} id {:?} name {:?}",
                rng,
                chunk,
                std::thread::current().id(),
                std::thread::current().name()
            );
            rng.fill(chunk)
        },
    );

    println!("v {:?}", v);

    // There's a remote chance that this will fail...
}

//并行计算并放入chan中
async fn for_each_with() {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();

    (0..10).into_par_iter().for_each_with(sender, |s, x| {
        println!(
            " num {} id {:?} name {:?}",
            x,
            std::thread::current().id(),
            std::thread::current().name()
        );
        s.send(x).unwrap();
    });

    let mut res: Vec<_> = receiver.iter().collect();
    println!("init res {:?}", res);
    res.sort();
    println!("sort res {:?}", res);
}

//并行执行`OP`迭代器生成的每个项目。
async fn for_each() {
    (0..100).into_par_iter().for_each(|x| {
        println!(
            " num {} id {:?} name {:?}",
            x,
            std::thread::current().id(),
            std::thread::current().name()
        );
        std::thread::sleep(std::time::Duration::from_secs(x as u64));
    });
}

async fn test() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum: Vec<()> = numbers
        .par_iter()
        .map(|v| {
            println!(
                " num {} id {:?} name {:?}",
                v,
                std::thread::current().id(),
                std::thread::current().name()
            );
            std::thread::sleep(std::time::Duration::from_secs(*v as u64));
            ()
        })
        .collect();

    println!("Sum: {:?}", sum);
}
