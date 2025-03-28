#![allow(unused)]
#![feature(test)]

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}

// 可以看出，benchmark 跟单元测试区别不大，最大的区别在于它是通过 #[bench]
// 标注，而单元测试是通过 #[test] 进行标注，这意味着 cargo test 将不会运行 benchmark 代码：
// cargo bench
