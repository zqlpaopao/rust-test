use std::borrow::Cow;

fn work_with_string(data: Cow<str>, i: i64) -> Cow<str> {
    if i.ge(&0) {
        // 这里我们没有修改字符串，所以可以直接返回
        data
    } else {
        // 这里我们需要修改字符串，所以我们将借用转换为拥有所有权的数据
        Cow::Owned(data.to_string().replace("owned", "bar"))
    }
}

pub fn test_cow() {
    let borrowed_str: &str = "This is a borrowed string";
    let owned_str: String = "This is an owned string".to_string();

    // 使用借用的字符串创建 Cow
    let cow_borrowed = work_with_string(Cow::Borrowed(borrowed_str), 0);
    println!("str cow_borrowed {}", cow_borrowed);

    // 使用拥有所有权的字符串创建 Cow
    let cow_owned = work_with_string(Cow::Owned(owned_str), 1);
    println!("String cow_owned {}", cow_owned);

    //处理切片
    let borrowed_slice: &[i32] = &[1, 2, 3];
    let owned_vec: Vec<i32> = vec![4, 5, 6];

    let cow_borrowed = process_slice(Cow::Borrowed(borrowed_slice));
    let cow_owned = process_slice(Cow::Owned(owned_vec));

    println!("&[i32] cow_borrowed {:?}", cow_borrowed);
    println!("Vec<i32> cow_owned {:?}", cow_owned);

    //在结构体中使用 Cow
    let local_string = "local string".to_string();

    let config_borrowed = Config::new("borrowed string");
    let config_owned = Config::new(local_string);

    println!("Config borrowed: {}", config_borrowed.get_data());
    println!("Config owned: {}", config_owned.get_data());

    // 在函数间传递部分拥有的数据
    let borrowed_data = "Borrowed data";
    needs_owned_data(Cow::Borrowed(borrowed_data));
    println!("borrowed_data {}", borrowed_data);
}

/************************************* 处理切片 **********************************/
fn process_slice(data: Cow<[i32]>) -> Cow<[i32]> {
    // 对切片进行某些处理
    if let Cow::Borrowed(b) = data {
        // 如果是借用的数据，转换为拥有所有权的数据进行修改
        Cow::Owned(b.iter().map(|&x| x * 2).collect())
    } else {
        // 如果已经拥有所有权，直接返回
        data
    }
}

/************************************* 结构体中使用 **********************************/

struct Config<'a> {
    // 可以包含借用的数据或拥有所有权的数据
    data: Cow<'a, str>,
}

impl<'a> Config<'a> {
    fn new<T: Into<Cow<'a, str>>>(data: T) -> Self {
        Config { data: data.into() }
    }

    fn get_data(&self) -> &str {
        &self.data
    }
}

/************************************* 使用 Cow 在函数间传递部分拥有的数据 **********************************/
fn needs_owned_data(data: Cow<str>) {
    // 这个函数可能会根据情况修改数据
    // 如果它是借用的，那么这里就会发生复制
    let mut owned_data = data.into_owned();
    owned_data.push_str(" and more");
    println!("owned_data {}", owned_data);
    // ...
}
