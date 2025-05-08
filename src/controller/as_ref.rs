#![allow(unused)]
// https://mp.weixin.qq.com/s/nJznpr7o55u26hmGK7cMeg

pub fn test_as_ref(){

    //多种参数表示
    // test_print()

    //自定义实现
    let person = Person{
        name : String::from("Alice"),
        age:30,
    };

    //使用as_ref获取人名
    let name : &str = person.as_ref();
    println!("Name: {}", name);

    //使用as_ref获取人名的字节表示
    let bytes : &[u8] = person.as_ref();
    println!("Name Bytes: {:?}", bytes);

    // 在需要AsRef<str>的上下文中使用Person
    print_info(person);

    // Name: Alice
    // Name Bytes: [65, 108, 105, 99, 101]
    // Name: Person {
    //     name: "Alice",
    //     age: 30,
    // }

}

// ********************* 自定义类型实现AsRef
#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

//为Person实现AsRef<str>,返回人名
impl AsRef<str> for Person{
    fn as_ref(&self) -> &str{
        &self.name
    }
}

//为Person实现AsRef<[u8]>，返回人名的字节表示
impl AsRef<[u8]> for Person{
    fn as_ref(&self) -> &[u8] {
        self.name.as_bytes()
    }
}

//******************************** 创建接受多种类型的函数
fn print_info<T : AsRef<str>  + std::fmt::Debug>(name : T){
    println!("Name: {:#?}", name);
}




fn test_print(){
    //可以传递&str
    print_info("Alice");

    //可以传递String
    print_info("Bob".to_string());

    //甚至可以传递&String
    let name = "Carol".to_string();
    print_info(&name);

    //可以传递String的切片
    let name = String::from("Alice");
    print_info(&name[0..5]);

    // Name: "Alice"
    // Name: "Bob"
    // Name: "Carol"
    // Name: "Alice"
}