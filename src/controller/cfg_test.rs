#![allow(unused)]
pub fn sum(i: i64, j: i64) -> i64 {
    i + j
}

// ***********************************
// 其中，tests 就是一个测试模块，it_works 则是我们的主角：测试函数。
#[cfg(test)]
mod tests {
    #[test]
    //  #[should_panic] 不通过的时候报panic
    // #[should_panic(expected = "Guess value must be less than or equal to 100")] 自定义panic
    fn test_sum() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore] //忽略测试
    fn expensive_test() {
        // 这里的代码需要几十秒甚至几分钟才能完成
    }
}

// 执行cargo test
/*
   running 2 tests
   test controller::cfg_test::tests::test_sum ... ok
   test controller::thread_pool::test::test_thread_pool ... ok

   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        Running unittests src/main.rs (target/debug/deps/my_test-024dc9158f1e11a1)

   running 0 tests

   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

      Doc-tests my_test

   running 0 tests

   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


*/

// 使用 -- 分割命令行参数 cargo test --

// 2 $ cargo test -- --test-threads=1 指定测试的线程数
// 3 指定测试的函数  cargo test test_sum
// 4 我们可以通过指定部分名称的方式来过滤运行相应的测试: cargo test add 还能使用名称中间的一部分：
// 5 通过模块名来进行测试 cargo test tests
