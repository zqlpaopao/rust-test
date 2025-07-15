#![allow(unused)]
use anyhow::Result;

pub trait Cmder {
    fn check(&self) -> Result<()>; // 改为实例方法
    fn doing(&self) -> Result<()>; // 改为实例方法
}

struct Object;

impl Cmder for Object {
    fn check(&self) -> Result<()> {
        println!("Object checking");
        Ok(())
    }

    fn doing(&self) -> Result<()> {
        println!("Object doing");
        Ok(())
    }
}

pub trait CmdAbleTrait {
    fn run(&self, cmd: &mut dyn Cmder) -> Result<()>;
}

// 函数指针类型
type CmdAble = fn(cmd: &mut dyn Cmder) -> Result<()>;

// 为函数指针实现 CmdAbleTrait
impl CmdAbleTrait for CmdAble {
    fn run(&self, cmd: &mut dyn Cmder) -> Result<()> {
        cmd.check()?;
        cmd.doing()?;
        self(cmd)?;
        Ok(())
    }
}

// 示例函数
fn example_cmd(cmd: &mut dyn Cmder) -> Result<()> {
    println!("Running example command");
    Ok(())
}

pub fn test_jc() -> Result<()> {
    let mut obj = Object;

    // 将函数指针转换为 trait 对象
    let cmd: &dyn CmdAbleTrait =
        &(example_cmd as for<'a> fn(&'a mut (dyn Cmder + 'a)) -> Result<(), anyhow::Error>);

    cmd.run(&mut obj)?;

    Ok(())
}
