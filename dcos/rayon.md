# 1 Rayon - 数据并行计算库

Rayon 是一个Rust的数据并行计算库。它非常轻巧，可以轻松地将顺序计算转换为并行计算。同时保证不会有数据争用情况出现。

## 并行迭代器

使用Rayon，可以轻松地将顺序迭代器转换为并行迭代器：通常，只需将您的`foo.iter()`调用更改为`foo.par_iter()`，其余则由Rayon完成：

```
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- 只需要更改这里
         .map(|&i| i * i)
         .sum()
}
```

**并行迭代器**负责确定如何将数据划分为任务；它会动态适应以达到最佳性能。如果你需要更大的灵活性，那么Rayon还提供了`join`和`scope`函数，允许用户自己创建并行任务。为了获得更多控制，还可以创建自定义线程池，而不是使用Rayon的默认全局线程池。

## 无数据争用

通常大家可能觉得并行执行会产生各种疯狂的错误。不用紧张，Rayon的API均保证无数据争用情况发生，通常可以排除大多数并行错误（尽管不是全部）。换句话说，**只要代码通过编译**，它通常会执行与非并行情况下相同的操作。

对于大多数情况，使用并行迭代器产生可以保证结果与顺序迭代器结果相同。不过需要特别注意的是：如果您的迭代器有副作用（例如，通过`Rust通道`将方法发送到其他线程，或者磁盘写入），这些副作用可能会以不同的顺序发生。还要注意，在某些情况下，并行迭代器提供了具有更高性能的顺序迭代器方法的替代版本。



# 2 基础使用

```
use rayon::prelude::*;

pub async fn test_rayon(){
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum : Vec<()>= numbers
        .par_iter()
        .map(|v|{
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
```



```
 num 1 id ThreadId(6) name None
 num 6 id ThreadId(10) name None
 num 7 id ThreadId(2) name None
 num 3 id ThreadId(8) name None
 num 4 id ThreadId(4) name None
 num 2 id ThreadId(11) name None
 num 8 id ThreadId(9) name None
 num 9 id ThreadId(7) name None
 num 5 id ThreadId(5) name None
 num 10 id ThreadId(3) name None
Sum: [(), (), (), (), (), (), (), (), (), ()]

```

结果清晰看到 是多线程执行的

# 3 并行迭代器

- `par_iter()`：将一个集合转换为并行迭代器，以便在并行环境中对其进行操作。

- `par_iter_mut()`：将一个可变集合转换为并行迭代器，以便在并行环境中对其进行修改。

- `par_chunks()`：将一个集合分成块，并在并行环境中对每个块进行操作。

- `par_sort()`：对一个集合进行并行排序。

并行迭代器可以轻松编写并行执行的类似迭代器的链：通常您所要做的就是将第一个`.iter()`(or `iter_mut()`、`into_iter()`等) 方法转换为 `par_iter()`(or `par_iter_mut()`、`into_par_iter()`等)。



- `.iter()`: 用于创建一个不可变的迭代器，用于按顺序访问集合中的每个元素。返回的迭代器类型是`Iterator<Item = &T>`，其中`T`是集合中元素的类型。
- `.iter_mut()`: 用于创建一个可变的迭代器，用于按顺序访问集合中的每个元素，以便进行修改。返回的迭代器类型是`Iterator<Item = &mut T>`，其中`T`是集合中元素的类型。
- `.into_iter()`: 用于创建一个拥有所有权的迭代器，用于按顺序访问集合中的每个元素。返回的迭代器类型是`Iterator<Item = T>`，其中`T`是集合中元素的类型。

## 3.1 [ParallelIterator](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#) value并行器

```
pub trait ParallelIterator: Sized + Send {
    type Item: Send;

Show 58 methods
}
```

标准迭代器特征的并行版本。

此特征的组合器可用于**所有**并行迭代器。可以在该特征上找到其他方法 [`IndexedParallelIterator`](https://docs.rs/rayon/latest/rayon/iter/trait.IndexedParallelIterator.html)：这些方法仅适用于预先知道项目数量的并行迭代器（因此，例如，在调用后`filter`，这些方法将变得不可用）。

有关使用并行迭代器的示例，请参阅[模块](https://docs.rs/rayon/latest/rayon/iter/index.html)[上的文档 `iter`](https://docs.rs/rayon/latest/rayon/iter/index.html)。



### 3.1.1 for_each

并行执行`OP`迭代器生成的每个项目。

无 返回值

```
fn for_each<OP>(self, op: OP)
where
    OP: Fn(Self::Item) + Sync + Send,
```



```
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

 num 0 id ThreadId(9) name None
 num 1 id ThreadId(9) name None
 num 93 id ThreadId(11) name None
 num 75 id ThreadId(10) name None
 num 87 id ThreadId(3) name None
 num 50 id ThreadId(7) name None
 num 62 id ThreadId(5) name None
```



### 3.1.2  for_each_with

```
fn for_each_with<OP, T>(self, init: T, op: OP)
where
    OP: Fn(&mut T, Self::Item) + Sync + Send,
    T: Send + Clone,
```

在并行计算中，使用给定的初始值对迭代器产生的每个项目执行操作。

初始值只在需要时进行克隆，以与每个Rayon作业中的项目组配对。它不要求类型是`Sync`（同步的）。

简而言之，这段话的意思是在并行计算中，可以使用给定的初始值和迭代器中的每个项目执行操作。初始值只会在需要时进行克隆，以便与每个并行任务中的项目组配对。这个操作不要求初始值的类



```
//并行计算并放入chan中
async fn for_each_with(){
    use std::sync::mpsc::channel;

    let (sender,receiver) = channel();

    (0..10).
        into_par_iter()
        .for_each_with(sender,|s,x|{
            println!(
                " num {} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            s.send(x).unwrap();
        });

    let mut  res : Vec<_> = receiver.iter().collect();
    println!("init res {:?}",res);
    res.sort();
    println!("sort res {:?}",res);
}

num 0 id ThreadId(8) name None
 num 1 id ThreadId(8) name None
 num 2 id ThreadId(8) name None
 num 3 id ThreadId(8) name None
 num 4 id ThreadId(3) name None
 num 5 id ThreadId(6) name None
 num 9 id ThreadId(3) name None
 num 7 id ThreadId(11) name None
 num 8 id ThreadId(8) name None
 num 6 id ThreadId(9) name None
init res [0, 1, 2, 3, 4, 5, 9, 7, 8, 6]
sort res [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]


```

可以看到数据用到了多线程，`for_eah_with`可以传入sender `OP: Fn(&mut T, Self::Item) + Sync + Send,`签名的类型



### 3.1.3 for_each_init

```
fn for_each_init<OP, INIT, T>(self, init: INIT, op: OP)
where
    OP: Fn(&mut T, Self::Item) + Sync + Send,
    INIT: Fn() -> T + Sync + Send,
```

在并行计算中，使用迭代器产生的每个项目对由初始化函数返回的值执行操作。

初始化函数仅在需要为每个Rayon作业中的项目组配对一个值时才会被调用。对于返回的值类型没有任何约束。

简而言之，这段话的意思是在并行计算中，可以使用由初始化函数返回的值和迭代器中的每个项目执行操作。初始化函数只会在需要为每个并行任务中的项目组配对一个值时进行调用。对于返回的值类型没有任何约束。



```

//可传递一个初始值，变为可变的参与活动
async fn for_each_init(){
    use rand::Rng;
    let mut v = vec![0u8; 1_0];

    v.par_chunks_mut(1000)
        .for_each_init(
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

    println!("v {:?}",v);

// There's a remote chance that this will fail...
  
}

rng ThreadRng { rng: UnsafeCell { .. } },num [0, 0, 0, 0, 0, 0, 0, 0, 0, 0] id ThreadId(1) name Some("main")
v [21, 34, 227, 155, 209, 208, 93, 112, 221, 191]


```

但是可以看到 初始值 和原vec 都是变化的



### 3.1.4 try_for_each

```
fn try_for_each<OP, R>(self, op: OP) -> R
where
    OP: Fn(Self::Item) -> R + Sync + Send,
    R: Try<Output = ()> + Send,
```

在 Rayon 库中，`try_for_each` 函数用于在并行环境中对迭代器中的每个项目执行操作，类似于 `for_each` 函数。不同之处在于，`try_for_each` 允许操作返回一个 `Result` 类型的结果，以处理可能的错误。



```

// for_each 迭代元素并返回结果
async fn try_for_each(){
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

num 2 id ThreadId(3) name None
 num 5 id ThreadId(10) name None
 num 4 id ThreadId(9) name None
 num 1 id ThreadId(5) name None
 num 3 id ThreadId(7) name None
Error occurred: Encountered an error!
```



类似于 golang的`errGroup`，多线程返回错误



### 3.1.5 try_for_each_with

```
fn try_for_each_with<OP, T, R>(self, init: T, op: OP) -> R
where
    OP: Fn(&mut T, Self::Item) -> R + Sync + Send,
    T: Send + Clone,
    R: Try<Output = ()> + Send,
```

`try_for_each_with` 函数允许您在 Rayon 的并行环境中对每个元素执行操作，并将一个可变的累加器传递给操作。该函数返回一个 `Result` 类型，以便您可以处理可能的错误。



```
//传入可变元素并操作
async fn try_for_each_with(){
    let number = vec![1,2,3,4,5];
    use std::sync::mpsc::channel;

    let (sender,receiver) = channel();

   let res =  (0..10).
        into_par_iter()
        .try_for_each_with(sender,|s,x|{
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
    println!("init res {:?}",res);

    let mut  data : Vec<_> = receiver.iter().collect();
    println!("init data {:?}",data);
    data.sort();
    println!("sort data {:?}",data);
}

 num 3 id ThreadId(10) name None
 num 0 id ThreadId(6) name None
 num 6 id ThreadId(3) name None
 num 4 id ThreadId(9) name None
 num 7 id ThreadId(2) name None
 num 5 id ThreadId(8) name None
 num 2 id ThreadId(4) name None
 num 1 id ThreadId(5) name None
init res Err("Encountered an error!")
init data [0, 6, 4, 7, 5, 2, 1]
sort data [0, 1, 2, 4, 5, 6, 7]

```



**案例二**

```
async fn try_for_each_with2(){
    let numbers = vec![1, 2, 3, 4, 5];

    // 定义一个可变的累加器
    let mut sum = 0;

    // 使用 try_for_each_with 在并行环境中对每个数字执行操作
    let result = numbers.par_iter().try_for_each_with(&mut sum, |acc, &num| {
        // 执行操作，这里尝试将数字累加到累加器中
        *acc = num+*acc;

        // 模拟可能的错误，如果数字为3，则返回一个错误
        if num == 3 {
            Err("Encountered an error!")
        } else {
            Ok(())
        }
    });

    // 检查操作是否成功
    match result {
        Ok(()) => println!("All operations completed successfully. Sum: {}", sum),
        Err(err) => println!("Error occurred: {}", err),
    }
}
```



```
 Compiling my_test v0.1.0 (/Users/xxx/Desktop/rust/my_test)
error[E0277]: the trait bound `&mut {integer}: Clone` is not satisfied
   --> src/controller/rayon.rs:16:55
    |
16  |     let result = numbers.par_iter().try_for_each_with(&mut sum, |acc, &num| {
    |                                     ----------------- ^^^^^^^^ the trait `Clone` is not implemented for `&mut {integer}`
    |                                     |
    |                                     required by a bound introduced by this call
    |
note: required by a bound in `rayon::iter::ParallelIterator::try_for_each_with`
   --> /Users/xxx/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/rayon-1.8.0/src/iter/mod.rs:509:19
    |
506 |     fn try_for_each_with<OP, T, R>(self, init: T, op: OP) -> R
    |        ----------------- required by a bound in this associated function
...
509 |         T: Send + Clone,
    |                   ^^^^^ required by this bound in `ParallelIterator::try_for_each_with`
help: consider removing the leading `&`-reference
    |
16  -     let result = numbers.par_iter().try_for_each_with(&mut sum, |acc, &num| {
16  +     let result = numbers.par_iter().try_for_each_with(sum, |acc, &num| {
    |

error[E0277]: cannot add `&mut {integer}` to `{integer}`
  --> src/controller/rayon.rs:18:19
   |
18 |         *acc = num+*acc;
   |                   ^ no implementation for `{integer} + &mut {integer}`
   |
   = help: the trait `Add<&mut {integer}>` is not implemented for `{integer}`
   = help: the following other types implement trait `Add<Rhs>`:
             <isize as Add>
             <isize as Add<num_bigint_dig::bigint::BigInt>>
             <isize as Add<&'a num_bigint_dig::bigint::BigInt>>
             <isize as Add<&isize>>
             <i8 as Add>
             <i8 as Add<num_bigint_dig::bigint::BigInt>>
             <i8 as Add<&'a num_bigint_dig::bigint::BigInt>>
             <i8 as Add<&i8>>
           and 126 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `my_test` (lib) due to 2 previous errors


```



改造后

也可以用arc

```

async fn try_for_each_with2(){
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
        Ok(()) => println!("All operations completed successfully. Sum: {}", *sum.lock().unwrap()),
        Err(err) => println!("Error occurred: {}", err),
    }
    println!("numbers {:?}",numbers);
    println!("numbers {:?}",sum);
}

num 1 id ThreadId(10) name None
 num 3 id ThreadId(8) name None
 num 6 id ThreadId(9) name None
 num 10 id ThreadId(11) name None
 num 15 id ThreadId(10) name None
Error occurred: Encountered an error!
numbers [1, 2, 3, 4, 5]
numbers Mutex { data: 15, poisoned: false, .. }

```



### 3.1.6 try_for_each_init

```
fn try_for_each_init<OP, INIT, T, R>(self, init: INIT, op: OP) -> R
where
    OP: Fn(&mut T, Self::Item) -> R + Sync + Send,
    INIT: Fn() -> T + Sync + Send,
    R: Try<Output = ()> + Send,
```

`try_for_each_init` 函数在 Rayon 中的作用是对一个可迭代对象进行初始化并行操作，并且可以处理可能的错误。



```
async  fn try_for_each_init(){
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

    println!("v {:?}",v);
// There's a remote chance that this will fail...

}
num [0, 0, 0, 0, 0, 0, 0, 0, 0, 0] id ThreadId(1) name Some("main")
v [149, 151, 43, 63, 142, 32, 183, 92, 163, 137]
```



提供的代码示例使用了 Rayon 的 `par_chunks_mut()` 和 `try_for_each_init()` 函数来并行地对一个大向量进行填充操作。

在这个示例中，我们首先创建了一个长度为 1,000,000 的 `u8` 类型的向量 `v`，并将其所有元素初始化为 0。

然后，我们使用 `par_chunks_mut(1000)` 函数将向量分成大小为 1000 的块，并在每个块上执行填充操作。`try_for_each_init()` 函数接受两个闭包作为参数，第一个闭包用于初始化随机数生成器，第二个闭包用于在每个块上填充随机数。这里使用了 `rand::thread_rng()` 初始化随机数生成器，并调用 `try_fill()` 方法来填充块。



### 3.1.7 count

```
fn count(self) -> usize

use rayon::prelude::*;

let count = (0..100).into_par_iter().count();

assert_eq!(count, 100);

```



### 3.1.8 map

```
map<F, R>(self, map_op: F) -> Map<Self, F>
where
    F: Fn(Self::Item) -> R + Sync + Send,
    R: Send,
```

应用于`map_op`此迭代器的每个项目，并使用结果生成一个新的迭代器。

```
//map 迭代器执行 返回新的迭代器
async fn map(){
    let mut par_iter = (0..5)
        .into_par_iter()
        .map(|x|{
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            x *2
        });
    let double : Vec<_> = par_iter.collect();
    println!("double {:?}",double);
}

 num 0 id ThreadId(9) name None
 num 1 id ThreadId(8) name None
 num 3 id ThreadId(11) name None
 num 4 id ThreadId(8) name None
 num 2 id ThreadId(2) name None
double [0, 2, 4, 6, 8]
```





### 3.1.9 map_with

应用于此迭代器的每个项目的`map_op`给定`init`值，并使用结果生成一个新的迭代器。

`init`仅当需要与每个人造丝作业中的项目组配对时才会克隆该值。它不要求类型为`Sync`.

```

async fn map_with(){
    let(sender ,receiver)  = channel();

   let mut res: Vec<_>=  (0..5)
        .into_par_iter()
        .map_with(sender,|s,v|{
            println!(
                " num {:?} id {:?} name {:?}",
                v,
                std::thread::current().id(),
                std::thread::current().name()
            );
            s.send(v).unwrap();
            v
        }).collect();

    println!("init res  {:?}",res);
    res.sort();
    println!("res  {:?}",res);
    let r_rec : Vec<_>= receiver.iter().collect();
    println!("init r_rec  {:?}",r_rec);


}


 num 4 id ThreadId(11) name None
 num 0 id ThreadId(5) name None
 num 3 id ThreadId(3) name None
 num 1 id ThreadId(10) name None
 num 2 id ThreadId(9) name None
init res  [0, 1, 2, 3, 4]
res  [0, 1, 2, 3, 4]
init r_rec  [4, 0, 3, 1, 2]

```



可以看到 res 返回时有序的，但是接收的事无序的



### 3.1.10 map_init



```
fn map_init<F, INIT, T, R>(
    self,
    init: INIT,
    map_op: F
) -> MapInit<Self, INIT, F>
where
    F: Fn(&mut T, Self::Item) -> R + Sync + Send,
    INIT: Fn() -> T + Sync + Send,
    R: Send,
```



```

async fn map_init(){
    let a : Vec<_> = (1i32..10)
        .into_par_iter()
        .map_init(
            || 10,
            |r,x|{
                println!(
                    " num {:?} id {:?} name {:?}",
                    x,
                    std::thread::current().id(),
                    std::thread::current().name()
                );
                *r += x;
                *r
            }
        ).collect();
    println!(" a {:?}",a);

}

 num 1 id ThreadId(2) name None
 num 2 id ThreadId(2) name None
 num 3 id ThreadId(2) name None
 num 4 id ThreadId(2) name None
 num 7 id ThreadId(2) name None
 num 8 id ThreadId(2) name None
 num 9 id ThreadId(2) name None
 num 6 id ThreadId(2) name None
 num 5 id ThreadId(3) name None
 a [11, 12, 13, 14, 15, 16, 17, 18, 19]

```

可以看到初始化的每个值都会加上给定的初始化的r的值，并且顺序返回

签名INIT 可以看到事sync send的



### 3.1.11 cloned

创建一个克隆其所有元素的迭代器。`&T`当您有一个迭代器但您需要`T`并且该类型实现时，这可能很有用`Clone`。另请参阅[`copied()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.copied)。

```
fn cloned<'a, T>(self) -> Cloned<Self>
where
    T: 'a + Clone + Send,
    Self: ParallelIterator<Item = &'a T>,
```

```

async fn cloned(){
    let a = vec![1,2,3];

    let v_cloned : Vec<_> = a.par_iter().cloned().collect();

    // let vp : Vec<_> = a.par_iter().map(|&x|{x}).collect();//x shi i32
    let vp : Vec<_> = a.par_iter().map(|x|{x}).collect();//&x shi &i32

    println!("a {:?}  vp {:?},a address {:p},vp address {:p}",a,vp,&a,&vp);
}
a [1, 2, 3]  vp [1, 2, 3],a address 0x16d1a5970,vp address 0x16d1a59a0

```

重新创建了一份，地址是不同的

### 3.1.12 copied

创建一个复制其所有元素的迭代器。`&T`当您有一个迭代器但您需要`T`并且该类型实现时，这可能很有用`Copy`。另请参阅[`cloned()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.cloned)。

```
fn copied<'a, T>(self) -> Copied<Self>
where
    T: 'a + Copy + Send,
    Self: ParallelIterator<Item = &'a T>,
```

```

async fn copied(){
    let a = vec![1,2,3];

    let v_cloned : Vec<_> = a.par_iter().copied().collect();

    // let vp : Vec<_> = a.par_iter().map(|&x|{x}).collect();//x shi i32
    let vp : Vec<_> = a.par_iter().map(|x|{x}).collect();//&x shi &i32

    println!("a {:?}  vp {:?},a address {:p},vp address {:p}",a,vp,&a,&vp);
}
a [1, 2, 3]  vp [1, 2, 3],a address 0x16dc51970,vp address 0x16dc519a0


```



### 3.1.13 Copied cloned区别

基本类型是copy

到堆上的是cloned

- `copied`：`copied` 是 Rust 标准库中迭代器的方法之一，它适用于实现了 `Copy` trait 的类型。
- `cloned` 是 Rust 标准库中迭代器的方法之一，它适用于实现了 `Clone` trait 的类型



在 Rust 中，几乎所有的类型都可以实现 `Clone` trait。标准库中的许多常见类型都已经实现了 `Clone` trait，包括：

- 所有基本数据类型，如整数、浮点数、布尔类型等。
- 字符串类型（`String`）和字符类型（`char`）。
- 向量类型（`Vec<T>`）和切片类型（`&[T]`）。
- 哈希映射类型（`HashMap<K, V>`）和哈希集合类型（`HashSet<T>`）。
- 数组类型（`[T; N]`）和元组类型（`(T1, T2, ...)`）。
- 枚举类型，只要其中的每个变体都实现了 `Clone` trait。

以下是一些常见的 Copy 类型：

- 所有的基本整数类型（如 `i32`、`u64`）和布尔类型（`bool`）。
- 所有的浮点数类型（如 `f32`、`f64`）。
- 字符类型（`char`）。
- 数组类型（`[T; N]`），其中 `T` 是 Copy 类型，并且数组的长度 `N` 是常量。
- 具有 Copy 类型字段的结构体或元组，只要所有字段都是 Copy 类型。



### 3.1.14 inspect

在 Rust 中，`inspect()` 是 `Iterator` trait 的一个方法，用于在迭代过程中调用一个闭包函数来检查每个元素，==而不改变原始迭代器==。

`inspect()` 方法接受一个闭包函数作为参数，该闭包函数会接收迭代器的每个元素作为参数，并在每次迭代时执行。闭包函数可以用于执行任意操作，例如打印元素、记录日志或进行其他的副作用操作。

```
fn inspect<OP>(self, inspect_op: OP) -> Inspect<Self, OP>
where
    OP: Fn(&Self::Item) + Sync + Send,
```



```

//inspect() 方法接受一个闭包函数作为参数，该闭包函数会接收迭代器的每个元素作为参数，
// 并在每次迭代时执行。闭包函数可以用于执行任意操作，例如打印元素、记录日志或进行其他的副作用操作。
async fn inspect(){
    let a = [1, 4, 2, 3];

// this iterator sequence is complex.
    let sum = a.par_iter()
        .cloned()
        .filter(|&x| x % 2 == 0)
        .reduce(|| 0, |sum, i| sum + i);

    println!("{}", sum);

// let's add some inspect() calls to investigate what's happening
    let sum = a.par_iter()
        .cloned()
        .inspect(|x| println!("about to filter: {}", x))
        .filter(|&x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {}", x))
        .reduce(|| 0, |sum, i| sum + i);

    println!("{}", sum);
}

6
about to filter: 1
about to filter: 4
made it through filter: 4
about to filter: 2
made it through filter: 2
about to filter: 3
6
```



### 3.1.15 update

在产生迭代器之前改变它的每一项。

```
fn update<F>(self, update_op: F) -> Update<Self, F>
where
    F: Fn(&mut Self::Item) + Sync + Send,
```



```

//根据原值进行操作，然后新的操作结果
async fn update(){
    let double : Vec<_> = (0..5)
        .into_par_iter()
        .update(|x|{
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            *x *= 2
        }).collect();

    println!("init double {:?}",double);

}

 num 0 id ThreadId(9) name None
 num 1 id ThreadId(2) name None
 num 2 id ThreadId(7) name None
 num 3 id ThreadId(11) name None
 num 4 id ThreadId(9) name None
init double [0, 2, 4, 6, 8]

```



### 3.1.16 filter

应用于`filter_op`此迭代器的每个项目，生成一个仅包含给出结果的项目的新迭代器`true`。

```
fn filter<P>(self, filter_op: P) -> Filter<Self, P>
where
    P: Fn(&Self::Item) -> bool + Sync + Send,
```



```

async fn filter(){
    // let    mut par_iter : Vec<_> = (0..10)//可变 也可以
    let   par_iter : Vec<_> = (0..10)
        .into_par_iter()
        .filter(|x| x %2 == 0).collect();

    println!(" {:?}",par_iter);
}
[0, 2, 4, 6, 8]
```



### 3.1.17 filter_map

应用于`filter_op`此迭代器的每个项目以获取`Option`，生成一个仅包含结果中的项目的新迭代器`Some`。

```
fn filter_map<P, R>(self, filter_op: P) -> FilterMap<Self, P>
where
    P: Fn(Self::Item) -> Option<R> + Sync + Send,
    R: Send,
```



```

async fn filter_map(){
    let mut par_iter : Vec<_> = (0..10)
        .into_par_iter()
        .filter_map(|x|{
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
            if x %2  == 0{
                Some(x)
            }else {
                None
            }
        }).collect();
    println!("par_iter {:?}",par_iter);

}

num 0 id ThreadId(8) name None
 num 2 id ThreadId(5) name None
 num 3 id ThreadId(5) name None
 num 4 id ThreadId(5) name None
 num 5 id ThreadId(10) name None
 num 6 id ThreadId(5) name None
 num 8 id ThreadId(10) name None
 num 9 id ThreadId(5) name None
 num 7 id ThreadId(3) name None
 num 1 id ThreadId(8) name None
par_iter [0, 2, 4, 6, 8]
```



### 3.1.18 flat_map



```
fn flat_map<F, PI>(self, map_op: F) -> FlatMap<Self, F>
where
    F: Fn(Self::Item) -> PI + Sync + Send,
    PI: IntoParallelIterator,
```



```

async fn flat_map(){
    let words =[[1, 2], [3, 4], [5, 6], [7, 8]];

    let letters : Vec<_> = words
        .par_iter()
        .cloned()
        .flat_map(|word| {
            println!(
                " num {:?} id {:?} name {:?}",
                word,
                std::thread::current().id(),
                std::thread::current().name()
            );
           let t =  word.to_vec();
            println!("t {:?}",t);
            t
        })
        .collect();

    println!("{:?}", words);
    println!("{:?}", letters);
}

 num [1, 2] id ThreadId(10) name None
t [1, 2]
 num [5, 6] id ThreadId(8) name None
 num [7, 8] id ThreadId(3) name None
t [7, 8]
 num [3, 4] id ThreadId(6) name None
t [3, 4]
t [5, 6]
[[1, 2], [3, 4], [5, 6], [7, 8]]
[1, 2, 3, 4, 5, 6, 7, 8]

```



### 3.1.19 flat_map_iter

```
fn flat_map_iter<F, SI>(self, map_op: F) -> FlatMapIter<Self, F>
where
    F: Fn(Self::Item) -> SI + Sync + Send,
    SI: IntoIterator,
    SI::Item: Send,
```

应用于`map_op`此迭代器的每一项以获取嵌套的串行迭代器，生成一个新的并行迭代器，将它们展平为一个。

[`flat_map_iter`相对`flat_map`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#flat_map_iter-versus-flat_map)

这两种方法相似，但行为略有不同。对于[`flat_map`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.flat_map)，每个嵌套迭代器都必须是并行迭代器，并且它们将通过嵌套并行性进一步拆分。对于`flat_map_iter`，每个嵌套迭代器都是一个顺序的`Iterator`，我们只在它们*之间*进行并行化，而每个嵌套迭代器产生的项目是按顺序处理的。

在这些方法之间进行选择时，请考虑嵌套并行性是否适合手头的潜在迭代器。如果涉及的计算很少，或者它的长度比外部并行迭代器小得多，那么它可能会表现得更好，以避免并行性的开销，只需按顺序展平即可`flat_map_iter`。如果有大量计算，可能超过外部并行迭代器，那么嵌套并行性`flat_map`可能是值得的。

```

async fn flat_map_iter(){
    let a = [[1, 2], [3, 4], [5, 6], [7, 8]];
    use std::cell::RefCell;
    let par_iter = a.par_iter().flat_map_iter(|a| {
        // The serial iterator doesn't have to be thread-safe, just its items.
        let cell_iter = RefCell::new(a.iter().cloned());
        std::iter::from_fn(move || cell_iter.borrow_mut().next())
    });

    let vec: Vec<_> = par_iter.collect();
    println!("vec {:?}",vec);
}

vec [1, 2, 3, 4, 5, 6, 7, 8]
```



### 3.1.20 flatten

一种适配器，可将并行迭代器扁平`Item`化为一个大型迭代器。

```
fn flatten(self) -> Flatten<Self>
where
    Self::Item: IntoParallelIterator,
```



```

async fn flatten(){
    let x: Vec<Vec<_>> = vec![vec![1, 2], vec![3, 4]];
    let y: Vec<_> = x.into_par_iter().flatten().collect();
    println!("y {:?}",y)
}
y [1, 2, 3, 4]
```



### 3.1.21 flatten_iter

```
fn flatten_iter(self) -> FlattenIter<Self>
where
    Self::Item: IntoIterator,
    <Self::Item as IntoIterator>::Item: Send,
```

```

async fn flatten_iter(){
    let x: Vec<Vec<_>> = vec![vec![1, 2], vec![3, 4]];
    let iters: Vec<_> = x.into_iter().map(Vec::into_iter).collect();
    let y: Vec<_> = iters.clone().into_par_iter().flatten_iter().collect();
    println!("{:?} {:?}",iters.clone(),y)
}
[IntoIter([1, 2]), IntoIter([3, 4])] [1, 2, 3, 4]
```





### 3.1.22 reduce

使用 将迭代器中的项目减少为一项`op`。参数`identity`应该是一个可以产生“身份”值的闭包，可以根据需要将其插入到序列中以创造并行执行的机会。因此，例如，如果您正在进行求和，那么`identity()`应该生成代表您类型的零的内容（但`sum()`在这种情况下请考虑仅调用）。

```
fn reduce<OP, ID>(self, identity: ID, op: OP) -> Self::Item
where
    OP: Fn(Self::Item, Self::Item) -> Self::Item + Sync + Send,
    ID: Fn() -> Self::Item + Sync + Send,
```



```

//
async fn reduce(){
    let sums = [(0,1),(5,6),(16,2),(8,9)]
        .par_iter()
        .cloned()
        .reduce(
            ||(0,0),
            |a,b|{
                println!(
                    " num {:?} id {:?} name {:?}",
                    b,
                    std::thread::current().id(),
                    std::thread::current().name()
                );
                (a.0+b.0,a.1+b.1)
            }
        );

    println!("sums {:?}",sums);
}
 num (0, 1) id ThreadId(4) name None
 num (16, 2) id ThreadId(9) name None
 num (5, 6) id ThreadId(5) name None
 num (5, 6) id ThreadId(4) name None
 num (8, 9) id ThreadId(10) name None
 num (8, 9) id ThreadId(9) name None
 num (24, 11) id ThreadId(1) name Some("main")
sums (29, 18)
```



### 3.1.23 reduce_with

返回值为Some、None

```
fn reduce_with<OP>(self, op: OP) -> Option<Self::Item>
where
    OP: Fn(Self::Item, Self::Item) -> Self::Item + Sync + Send,
```



使用 将迭代器中的项目减少为一项`op`。如果迭代器为空，`None`则返回；否则， `Some`返回。

这个版本`reduce`很简单，但效率稍低。如果可能，最好调用`reduce()`，这需要一个单位元素。

```

async fn reduce_with(){
    let sums = [(0, 1), (5, 6), (16, 2), (8, 9)]
        .par_iter()        // iterating over &(i32, i32)
        .cloned()          // iterating over (i32, i32)
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
    println!(" {:?}",sums);
}
num (5, 6) id ThreadId(8) name None
 num (8, 9) id ThreadId(3) name None
 num (24, 11) id ThreadId(1) name Some("main")
 (29, 18)
```



### 3.1.24 try_reduce

```
fn try_reduce<T, OP, ID>(self, identity: ID, op: OP) -> Self::Item
where
    OP: Fn(T, T) -> Self::Item + Sync + Send,
    ID: Fn() -> T + Sync + Send,
    Self::Item: Try<Output = T>,
```

在 Rust 的 Rayon 库中，`try_reduce()` 方法用于执行并行的 reduce 操作，其中闭包可以返回 `Result` 类型的结果。



如果找到一个`Result::Err`或项，或者减少到一个，我们将尝试尽快停止处理迭代器中的其余项，并且我们将返回该终止值。否则，我们将返回最终的减少的或 。

```

async fn try_reduce(){
   println!("{:?} {:?}",sum_squares(0..5), Some(0 + 1 + 4 + 9 + 16));

// The sum might overflow
    println!("{:?} ",sum_squares(0..10_000));

// Or the squares might overflow before it even reaches `try_reduce`
    println!("{:?} ",sum_squares(1_000_000..1_000_001));
}
// Compute the sum of squares, being careful about overflow.
fn sum_squares<I: IntoParallelIterator<Item = i32>>(iter: I) -> Option<i32> {
    iter.into_par_iter()
        .map(|i| i.checked_mul(i))            // square each item,
        .try_reduce(|| 0, i32::checked_add)   // and add them up!
}
Some(30) Some(30)
None 
None 

```

首先，我们使用 `iter.into_par_iter()` 将传入的迭代器 `iter` 转换为一个并行迭代器。

然后，我们使用 `map()` 方法对每个元素进行平方操作。在这里，我们使用闭包 `|i| i.checked_mul(i)` 将每个元素平方，并返回一个 `Option<i32>` 类型的结果。

接着，我们使用 `try_reduce()` 方法来执行并行的 reduce 操作。第一个闭包 `|| 0` 定义了 reduce 操作的初始值，这里我们将初始值设置为 0。第二个闭包 `i32::checked_add` 定义了如何将两个元素进行累积操作，这里我们使用 `checked_add` 方法来执行加法操作，并返回一个 `Option<i32>` 类型的结果。

最后，函数返回了 `try_reduce()` 方法的结果，这是一个 `Option<i32>` 类型的值。

通过这个函数，您可以将一个实现了 `IntoParallelIterator<Item = i32>` trait 的迭代器传入，并对每个元素进行平方操作，并将结果累加起来。如果在平方或累加过程中发生了溢出，那么返回的结果将是 `None`。



### 3.1.25 try_reduce_with

```
fn try_reduce_with<T, OP>(self, op: OP) -> Option<Self::Item>
where
    OP: Fn(T, T) -> Self::Item + Sync + Send,
    Self::Item: Try<Output = T>,
```

使用错误将迭代器中的项目减少为一项`op`。

就像[`reduce_with()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.reduce_with)，如果迭代器为空，`None`则返回；否则，`Some`返回。除此之外，它的行为类似于 [`try_reduce()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.try_reduce)处理`Err`/ `None`。

例如，对于`Option`项目，返回值可能是：

- `None`，迭代器为空
- `Some(None)`，我们相遇后停了下来`None`。
- `Some(Some(x))`，整个迭代器减少为`x`.

对于`Result`项目，嵌套更加明显：

- `None`，迭代器为空
- `Some(Err(e))`，我们在遇到错误后停止了`e`。
- `Some(Ok(x))`，整个迭代器减少为`x`.

```
use rayon::prelude::*;

let files = ["/dev/null", "/does/not/exist"];

// Find the biggest file
files.into_par_iter()
    .map(|path| std::fs::metadata(path).map(|m| (path, m.len())))
    .try_reduce_with(|a, b| {
        Ok(if a.1 >= b.1 { a } else { b })
    })
    .expect("Some value, since the iterator is not empty")
    .expect_err("not found");
```



### 3.1.26 fold

```
fn fold<T, ID, F>(self, identity: ID, fold_op: F) -> Fold<Self, ID, F>
where
    F: Fn(T, Self::Item) -> T + Sync + Send,
    ID: Fn() -> T + Sync + Send,
    T: Send,
```

在 Rust 中，`fold()` 方法用于将一个迭代器的元素进行累积操作，并返回最终的结果。

以下是一个使用 `fold()` 方法的示例：

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum = numbers.into_iter().fold(0, |acc, x| acc + x);

    println!("Sum: {}", sum);
}
```

聚合数组元素到某个地方

```

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
s abcde
```



现在`fold`将一次处理一组字符，并且每组只生成一个字符串。我们最终应该得到一些数量较少的字符串，这些字符串大致与您拥有的 CPU 数量成正比（这最终取决于您的处理器的繁忙程度）。请注意，我们仍然需要随后进行归约以将这些字符串组合并为单个字符串。

您可以使用类似的技巧来保存部分结果（例如缓存）或类似的东西。

[将折叠与其他操作结合起来](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#combining-fold-with-other-operations)

如果您想产生单个值，可以组合使用`fold`。`reduce`这大致相当于有效的映射/归约组合：

```
use rayon::prelude::*;

let bytes = 0..22_u8;
let sum = bytes.into_par_iter()
               .fold(|| 0_u32, |a: u32, b: u8| a + (b as u32))
               .sum::<u32>();

assert_eq!(sum, (0..22).sum()); // compare to sequential
```



### 3.1.27 fold_with

给定初始值，进行累加

```
fn fold_with<F, T>(self, init: T, fold_op: F) -> FoldWith<Self, T, F>
where
    F: Fn(T, Self::Item) -> T + Sync + Send,
    T: Send + Clone,
```

应用于此迭代器的每个项目的`fold_op`给定`init`值，最终生成该值以供进一步使用。

这基本上类似于`fold(|| init.clone(), fold_op)`，但它不需要`init`类型为`Sync`，也不需要任何其他形式的添加同步。

```
use rayon::prelude::*;

let bytes = 0..22_u8;
let sum = bytes.into_par_iter()
               .fold_with(0_u32, |a: u32, b: u8| a + (b as u32))
               .sum::<u32>();

assert_eq!(sum, (0..22).sum()); // compare to sequential
```



### 3.1.28 try_fold 累加返回Option

执行容易出错的平行折叠。

这是 for 操作的变体[`fold()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.fold)，可能会因 `Option::None`或 而失败`Result::Err`。第一次此类失败会停止处理本地项目集，而不影响迭代器细分中的其他折叠。

通常，`try_fold()`随后会[`try_reduce()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.try_reduce) 出现最终的减少和全局短路效应。

```
fn try_fold<T, R, ID, F>(
    self,
    identity: ID,
    fold_op: F
) -> TryFold<Self, R, ID, F>
where
    F: Fn(T, Self::Item) -> R + Sync + Send,
    ID: Fn() -> T + Sync + Send,
    R: Try<Output = T> + Send,
```

```
use rayon::prelude::*;

let bytes = 0..22_u8;
let sum = bytes.into_par_iter()
               .try_fold(|| 0_u32, |a: u32, b: u8| a.checked_add(b as u32))
               .try_reduce(|| 0, u32::checked_add);

assert_eq!(sum, Some((0..22).sum())); // compare to sequential
```



### 3.1.29 try_fold_with

使用可克隆值执行易出错的平行折叠`init`。

这结合了`init`的语义[`fold_with()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.fold_with)和 的故障语义[`try_fold()`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.try_fold)。

```
fn try_fold_with<F, T, R>(self, init: T, fold_op: F) -> TryFoldWith<Self, R, F>
where
    F: Fn(T, Self::Item) -> R + Sync + Send,
    R: Try<Output = T> + Send,
    T: Clone + Send,
```

```
use rayon::prelude::*;

let bytes = 0..22_u8;
let sum = bytes.into_par_iter()
               .try_fold_with(0_u32, |a: u32, b: u8| a.checked_add(b as u32))
               .try_reduce(|| 0, u32::checked_add);

assert_eq!(sum, Some((0..22).sum())); // compare to sequential
```



### 3.1.30 sum 累加

```
fn sum<S>(self) -> S
where
    S: Send + Sum<Self::Item> + Sum<S>,

```



```
use rayon::prelude::*;

let a = [1, 5, 7];

let sum: i32 = a.par_iter().sum();

assert_eq!(sum, 13);
```





### 3.1.31 阶乘

```
where
    P: Send + Product<Self::Item> + Product<P>,
```

将迭代器中的所有项目相乘。

```
use rayon::prelude::*;

fn factorial(n: u32) -> u32 {
   (1..n+1).into_par_iter().product()
}

assert_eq!(factorial(0), 1);
assert_eq!(factorial(1), 1);
assert_eq!(factorial(5), 120);
```





### 3.1.32 min 最小

```
fn min(self) -> Option<Self::Item>
where
    Self::Item: Ord,
```

计算迭代器中所有项目的最小值。如果迭代器为空，`None`则返回；否则，`Some(min)` 返回。

```
use rayon::prelude::*;

let a = [45, 74, 32];

assert_eq!(a.par_iter().min(), Some(&32));

let b: [i32; 0] = [];

assert_eq!(b.par_iter().min(), None);
```



### 3.1.33 min_by 指定最小值 

```
fn min_by<F>(self, f: F) -> Option<Self::Item>
where
    F: Sync + Send + Fn(&Self::Item, &Self::Item) -> Ordering,
```

```
use rayon::prelude::*;

let a = [-3_i32, 77, 53, 240, -1];

assert_eq!(a.par_iter().min_by(|x, y| x.cmp(y)), Some(&-3));
```





### 3.1.34 min_by_key 指定key比较

```
fn min_by_key<K, F>(self, f: F) -> Option<Self::Item>
where
    K: Ord + Send,
    F: Sync + Send + Fn(&Self::Item) -> K,

```

计算产生给定函数最小值的项。如果迭代器为空，`None`则返回；否则，`Some(item)`返回。

```
use rayon::prelude::*;

let a = [-3_i32, 34, 2, 5, -10, -3, -23];

assert_eq!(a.par_iter().min_by_key(|x| x.abs()), Some(&2));
```



### 3.1.35 max 最大

```
fn max(self) -> Option<Self::Item>
where
    Self::Item: Ord,
```

计算迭代器中所有项目的最大值。如果迭代器为空，`None`则返回；否则，`Some(max)` 返回。

```
use rayon::prelude::*;

let a = [45, 74, 32];

assert_eq!(a.par_iter().max(), Some(&74));

let b: [i32; 0] = [];

assert_eq!(b.par_iter().max(), None);
```



### 3.1.36 max_by 最大指定值

```
fn max_by<F>(self, f: F) -> Option<Self::Item>
where
    F: Sync + Send + Fn(&Self::Item, &Self::Item) -> Ordering,
```

```
use rayon::prelude::*;

let a = [-3_i32, 77, 53, 240, -1];

assert_eq!(a.par_iter().max_by(|x, y| x.abs().cmp(&y.abs())), Some(&240));
```



### 3.1.37 max_by_key 最大指定key

```
fn max_by_key<K, F>(self, f: F) -> Option<Self::Item>
where
    K: Ord + Send,
    F: Sync + Send + Fn(&Self::Item) -> K,
```

```
use rayon::prelude::*;

let a = [-3_i32, 34, 2, 5, -10, -3, -23];

assert_eq!(a.par_iter().max_by_key(|x| x.abs()), Some(&34));
```



### 3.1.38 chain 合并两个迭代器

```
fn chain<C>(self, chain: C) -> Chain<Self, C::Iter>
where
    C: IntoParallelIterator<Item = Self::Item>,

```



```
use rayon::prelude::*;

let a = [0, 1, 2];
let b = [9, 8, 7];

let par_iter = a.par_iter().chain(b.par_iter());

let chained: Vec<_> = par_iter.cloned().collect();

assert_eq!(&chained[..], &[0, 1, 2, 9, 8, 7]);
```



### 3.1.39 find_any 查找某个值

```
fn find_any<P>(self, predicate: P) -> Option<Self::Item>
where
    P: Fn(&Self::Item) -> bool + Sync + Send,
```

在并行迭代器中搜索与给定谓词匹配的**某些项并返回它。**此操作类似于[`find`顺序迭代器](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)，但返回的项可能不是并行序列中匹配的**第一个**项，因为我们并行搜索整个序列。

一旦找到匹配项，我们将尝试尽快停止处理迭代器中的其余项目（就像`find`找到匹配项后停止迭代一样）。

```
use rayon::prelude::*;

let a = [1, 2, 3, 3];

assert_eq!(a.par_iter().find_any(|&&x| x == 3), Some(&3));

assert_eq!(a.par_iter().find_any(|&&x| x == 100), None);
```



### 3.1.40 find_first 查找第一个

```
fn find_first<P>(self, predicate: P) -> Option<Self::Item>
where
    P: Fn(&Self::Item) -> bool + Sync + Send,
```

在并行迭代器中搜索与给定谓词匹配的顺序**第一项并返回它。**

一旦找到匹配项，该匹配项右侧的所有尝试都将停止

```
use rayon::prelude::*;

let a = [1, 2, 3, 3];

assert_eq!(a.par_iter().find_first(|&&x| x == 3), Some(&3));

assert_eq!(a.par_iter().find_first(|&&x| x == 100), None);
```



### 3.1.41 find_last

```
fn find_last<P>(self, predicate: P) -> Option<Self::Item>
where
    P: Fn(&Self::Item) -> bool + Sync + Send,
```

```
use rayon::prelude::*;

let a = [1, 2, 3, 3];

assert_eq!(a.par_iter().find_last(|&&x| x == 3), Some(&3));

assert_eq!(a.par_iter().find_last(|&&x| x == 100), None);
```

### 3.1.42 find_map_any 找到第一个符合条件的

```
fn find_map_any<P, R>(self, predicate: P) -> Option<R>
where
    P: Fn(Self::Item) -> Option<R> + Sync + Send,
    R: Send,
```

`find_map_any()` 中，我们尝试将字符串解析为数字，并返回第一个成功解析的数字。

```

async  fn find_map_any(){
    let c = ["lol", "NaN", "5", "5"];

    let found_number = c.par_iter().find_map_any(|s| s.parse().ok());
    println!("{:#?}",found_number);

    assert_eq!(found_number, Some(5));
}
Some(
    5,
)

```



### 3.1.43 find_map_first 找到第一个符合条件的

```
fn find_map_first<P, R>(self, predicate: P) -> Option<R>
where
    P: Fn(Self::Item) -> Option<R> + Sync + Send,
    R: Send,
```

将给定谓词应用于并行迭代器中的项，并按顺序返回映射操作的**第一个非 None 结果。**

### 3.1.44 find_map_last 找到最后一个

```
fn find_map_last<P, R>(self, predicate: P) -> Option<R>
where
    P: Fn(Self::Item) -> Option<R> + Sync + Send,
    R: Send,
```

```
use rayon::prelude::*;

let c = ["lol", "NaN", "2", "5"];

let last_number = c.par_iter().find_map_last(|s| s.parse().ok());

assert_eq!(last_number, Some(5));
```



### 3.1.45 any 并行的任何一个

```
fn any<P>(self, predicate: P) -> bool
where
    P: Fn(Self::Item) -> bool + Sync + Send,
```

在并行迭代器中搜索与给定谓词匹配的**某个**项目，如果是则返回 true。一旦找到匹配项，我们将尝试停止处理其余项目。证明不存在匹配，返回 false，确实需要访问每一项。

```

async fn any(){
    let a = [0, 12, 3, 4, 0, 23, 0];

    let is_valid = a.par_iter().any(|&x| {
        println!("{}",x);
        x > 10
    });
    println!("{}",is_valid)
}

0
12
4
3
true

```





### 3.1.46 all 全部都满足

```
fn all<P>(self, predicate: P) -> bool
where
```

测试并行迭代器中的每个项目是否与给定谓词匹配，如果是则返回 true。如果找到反例，我们将尝试停止处理更多项目，然后返回 false。

```

async fn any(){
    let a = [0, 12, 3, 4, 0, 23, 0];

    let is_valid = a.par_iter().all(|&x| {
        println!("{}",x);
        x > 10
    });
    println!("{}",is_valid)
}
0
0
0
23
12
4
false

```



### 3.1.47 while_some 非None就停止

```
fn while_some<T>(self) -> WhileSome<Self>
where
    Self: ParallelIterator<Item = Option<T>>,
    T: Send,
```

在此迭代器的项目上创建一个迭代器`Some`，一旦`None`找到任何项目就停止。

```

async fn while_some(){
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
            if x < 1024 { Some(x) } else { None }
        })
        .while_some()
        .max();

    println!(" value {:?}",value);
    println!(" counter {:?}",counter.load(Ordering::SeqCst));
    assert!(value < Some(1024));
    assert!(counter.load(Ordering::SeqCst) < 2048); // should not have visited every single one
}

num 0 id ThreadId(10) name None
 num 1 id ThreadId(10) name None
 num 2 id ThreadId(10) name None
 num 3 id ThreadId(10) name None
 num 384 id ThreadId(5) name None
 num 1536 id ThreadId(11) name None
 num 1152 id ThreadId(7) name None
 num 4 id ThreadId(10) name None
 num 1024 id ThreadId(6) name None
 num 385 id ThreadId(5) name None
 num 448 id ThreadId(8) name None
 num 512 id ThreadId(2) name None
 num 256 id ThreadId(4) name None
 num 1280 id ThreadId(9) name None
 num 1408 id ThreadId(3) name None
 value Some(384)
 counter 15
```



### 3.1.48 panic_fuse 制造panic 停止所有线程

```
fn panic_fuse(self) -> PanicFuse<Self>
```

在出现紧急情况时用保险丝包装迭代器，以尽快停止所有线程。

并行迭代器中的恐慌总是传播到调用者，但由于[`join`](https://docs.rs/rayon/1.8.0/rayon/fn.join.html#panics). 该适配器会付出更大的努力来更快地停止处理其他项目，但会带来额外的同步开销，这也可能会抑制某些优化。

```

async fn panic_fuse(){
    use rayon::prelude::*;
    use std::{thread, time};

    (0..1_000_000)
        .into_par_iter()
        .panic_fuse()
        .for_each(|i| {
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
 num 0 id ThreadId(9) name None
 num 625000 id ThreadId(11) name None
 num 500000 id ThreadId(7) name None
 num 812500 id ThreadId(10) name None
 num 875000 id ThreadId(6) name None
 num 750000 id ThreadId(5) name None
 num 781250 id ThreadId(3) name None
 num 765625 id ThreadId(4) name None
 num 687500 id ThreadId(2) name None
 num 843750 id ThreadId(8) name None
 num 687501 id ThreadId(2) name None
 num 500001 id ThreadId(7) name None
thread '<unnamed>' panicked at /Users/xxx/Desktop/rust/my_test/src/controller/rayon.rs:25:13:
assertion failed: i > 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```



### 3.1.49 collect 迭代收集器

```
fn collect<C>(self) -> C
where
    C: FromParallelIterator<Self::Item>,
```

创建一个新集合，其中包含此并行迭代器生成的所有元素。

如果您的底层迭代器也实现了它，您可能更喜欢[`collect_into_vec()`](https://docs.rs/rayon/1.8.0/rayon/iter/trait.IndexedParallelIterator.html#method.collect_into_vec)在 上 实现。通过精确了解迭代器包含的元素数量来进行有效分配，甚至允许您重用现有向量的后备存储，而不是分配新向量。[`IndexedParallelIterator`](https://docs.rs/rayon/1.8.0/rayon/iter/trait.IndexedParallelIterator.html)[`collect_into_vec()`](https://docs.rs/rayon/1.8.0/rayon/iter/trait.IndexedParallelIterator.html#method.collect_into_vec)

```
use rayon::prelude::*;

let sync_vec: Vec<_> = (0..100).into_iter().collect();

let async_vec: Vec<_> = (0..100).into_par_iter().collect();

assert_eq!(sync_vec, async_vec);
```

您可以收集一对集合，例如[`unzip`](https://docs.rs/rayon/1.8.0/rayon/iter/trait.ParallelIterator.html#method.unzip) 配对物品：

```
use rayon::prelude::*;

let a = [(0, 1), (1, 2), (2, 3), (3, 4)];
let (first, second): (Vec<_>, Vec<_>) = a.into_par_iter().collect();

assert_eq!(first, [0, 1, 2, 3]);
assert_eq!(second, [1, 2, 3, 4]);
```

或者喜欢[`partition_map`](https://docs.rs/rayon/1.8.0/rayon/iter/trait.ParallelIterator.html#method.partition_map)的`Either`物品：

```
use rayon::prelude::*;
use rayon::iter::Either;

let (left, right): (Vec<_>, Vec<_>) = (0..8).into_par_iter().map(|x| {
    if x % 2 == 0 {
        Either::Left(x * 4)
    } else {
        Either::Right(x * 3)
    }
}).collect();

assert_eq!(left, [0, 8, 16, 24]);
assert_eq!(right, [3, 9, 15, 21]);
```



您甚至可以收集任意嵌套的对 和 的组合`Either`：

```
use rayon::prelude::*;
use rayon::iter::Either;

let (first, (left, right)): (Vec<_>, (Vec<_>, Vec<_>))
    = (0..8).into_par_iter().map(|x| {
        if x % 2 == 0 {
            (x, Either::Left(x * 4))
        } else {
            (-x, Either::Right(x * 3))
        }
    }).collect();

assert_eq!(first, [0, -1, 2, -3, 4, -5, 6, -7]);
assert_eq!(left, [0, 8, 16, 24]);
assert_eq!(right, [3, 9, 15, 21]);
```

所有这些*也*`Result`可以与或类型的短路集合结合使用 `Option`：

```
use rayon::prelude::*;
use rayon::iter::Either;

let result: Result<(Vec<_>, (Vec<_>, Vec<_>)), _>
    = (0..8).into_par_iter().map(|x| {
        if x > 5 {
            Err(x)
        } else if x % 2 == 0 {
            Ok((x, Either::Left(x * 4)))
        } else {
            Ok((-x, Either::Right(x * 3)))
        }
    }).collect();

let error = result.unwrap_err();
assert!(error == 6 || error == 7);

```





### 3.1.50 unzip 多子集合合并

```
fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
where
    Self: ParallelIterator<Item = (A, B)>,
    FromA: Default + Send + ParallelExtend<A>,
    FromB: Default + Send + ParallelExtend<B>,
    A: Send,
    B: Send,
```

将并行迭代器的项解压缩到一对任意 `ParallelExtend`容器中。

您可能更喜欢使用`unzip_into_vecs()`，它可以通过精确了解迭代器包含的元素数量来更有效地进行分配，甚至允许您重用现有向量的后备存储，而不是分配新向量。

```
use rayon::prelude::*;

let a = [(0, 1), (1, 2), (2, 3), (3, 4)];

let (left, right): (Vec<_>, Vec<_>) = a.par_iter().cloned().unzip();

assert_eq!(left, [0, 1, 2, 3]);
assert_eq!(right, [1, 2, 3, 4]);
```

嵌套对也可以解压。

```
use rayon::prelude::*;

let (values, (squares, cubes)): (Vec<_>, (Vec<_>, Vec<_>)) = (0..4).into_par_iter()
    .map(|i| (i, (i * i, i * i * i)))
    .unzip();

assert_eq!(values, [0, 1, 2, 3]);
assert_eq!(squares, [0, 1, 4, 9]);
assert_eq!(cubes, [0, 1, 8, 27]);
```



### 3.1.51 partition 按照条件划分

```
fn partition<A, B, P>(self, predicate: P) -> (A, B)
where
    A: Default + Send + ParallelExtend<Self::Item>,
    B: Default + Send + ParallelExtend<Self::Item>,
    P: Fn(&Self::Item) -> bool + Sync + Send,

```



### 3.1.52 partition_map 

```
fn partition_map<A, B, P, L, R>(self, predicate: P) -> (A, B)
where
    A: Default + Send + ParallelExtend<L>,
    B: Default + Send + ParallelExtend<R>,
    P: Fn(Self::Item) -> Either<L, R> + Sync + Send,
    L: Send,
    R: Send,
```



```
use rayon::prelude::*;

let (left, right): (Vec<_>, Vec<_>) = (0..8).into_par_iter().partition(|x| x % 2 == 0);

assert_eq!(left, [0, 2, 4, 6]);
assert_eq!(right, [1, 3, 5, 7]);
```

将并行迭代器的项分区并映射到一对任意`ParallelExtend`容器中。 `Either::Left`物品进入第一个容器，`Either::Right`物品进入第二个容器。

```
use rayon::prelude::*;
use rayon::iter::Either;

let (left, right): (Vec<_>, Vec<_>) = (0..8).into_par_iter()
    .partition_map(|x| {
        if x % 2 == 0 {
            Either::Left(x * 4)
        } else {
            Either::Right(x * 3)
        }
    });

assert_eq!(left, [0, 8, 16, 24]);
assert_eq!(right, [3, 9, 15, 21]);

```

嵌套`Either`枚举也可以拆分。

```
use rayon::prelude::*;
use rayon::iter::Either::*;

let ((fizzbuzz, fizz), (buzz, other)): ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = (1..20)
    .into_par_iter()
    .partition_map(|x| match (x % 3, x % 5) {
        (0, 0) => Left(Left(x)),
        (0, _) => Left(Right(x)),
        (_, 0) => Right(Left(x)),
        (_, _) => Right(Right(x)),
    });

assert_eq!(fizzbuzz, [15]);
assert_eq!(fizz, [3, 6, 9, 12, 18]);
assert_eq!(buzz, [5, 10]);
assert_eq!(other, [1, 2, 4, 7, 8, 11, 13, 14, 16, 17, 19]);
```



### 3.1.53 intersperse 插入元素

```
fn intersperse(self, element: Self::Item) -> Intersperse<Self>
where
    Self::Item: Clone,
Intersperses clones of an element between items of this iterator.
```



```
use rayon::prelude::*;

let x = vec![1, 2, 3];
let r: Vec<_> = x.into_par_iter().intersperse(-1).collect();

assert_eq!(r, vec![1, -1, 2, -1, 3]);
```



### 3.1.54 take_any 任意去指定个元素

```
fn take_any(self, n: usize) -> TakeAny<Self>

```



```

async fn take_any(){
    use rayon::prelude::*;

    let result: Vec<_> = (0..100)
        .into_par_iter()
        .filter(|&x| x % 2 == 0)
        .take_any(5)
        .collect();

    println!("{}",result.len());
    println!("{:?}",result);

    assert_eq!(result.len(), 5);
    assert!(result.windows(2).all(|w| w[0] < w[1]));
}
5
[0, 50, 52, 62, 76]

5
[0, 2, 4, 50, 52]


```



### 3.1.55 skip_any 跳过指定个元素

```
fn skip_any(self, n: usize) -> SkipAny<Self>
```

```


async fn take_any(){
    use rayon::prelude::*;

    let result: Vec<_> = (0..100)
        .into_par_iter()
        .filter(|&x| x % 2 == 0)
        .skip_any(49)
        .collect();


    println!("{}",result.len());
    println!("{:?}",result);


    assert_eq!(result.len(), 45);
    assert!(result.windows(2).all(|w| w[0] < w[1]));

}
【98】
```



### 3.1.56 take_any_while 任意位置开始到满足条件

```
fn take_any_while<P>(self, predicate: P) -> TakeAnyWhile<Self, P>
where
    P: Fn(&Self::Item) -> bool + Sync + Send,
```

创建一个迭代器，该迭代器从原始迭代器中的*任何位置*获取元素，直到给定的`predicate`returns `false`。

可以`predicate`是任何东西——例如，它可以检查有关该项目的事实、与该项目本身无关的全局条件或其某种组合。

如果并行调用`predicate`比赛并给出不同的结果，则 `true`结果仍将采用这些特定项目，同时尊重`false` 其他地方的结果以跳过任何其他项目。

这类似于[`Iterator::take_while`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)不受原始迭代器顺序的限制。所采取的项目仍将保持其相对顺序，这在`collect`、`reduce`和类似输出中可见。

```


async fn take_any(){
    let result: Vec<_> = (0..100)
        .into_par_iter()
        .take_any_while(|x| {
            println!(
                " num {:?} id {:?} name {:?}",
                x,
                std::thread::current().id(),
                std::thread::current().name()
            );
           if  *x < 50{
            Some(()).is_some()
           }else {
               println!("x > 50 {} x",x);

               None::<bool>.is_some()
           }

        })

        .collect();
println!("{:?}",result);
    assert!(result.len() <= 50);
    assert!(result.windows(2).all(|w| w[0] < w[1]));

}

 num 0 id ThreadId(4) name None
 num 1 id ThreadId(4) name None
 num 2 id ThreadId(4) name None
 num 3 id ThreadId(4) name None
 num 50 id ThreadId(9) name None
x > 50 50 x
 num 25 id ThreadId(10) name None
 num 12 id ThreadId(6) name None
 num 6 id ThreadId(11) name None
 num 4 id ThreadId(4) name None
[0, 1, 2, 3, 4, 6, 12, 25]

```



### 3.1.57 skip_any_while 

```
fn skip_any_while<P>(self, predicate: P) -> SkipAnyWhile<Self, P>
where
    P: Fn(&Self::Item) -> bool + Sync + Send,

```

创建一个迭代器，该迭代器从原始迭代器中的*任何位置*跳过元素，直到给定的`predicate`returns `false`。

可以`predicate`是任何东西——例如，它可以检查有关该项目的事实、与该项目本身无关的全局条件或其某种组合。

如果并行调用`predicate`比赛并给出不同的结果，则 `true`结果仍将跳过这些特定项目，同时尊重`false` 其他地方的结果以跳过任何其他项目。

这类似于[`Iterator::skip_while`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)不受原始迭代器顺序的限制。其余项目仍将保持其相对顺序，这在`collect`、`reduce`和类似输出中可见。

```
use rayon::prelude::*;

let result: Vec<_> = (0..100)
    .into_par_iter()
    .skip_any_while(|x| *x < 50)
    .collect();

assert!(result.len() >= 50);
assert!(result.windows(2).all(|w| w[0] < w[1]));
```



### 3.1.58 opt_len

```
fn opt_len(&self) -> Option<usize>
```



## 3.2 索引并行器



































































































































































