#![allow(unused)]

#[derive(Debug)]
struct DefaultStruct {
    pub a: u8,  //1字节
    pub b: u32, // 4字节
    pub c: u16, //2字节
}

// C 布局
// C布局
#[repr(C)]
struct CStruct {
    a: u8,
    b: u32,
    c: u16,
}

// 紧凑布局
#[repr(packed)]
struct PackedStruct {
    a: u8,
    b: u32,
    c: u16,
}

// 透明布局
#[repr(transparent)]
struct TransparentStruct(u64);

// 优化前
struct UnoptimizedStruct {
    small1: u8,
    big1: u64,
    small2: u8,
    big2: u64,
}

// 优化后
#[repr(C)]
struct OptimizedStruct {
    big1: u64,
    big2: u64,
    small1: u8,
    small2: u8,
}

pub fn test_repr() {
    println!(
        "Size of DefaultStruct: {}",
        std::mem::size_of::<DefaultStruct>()
    );
    // 8

    println!("Size of CStruct: {}", std::mem::size_of::<CStruct>());
    println!(
        "Size of PackedStruct: {}",
        std::mem::size_of::<PackedStruct>()
    );
    println!(
        "Size of TransparentStruct: {}",
        std::mem::size_of::<TransparentStruct>()
    );
    //Size of CStruct: 12
    // Size of PackedStruct: 7
    // Size of TransparentStruct: 8

    println!(
        "Unoptimized size: {}",
        std::mem::size_of::<UnoptimizedStruct>()
    );
    println!("Optimized size: {}", std::mem::size_of::<OptimizedStruct>());
    //Unoptimized size: 24
    // Optimized size: 24
}
