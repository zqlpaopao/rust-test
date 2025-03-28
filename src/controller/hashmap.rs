#![allow(unused)]
#[warn(non_snake_case)]
use std::collections::HashMap;

pub fn test_hashmap() {
    hash_set()
}

fn hash_set() {
    use std::collections::{HashMap, HashSet};

    fn main() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);

        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        println!("Set: {:?}", set);
        println!("Map: {:?}", map);

        set.remove(&2);
        map.remove("key2");

        println!("Set after removing 2: {:?}", set);
        println!("Map after removing key2: {:?}", map);

        println!("Set contains 1: {}", set.contains(&1));
        println!("Map contains key1: {}", map.contains_key("key1"));
    }
}

fn remove_entry() {
    let mut map = HashMap::new();

    map.insert("key1", "value1");
    map.insert("key2", "value2");

    if let Some((key, value)) = map.remove_entry("key1") {
        println!("Removed: {} = {}", key, value);
    } else {
        println!("Key not found");
    }

    println!("{:?}", map);
}

fn get_key_value() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    let res = map.get_key_value(&1);
    println!("res {:?}", res);
    let res = map.get_key_value(&2);
    println!("res {:?}", res);
}

fn shrink_to() {
    let mut map = HashMap::with_capacity(300);

    for i in 0..100 {
        map.insert(i, i * 2);
    }

    println!("Before shrink: capacity = {} ", map.capacity());

    map.shrink_to(50);

    println!("Before shrink: capacity = {} ", map.capacity());
}

fn shrink_to_fit() {
    let mut map = HashMap::with_capacity(200);

    for i in 0..100 {
        map.insert(i, i * 2);
    }

    println!("Before shrink: capacity = {}", map.capacity());

    map.remove(&99);
    map.shrink_to_fit();

    println!("After shrink: capacity = {}", map.capacity());
}

fn drain() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let drained: Vec<(i32, &str)> = map.drain().collect();

    for (key, value) in drained {
        println!("Key: {}, Value: {}", key, value);
    }
    println!("{:?}", map);
}

fn value_mut() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for val in map.values_mut() {
        *val = *val + 10;
    }

    for val in map.values() {
        println!("{val}");
    }
    println!("{:?}", map)
}

fn into_key() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    let mut vec: Vec<&str> = map.into_keys().collect();
    // The `IntoKeys` iterator produces keys in arbitrary order, so the
    // keys must be sorted to test them against a sorted array.
    vec.sort_unstable();
    println!("{:?}", vec);
    // println!("{:?}",map);
    //let mut vec: Vec<&str> = map.into_keys().collect();
    //     |                                  ----------- `map` moved due to this method call
    // ...
    // 20  |    println!("{:?}",map);
    //     |                    ^^^ value borrowed here after move
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn t1() {
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{viking:?} has {health} hp");
    }
}

//乳沟不存在插入新值，存在则在原值基础上操作
fn and_modify() {
    let mut player_stats = HashMap::new();

    player_stats.insert("health", 100);

    player_stats
        .entry("mana")
        .and_modify(|mana| *mana += 200)
        .or_insert(100);

    println!("{:?}", player_stats);

    player_stats
        .entry("health")
        .and_modify(|mana| *mana += 200)
        .or_insert(100);

    println!("{:?}", player_stats);
}

fn entry_or_insert_with() {
    let mut map = HashMap::new();

    map.insert("key1", "value1");

    map.entry("key2").or_insert_with(|| "default_value");
    println!("{:?}", map);

    map.entry("key2").or_insert_with(|| "default_value1");
    println!("{:?}", map);
}

//它允许你根据指定的条件保留或移除HashMap中的键值对
fn retain() {
    let mut map = HashMap::new();

    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);

    println!("Before retain: {:?}", map);

    map.retain(|key, value| *value > 1);

    println!("After retain: {:?}", map);
}

// 根据key 移除一个val
fn remove() {
    let mut map = HashMap::new();

    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    println!("Before removal: {:?}", map);

    map.remove("key2");

    println!("After removal: {:?}", map);
}

// 获取数据，如果不存在，则插入
fn entry_or_insert() {
    let mut h = HashMap::with_capacity(5);
    h.insert("a", "a");
    h.insert("b", "b");
    h.insert("c", "c");
    h.insert("d", "d");

    println!("init hashmap {:?}", h);

    h.entry("f").or_insert("f");
    println!("entry or_insert hashmap {:?} ", h);

    h.entry("a").or_insert("g");
    println!("entry or_insert hashmap {:?}", h);
}
