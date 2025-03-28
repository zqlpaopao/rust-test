#![allow(dead_code)]
use funty::Signed;
pub fn test_vec() {
    let mut bun1: Vec<i32> = Vec::with_capacity(10);
    println!(
        "扩容前内存地址的{:p},容量{},数组长度{}",
        &(*bun1),
        bun1.capacity(),
        bun1.len()
    );
    //批量添加数据
    bun1.extend([
        1, 2, 5, 33, 99, 88, 77, 66, 55, 44, 33, 22, 11, 12, 3, 4, 5, 6, 7, 8, 4, 5,
    ]);
    //扩容到20
    //打印扩容后的地址
    println!(
        "扩容前内存地址的{:p},容量{},数组长度{}",
        &(*bun1),
        bun1.capacity(),
        bun1.len()
    );

    let mut vec = vec![1, 2, 3];
    vec.reserve(10);
    println!("Capacity after reserve: {}", vec.capacity());

    let mut vec = vec![1, 2, 3];
    vec.reserve_exact(10);
    println!("Capacity after reserve_exact: {}", vec.capacity());

    let mut vec = vec![1, 2, 3];
    vec.reserve(10); // 增加容量
    println!("Capacity before shrink: {}", vec.capacity());
    vec.shrink_to_fit();
    println!("Capacity after shrink: {}", vec.capacity());

    let mut vec = vec![1, 2, 3, 4, 5];
    vec.reserve(10); // 增加容量
    println!("Capacity before shrink: {}", vec.capacity());
    vec.shrink_to(3); // 缩小容量
    println!("Capacity after shrink: {}", vec.capacity());
    println!("Capacity after shrink: vec{:?}", vec);

    let vec = vec![1, 2, 3, 4, 5];

    // 使用 into_boxed_slice()
    let boxed_slice = vec.into_boxed_slice();
    println!("Boxed slice: {:?}", boxed_slice);

    // 使用 truncate()
    let mut vec2 = vec![1, 2, 3, 4, 5];
    vec2.truncate(3);
    println!("Truncated vector: {:?}", vec2);

    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 as_slice()
    let slice = vec.as_slice();
    println!("Slice: {:?}", slice);

    // 使用 as_mut_slice()
    let slice_mut = vec.as_mut_slice();
    slice_mut[0] = 10;
    println!("Mutated slice: {:?}", slice_mut);

    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 as_ptr()
    let ptr = vec.as_ptr();
    println!("Raw pointer: {:p}", ptr);

    // 使用 as_mut_ptr()
    let ptr_mut = vec.as_mut_ptr();
    unsafe {
        *ptr_mut = 10;
    }
    println!("Modified vector: {:?}", vec);

    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 swap_remove()
    let removed_element = vec.swap_remove(2);
    println!("Removed element: {}", removed_element);
    println!("Modified vector: {:?}", vec);

    // 使用 remove()
    let removed_element = vec.remove(1);
    println!("Removed element: {}", removed_element);
    println!("Modified vector: {:?}", vec);

    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 insert()
    vec.insert(2, 10);
    println!("Modified vector: {:?}", vec);

    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 retain()
    vec.retain(|&x| x % 2 == 0);
    println!("After retain(): {:?}", vec);

    // 使用 retain_mut()
    vec.retain_mut(|x| {
        *x *= 2;
        *x != 8
    });
    println!("After retain_mut(): {:?}", vec);

    let mut vec1 = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let mut vec2 = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Alice".to_string(),
            age: 31,
        },
    ];

    // 使用 dedup()
    vec1.dedup();
    println!("After dedup(): {:?}", vec1);

    // 使用 dedup_by_key()
    vec2.dedup_by_key(|p| p.name.clone());
    println!("After dedup_by_key(): {:?}", vec2);

    let mut vec = vec![1, 2, 2, 3, 3, 3, 2, 4, 4, 8, 34, 4];

    // 使用 dedup_by()
    vec.dedup_by(|a, b| a.abs() == b.abs());
    println!("After dedup_by(): {:?}", vec);

    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 pop()
    let popped = vec.pop();
    println!("After pop(): {:?}, vec: {:?}", popped, vec);

    // // 使用 pop_if()
    // let popped = vec.pop_if(|x| x % 2 == 0);
    // println!("After pop_if(): {:?}, vec: {:?}", popped, vec);
    //
    // // 再次尝试使用 pop_if()
    // let popped = vec.pop_if(|x| x > 5);
    // println!("After pop_if(): {:?}, vec: {:?}", popped, vec);

    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];

    // 使用 append()
    vec1.append(&mut vec2);
    println!("After append(): vec1: {:?}, vec2: {:?}", vec1, vec2);

    let _arr = [7, 8, 9];

    // 使用 extend_elements()
    // vec1.extend_elements(arr.iter());
    // println!("After extend_elements(): vec1: {:?}", vec1);

    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 drain()
    // let drained = vec.drain(1..3);
    // println!("After drain(): vec: {:?}, drained: {:?}", vec, drained);

    // 使用 clear()
    vec.clear();
    println!("After clear(): vec: {:?}", vec);

    // 使用 retain()
    let mut vec = vec![1, 2, 3, 4, 5];
    let removed_count = vec.retain(|&x| x % 2 == 0);
    println!(
        "After retain(): vec: {:?}, removed_count: {:?}",
        vec, removed_count
    );

    let mut vec = vec![1, 2, 3];

    // 使用 split_off()
    let split_vec = vec.split_off(1);
    println!(
        "After split_off(): vec: {:?}, split_vec: {:?}",
        vec, split_vec
    );

    // 使用 resize_with()
    vec.resize_with(5, || 0);
    println!("After resize_with(): vec: {:?}", vec);

    // 使用 leak()
    let leaked_element = vec.clone().leak();
    println!(
        "After leak(): leaked_element: {:?}, vec: {:?}",
        leaked_element,
        vec.clone()
    ); // vec is now invalid

    // 使用 resize()
    let mut vec = vec![1, 2, 3];
    vec.resize(5, 0);
    println!("After resize(): vec: {:?}", vec);

    let mut vec = Vec::with_capacity(10);
    vec.push(1);
    vec.push(2);

    // 使用 spare_capacity_mut()
    // let spare_capacity = vec.spare_capacity_mut();
    // println!("Before split_at_spare_mut(): spare_capacity = {}", spare_capacity);

    // // 使用 split_at_spare_mut()
    // let (split_vec, remaining_spare_capacity) = vec.split_at_spare_mut();
    // println!("After split_at_spare_mut(): split_vec = {:?}, remaining_spare_capacity = {}", split_vec, remaining_spare_capacity);
}
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

// //初始化内存为10的Vec数组
//     //动态增加元素如果容量不足，会触发扩容，重新申请一块内存2倍大小的内存
//     //再将所有元素拷贝到新的内存，同时更新vec在栈中的内存地址
//     //频繁的扩容会降低性能，因此预定义一个合适的大小的内存很重要
//     let mut bun1: Vec<i32> = Vec::with_capacity(10);
//     println!(
//         "扩容前内存地址的{:p},容量{},数组长度{}",
//         &(*bun1),
//         bun1.capacity(),
//         bun1.len()
//     );
//
//     //批量添加数据
//     bun1.extend([
//         1, 2, 5, 33, 99, 88, 77, 66, 55, 44, 33, 22, 11, 12, 3, 4, 5, 6, 7, 8, 4, 5,
//     ]);
//     //扩容到20
//     bun1.reserve(30);
//     //打印扩容后的地址
//     println!(
//         "扩容前内存地址的{:p},容量{},数组长度{}",
//         &(*bun1),
//         bun1.capacity(),
//         bun1.len()
//     );
//
//     //释放剩余的容量
//     bun1.shrink_to_fit();
//     println!(
//         "扩容前内存地址的{:p},容量{},数组长度{}",
//         &(*bun1),
//         bun1.capacity(),
//         bun1.len()
//     );
//     //扩容前内存地址的0x600000f3c120,容量10,数组长度0
//     // 扩容前内存地址的0x600003a38000,容量52,数组长度22
//     // 扩容前内存地址的0x60000253c000,容量22,数组长度22
//
//     //断言是否为空
//     assert!(!bun1.is_empty());
//
//     //指定索引加入
//     bun1.insert(1, 999999);
//     assert_eq!(999999, *bun1.get(1).unwrap());
//
//     //删除指定位置的数据
//     bun1.remove(1);
//     //删除并范围尾部的数据
//     let tail_num = bun1.pop();
//     println!("tail num{:?}", tail_num);
//
//     //筛选满足条件的数据，并删除不满足的数据
//     bun1.retain_mut(|x| *x > 10);
//     println!("删除<10的数据后{:?}", bun1);
//
//     //删除指定范围的数据，返回删除后的数据的迭代器
//     let del_eles: Vec<i32> = bun1.drain(0..2).collect();
//     println!("被删除的元素:{:?}", del_eles);
//
//     //支持切片,用切片获取连续的数据
//     let slice: &[i32] = &bun1[0..2];
//     assert_eq!(slice, &[88, 77]);
//     //清空
//     bun1.clear();
//     println!(
//         "清空后内存地址的{:p},容量{},数组长度{}",
//         &(*bun1),
//         bun1.capacity(),
//         bun1.len()
//     );
