#![allow(unused)]
/// https://mp.weixin.qq.com/s/_fGsQxXL4mia79oOc-26Gg

pub fn test_range() {
    //基础遍历 移动、引用、可变引用
    // basic_iteration()

    //while let
    // while_let_iteration()

    //iteration map filter collect
    // enumerate() 方法也返回一个迭代器，但它不仅提供了每个元素的值，还提供了该元素在集合中的索引。
    // iteration_transformations()

    //reduce 第一个值为基准累加
    //fold 提供初始值进行累加
    // accumulation_operations()

    // 自定义遍历
    // custom_iterator_usage()

    //并行遍历 rayon
    // parallel_iteration()

    //带步长的遍历
    // step_iteration()

    //多重遍历 多维数据 flat_map
    // flat_map 的强大之处在于它允许你在处理中间结果时保持逻辑的简洁和高效，
    // 特别是在处理需要展平的嵌套数据结构时。
    // multiple_iterations()

    //零成本抽象
    // zero_cost_abstractions()

    //内存优化遍历
    // 将大数据 分块处理 large_vec.chunks
    // memory_efficient_iteration()

    //组合子模式
    // combinator_patterns()

    //错误处理
    // let err = error_handling_iteration();
    // println!("{:?}", err);

    //预分配空间
    performance_optimizations()
}
/************************* 预分配空间  ******************************/

fn performance_optimizations() {
    let source = vec![1, 2, 3, 4, 5];

    // 预分配空间
    let mut result = Vec::with_capacity(source.len());
    for &num in &source {
        if num % 2 == 0 {
            result.push(num * 2);
        }
    }

    // 使用Iterator::size_hint
    let (min_size, max_size) = source.iter().filter(|&&x| x % 2 == 0).size_hint();
}
/************************* 错误处理  ******************************/
use std::fs::File;
use std::io::{self, BufRead};

fn error_handling_iteration() -> io::Result<()> {
    let file = File::open("log.log")?;
    let reader = io::BufReader::new(file);

    // 处理每行可能的错误
    // for line in reader.lines() {
    //     let line = line?;
    //     println!("{}", line);
    // }

    // 使用filter_map处理错误
    // filter_map 是 Rust 中的一个迭代器适配器，
    // 它结合了 filter 和 map 的功能。具体来说，它允许你对迭代器中的每个元素进行转换，
    // 并同时过滤掉那些不符合条件的元素。filter_map 接受一个闭包，
    // 闭包返回一个 Option<T>。如果返回 Some(value)，value 会被包含在结果中；
    // 如果返回 None，该元素会被过滤掉。
    let numbers: Vec<i32> = reader
        .lines()
        .filter_map(|line| line.ok()?.parse().ok())
        .collect();

    Ok(())
}
/************************* 内存优化遍历  ******************************/
fn combinator_patterns() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 链式组合子
    let result = numbers
        .iter()
        .take_while(|&&x| x < 4)
        .map(|&x| x * x)
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<_>>();
    println!("{:?}", result);

    // 使用cycle无限循环
    // numbers.iter().cycle(): cycle() 产生的序列从 [1, 2, 3, 1, 2, 3, ...] 变为 [3, 1, 2, 3, 1, 2, ...]。
    //.zip():
    //
    // 将两个迭代器合并为一个迭代器，产生的元素是一个元组，包含来自每个迭代器的一个元素。
    // 合并的迭代器长度由较短的迭代器决定。在这个例子中，由于 numbers.iter() 只有 3 个元素，zip 只会产生 3 个元组。
    for (num, cycle_num) in numbers.iter().zip(numbers.iter().cycle().skip(2)) {
        println!("({}, {})", num, cycle_num);
    }
    // [4]
    // (1, 3)
    // (2, 4)
    // (3, 5)
    // (4, 1)
    // (5, 2)
}

/************************* 内存优化遍历  ******************************/
fn memory_efficient_iteration() {
    let large_vec: Vec<String> = vec![String::from("hello"); 1000000];

    // 避免克隆，使用引用遍历
    for item in &large_vec {
        // 处理item
    }

    // 使用chunks进行批处理
    for chunk in large_vec.chunks(1000) {
        println!("chunk: {:?}", chunk.len());
        // 批量处理
    }
}

/************************* 零成本抽象  ******************************/
fn zero_cost_abstractions() {
    let numbers = vec![1, 2, 3];

    //这些遍历方式在编译后有相同的性能
    let sum1: i32 = numbers.iter().sum();

    let mut sum2 = 0;
    for &num in &numbers {
        sum2 += num
    }

    println!("The sum of all numbers is {},{}", sum1, sum2);
    // The sum of all numbers is 6,6
}

/************************* 多重遍历 多维数据  ******************************/
// flat_map 是一个非常有用的迭代器适配器，它可以将每个元素映射到一个迭代器，
// 然后将这些迭代器的元素扁平化为一个单一的迭代器。它常用于需要将嵌套的迭代器结构展平的场景。
fn multiple_iterations() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    //嵌套遍历
    for row in &matrix {
        for &cell in row {
            print!("cell: {} ", cell);
        }
        println!();
    }

    //使用flat_map扁平化遍历
    let flattened: Vec<i32> = matrix.iter().flat_map(|row| row.iter().cloned()).collect();
    println!("flattened: {:?}", flattened);

    // cell: 1 cell: 2 cell: 3
    // cell: 4 cell: 5 cell: 6
    // cell: 7 cell: 8 cell: 9
    // flattened: [1, 2, 3, 4, 5, 6, 7, 8, 9]
}

/************************* 带步长的遍历  ******************************/
fn step_iteration() {
    //使用step_by
    for i in (1..10).step_by(2) {
        println!("{}", i);
    }

    //自定义步长迭代器
    let numbers: Vec<i32> = (1..10).step_by(5).collect();
    println!("{:?}", numbers);
    // 1
    // 3
    // 5
    // 7
    // 9
    // [1, 6]
}

/************************* 并行遍历  ******************************/
use rayon::prelude::*;

fn parallel_iteration() {
    let numbers: Vec<i32> = (0..1000).collect();

    // 并行遍历
    numbers
        .par_iter()
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .for_each(|x| println!("{}", x));

    // 并行求和
    let sum: i32 = numbers.par_iter().sum();
}

/************************* 3、自定义遍历  ******************************/
struct Counter {
    count: i32,
    max: i32,
}

impl Counter {
    fn new(max: i32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn custom_iterator_usage() {
    let counter = Counter::new(5);
    for num in counter {
        println!("{}", num);
    }
    // 1
    // 2
    // 3
    // 4
    // 5
}

/************************* Iterator  ******************************/
// fold reduce
//reduce
// 用法: reduce方法用于将迭代器中的元素通过一个闭包函数逐步合并为单一的值。
// 初始值: reduce没有显式的初始值。它会使用迭代器的第一个元素作为初始值，并从第二个元素开始应用闭包。
// 返回类型: reduce返回一个Option，即如果迭代器为空，它会返回None，否则返回Some值。

// 用法: fold方法类似于reduce，但它需要显式地提供一个初始值。
// 初始值: fold的第一个参数是初始值，这个值用于开始累积操作。
// 返回类型: fold总是返回累积的结果，因为它总是有一个初始值。
fn accumulation_operations() {
    let numbers = vec![1, 2, 3];

    //fold
    let sum = numbers.iter().fold(0, |acc, num| acc + num);
    println!("Number sum is {}", sum);

    //reduce
    // map(|num: &i32| num): 这个闭包接收一个对i32的引用，并返回这个引用本身。
    // 实际上，这个操作并没有改变数据，它只是将每个元素的引用传递给下一个迭代器阶段。
    // 这个操作有点多余，因为它并没有实际改变数据的类型或值。
    //
    // map(|&num| num): 这个闭包使用模式匹配来解引用传入的引用。&num模式匹配一个引用，
    // 并将其解引用为一个i32值。这样做的结果是，迭代器不再是引用类型，而是实际的值类型。
    // 这样可以在后续操作中直接处理值，而不是引用。
    let product = numbers
        .iter()
        //.map(|num : &i32| num)
        .map(|&num| num)
        .reduce(|acc, num| acc * num)
        .unwrap_or(0);
    println!("Product is {}", product);

    let stats = numbers.iter().fold((0, 0), |sum, count| {
        println!("sum:{:?} count:{}", sum, count);
        (count + 1, count + 1)
    });
    println!("Number stat is {:?}", stats); //分析为射门时4，4
                                            // Number sum is 6
                                            // Product is 6
                                            // sum:(0, 0) count:1
                                            // sum:(2, 2) count:2
                                            // sum:(3, 3) count:3
                                            // Number stat is (4, 4)
}

/************************* Iterator  ******************************/
//map filter
fn iteration_transformations() {
    let numbers = vec![1, 2, 3];

    //链式调用
    let result: Vec<i32> = numbers
        .iter()
        .map(|num| num * 2)
        .filter(|num| num % 3 == 0)
        .collect();
    println!("numbers: {:?}", result);

    //enumerate
    for (index, value) in numbers.iter().enumerate() {
        println!("index is {} and value is {}", index, value);
    }
    // numbers: [6]
    // index is 0 and value is 1
    // index is 1 and value is 2
    // index is 2 and value is 3
}

/************************* while let  ******************************/
fn while_let_iteration() {
    let mut numbers = vec![1, 2, 3, 99];

    //使用while let遍历
    while let Some(value) = numbers.pop() {
        println!("value = {}", value);
    }
    // value = 99
    // value = 3
    // value = 2
    // value = 1
}

/************************* 基础遍历模式  ******************************/
fn basic_iteration() {
    let numbers = vec![1, 2, 3, 4, 5];

    //最基础的for循环
    for n in numbers {
        println!("{}", n);
    }

    //引用遍历
    let numbers = vec![1, 2, 3, 4, 5];

    //for n in numbers.iter() {}
    for n in &numbers {
        println!("引用遍历：{}", n)
    }

    //可引用遍历
    let mut numbers = vec![1, 2, 3, 4, 5];
    for n in &mut numbers {
        *n *= 2;
    }

    println!("{:?}", numbers);

    // 1
    // 2
    // 3
    // 4
    // 5
    // 引用遍历：1
    // 引用遍历：2
    // 引用遍历：3
    // 引用遍历：4
    // 引用遍历：5
    // [2, 4, 6, 8, 10]
}
