
# 1、类型转换

## 1.1 标量类型转换

标量类型转换用 as

```
fn bl(){
    let x  = 3i32;
    let y : u64 = x as u64;
    println!("x:{} y:{}",x,y);
}

x:3 y:3
```



## 1.2 复杂类型转换

复杂类型转换提供From和Into两个Trait进行转换

From是将其他类型转换为本类型

Into是将本类型转换为其他类型

```

///复杂类型转换
#[derive(Debug)]
struct Animal{
    age : u32
}

#[derive(Debug)]
struct Long{
    age:u32
}

impl From<Animal> for Long {
    fn from(value: Animal) -> Self {
        Long{age:value.age}
    }
}

impl Into<Animal> for  Long{
    fn into(self) -> Animal {
        Animal{age:self.age}
    }
}
fn fz(){
    let loong = Long{age:1000};
    println!("龙：{:?}",loong);
    let long_is_animal:Animal = loong.into();
    println!("long -into->animal：{:?}",long_is_animal);

    let animal = Animal{age:2000};
    let long = Long::from(animal);
    println!("animal -from->long：{:?}",long);
}

龙：Long { age: 1000 }
long -into->animal：Animal { age: 1000 }
animal -from->long：Long { age: 2000 }
```



# 2、特征对象

[2](https://mp.weixin.qq.com/s/-L_WPcghz2ucphNvg14mrQ)

```
// 定义函数，接收一个参数

// 函数体根据参数，决定返回哪个实现了Action的实例

fn do_action_with_arg(label: u8) -> impl Action {

    if label == 1 {

        Tiger::new(3, "OldSix")

    } else {

        Whale::new(60, "Tom")

    }

}
```

上述代码编译错误，原因是if和else块中返回的类型是不相容的，返回impl特征只支持一种特定的类型。

```
// 使用特征对象改进

fn do_action_with_arg(label: u8) -> Box<dyn Action> {

    if label == 1 {

        Box::new(Tiger::new(3, "OldSix"))

    } else {

        Box::new(Whale::new(60, "Tom"))

    }

}
```

==使用智能指针Box包裹特征对象，使得返回值类型大小固定，因为指针的大小是固定的，Rust编译器只支持对大小可知的类型进行编译==





# 3、指针Deref Drop

```

#[derive(Debug)]
struct SandBox<T>(T);

impl <T> SandBox<T>{
    fn new(x:T)->Self{
        Self(x)
    }
}

///实现Deref
impl<T> Deref for SandBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("解引用...");
        &self.0
    }
}

///实现Drop
impl <T> Drop for SandBox<T>{
    fn drop(&mut self) {
        println!("调用drop方法释放内存")
    }
}

fn test_deref_drop(){
    let sandbox = SandBox("Ruster");
    println!("sanbox:{:?}",sandbox);
    // 使用*解引用运算符获取结构体中的值(如果是指针就是获取指针指向的内存中的值)
    // 如果没有实现Deref接口，*sandbox是会报错的
    println!("sandbox value:{:?}", *sandbox);
}

sanbox:SandBox("Ruster")
解引用...
sandbox value:"Ruster"
调用drop方法释放内存

```



## 3.1 智能指针解引用

==智能指针都实现了Deref特征，使用时将自动解引用。下面是Box指针的简单使用示例：==





























































