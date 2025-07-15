#![allow(unused)]

use stackerror::prelude::*;
// https://mp.weixin.qq.com/s/AXzVGFJNdQ9tOp0sruFc-g

pub fn test_stack_error() {
    let res = process_data("error");
    println!("{:?}", res);
}

pub fn process_data(data: &str) -> StackResult<String> {
    let data: Vec<String> = serde_json::from_str(data)
        .map_err(StackError::from_msg)
        .stack_err_msg(fmt_loc!("data is not a list of strings"))?;
    data.first()
        .cloned()
        .ok_or_else(StackError::new)
        .with_err_msg(fmt_loc!("data is empty"))
}
