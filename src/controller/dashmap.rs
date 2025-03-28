// https://mp.weixin.qq.com/s/AY-29Xgfimp34hvu6Jhb6w

use dashmap::DashMap;
use std::collections::hash_map::RandomState;
// use dashmap::try_result::TryResult;

pub fn dash_map_test() {
    let reviews = DashMap::with_capacity(5);
    reviews.insert(2, 4);
    reviews.insert(8, 16);
    println!("{:?}", reviews);

    //使用自己提供的hasher
    println!("使用自己的hasher--------------->");
    let s = RandomState::new();
    let reviews = DashMap::with_hasher(s);
    reviews.insert("Veloren", 15);
    println!("{:?}", reviews); //{"Veloren": 15}

    //获取可变内容
    println!("获取可变内容--------------->");
    *reviews.get_mut("Veloren").unwrap() -= 1;
    println!("{:?}", reviews); //{"Veloren": 14}

    println!("试图获取内容--------------->");
    let map = DashMap::new();
    map.insert("Johnny", 21);

    let vl = *map.try_get("Johnny").unwrap();
    println!("{:?}", vl); //{"Veloren": 14}

    println!("删除值片段--------------->");
    let soccer_team = DashMap::new();
    soccer_team.insert("Jack", "Goalie");
    println!("删除前------{:#?}", soccer_team);

    let remove = soccer_team.remove("Jack").unwrap().1;
    println!("删除的值{:#?}", remove);
    println!("删除后------{:#?}", soccer_team);

    println!("清空map--------------->");
    let stats = DashMap::new();
    stats.insert("Goals", 4);
    println!("清空前------{:#?}", stats);
    let s = stats.is_empty();
    println!("是否是空的------{:#?}", s);
    stats.clear();
    println!("清空后------{:#?}", stats);

    println!("获取map长度--------------->");
    let people = DashMap::new();
    people.insert("Albin", 15);
    people.insert("Jones", 22);
    people.insert("Charlie", 27);
    println!("长度------{:#?}", people.len());

    println!("迭代--------------->");
    let words = DashMap::new();
    words.insert("hello", "world");
    let s = words.iter().count();
    println!("数量------{:#?}", s);
    let map = DashMap::new();
    map.insert("Johnny", 21);
    println!("可变迭代前------{:#?}", map);
    map.iter_mut().for_each(|mut r| *r += 1);
    println!("可变迭代后------{:#?}", map);
}
