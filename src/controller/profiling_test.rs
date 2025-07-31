
#![allow(unused)]
use profiling::*;


//https://mp.weixin.qq.com/s/anUfFmlX3fVjyTUynJZ_dA

// 为单个函数添加性能追踪，追踪名称为函数名 "process_data"
#[profiling::function]
pub fn process_data_test(input: &str) ->usize {
    // 模拟数据处理（耗时操作）
    let mut result = 0;
    for c in input.chars() {
        if c.is_alphanumeric() {
            result += 1;
        }
    }
    result
}


pub fn test_profiling() {

}

