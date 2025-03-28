# 1 概述



在Rust中，HashMap是一个非常有用的数据结构，它提供了一系列方法来操作和管理键值对。以下是一些常用的HashMap方法的汇总：

1. 创建和初始化HashMap：
   - `new()`：创建一个空的HashMap。
   - `with_capacity(capacity: usize)`：创建一个具有指定初始容量的HashMap。

2. 插入和获取元素：
   - `insert(key, value)`：插入一个键值对到HashMap中。
   - `get(key)`：根据键获取对应的值，返回Option类型。
   - `entry(key).or_insert(value)`：根据键获取对应的值，如果键不存在则插入新的值。

3. 更新和删除元素：
   - `insert(key, value)`：插入一个键值对到HashMap中，如果键已存在，则替换原有的值。
   - `remove(key)`：根据键删除对应的键值对，返回Option类型。
   - `clear()`：清空HashMap中的所有键值对。

4. 查询和遍历元素：
   - `contains_key(key)`：检查HashMap中是否存在指定的键。
   - `len()`：返回HashMap中键值对的数量。
   - `is_empty()`：检查HashMap是否为空。
   - `keys()`：返回一个包含所有键的迭代器。
   - `values()`：返回一个包含所有值的迭代器。
   - `iter()`：返回一个包含所有键值对的迭代器。

5. 其他常用方法：
   - `clone()`：克隆HashMap，创建一个完全相同的副本。
   - `retain(predicate)`：保留满足条件的键值对，移除不满足条件的键值对。
   - `get_mut(key)`：根据键获取对应的可变引用，返回Option类型。
   - `entry(key).or_insert_with(|| value)`：根据键获取对应的值，如果键不存在则通过闭包生成新的值。

这些方法只是HashMap提供的一部分功能，还有其他更高级的用法和方法可以根据具体需求进行探索和使用。你可以查阅Rust官方文档或HashMap的API文档来获取更详细的信息和示例。



## 1.0 初始化带值

```
let solar_distance = HashMap::from([
    ("Mercury", 0.4),
    ("Venus", 0.7),
    ("Earth", 1.0),
    ("Mars", 1.5),
]);
```





## 1.1  `entry(key).or_insert(value)`

根据键获取对应的值，如果键不存在则插入新的值。

```

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


init hashmap {"a": "a", "b": "b", "c": "c", "d": "d"}
entry or_insert hashmap {"a": "a", "b": "b", "c": "c", "d": "d", "f": "f"} 
entry or_insert hashmap {"a": "a", "b": "b", "c": "c", "d": "d", "f": "f"}
```



## 1.2 `remove(key)`

根据键删除对应的键值对，返回Option类型。

```

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

Before removal: {"key3": "value3", "key1": "value1", "key2": "value2"}
After removal: {"key3": "value3", "key1": "value1"}
```



## 1.3 `retain(predicate)`

保留满足条件的键值对，移除不满足条件的键值对。

```

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
Before retain: {"key3": 3, "key1": 1, "key2": 2}
After retain: {"key3": 3, "key2": 2}
```



## 1.4 `entry(key).or_insert_with(|| value)`

根据键获取对应的值，如果键不存在则通过闭包生成新的值,存在不做操作 和`entyr or_insert`区别就是这个是闭包

```

fn entry_or_insert_with() {
    let mut map = HashMap::new();

    map.insert("key1", "value1");

    map.entry("key2").or_insert_with(|| "default_value");
    println!("{:?}", map);

    map.entry("key2").or_insert_with(|| "default_value1");
    println!("{:?}", map);


}
{"key1": "value1", "key2": "default_value"}
{"key1": "value1", "key2": "default_value"}
```



## 1.5 `entry().and_modify().or_insert()`

`entry`方法来获取"mana"键的插入位置，并使用`and_modify`方法来修改已存在的值。如果"mana"键存在，则调用闭包中的逻辑来修改值；如果"mana"键不存在，则调用`or_insert`方法插入一个默认值。

```

//乳沟不存在插入新值，存在则在原值基础上操作
fn and_modify(){
    let mut player_stats = HashMap::new();

    player_stats.insert("health", 100);

    player_stats.entry("mana").and_modify(|mana| *mana += 200).or_insert(100);

    println!("{:?}", player_stats);


    player_stats.entry("health").and_modify(|mana| *mana += 200).or_insert(100);

    println!("{:?}", player_stats);
}

{"health": 100, "mana": 100}
{"health": 300, "mana": 100}
```



## 1.5 结构体为key

```

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}

fn t1(){

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
Viking { name: "Harald", country: "Iceland" } has 12 hp
Viking { name: "Einar", country: "Norway" } has 25 hp
Viking { name: "Olaf", country: "Denmark" } has 24 hp
```



## 1.6 指定 hash函数

创建一个空的`HashMap`，它将使用给定的哈希生成器来哈希键。

```
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

let s = RandomState::new();
let mut map = HashMap::with_hasher(s);
map.insert(1, 2);
```





## 1.7 指定hasher和cap

```
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

let s = RandomState::new();
let mut map = HashMap::with_capacity_and_hasher(10, s);
map.insert(1, 2);
```



## 1.8 `into_keys`

获取map中的所有key的所有权 调用此函数后，地图将无法使用。迭代器元素类型为`K`.

```

fn into_key(){
    let map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);

    let mut vec: Vec<&str> = map.into_keys().collect();
// The `IntoKeys` iterator produces keys in arbitrary order, so the
// keys must be sorted to test them against a sorted array.
    vec.sort_unstable();
   println!("{:?}",vec);
   println!("{:?}",map);
    //let mut vec: Vec<&str> = map.into_keys().collect();
    //     |                                  ----------- `map` moved due to this method call
    // ...
    // 20  |    println!("{:?}",map);
    //     |                    ^^^ value borrowed here after move
}
["a", "b", "c"]
```



## 1.9 `value value_mut`

```
use std::collections::HashMap;

let map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

for val in map.values() {
    println!("{val}");
}


```



```

fn value_mut(){
    let mut map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ]);

    for val in map.values_mut() {
        *val = *val + 10;
    }

    for val in map.values() {
        println!("{val}");
    }
    println!("{:?}",map)
}

12
13
11
{"b": 12, "c": 13, "a": 11}
```



## 1.10 `into_value`

创建一个以任意顺序访问所有值的消耗迭代器。调用此函数后，map将无法使用。迭代器元素类型为`V`.

```
use std::collections::HashMap;

let map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

let mut vec: Vec<i32> = map.into_values().collect();
// The `IntoValues` iterator produces values in arbitrary order, so
// the values must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [1, 2, 3]);

```



## 1.11 `iter iter_mut`

以任意顺序访问所有键值对的迭代器。迭代器元素类型为`(&'a K, &'a V)`.

```
use std::collections::HashMap;

let map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

for (key, val) in map.iter() {
    println!("key: {key} val: {val}");
}
```



迭代器以任意顺序访问所有键值对，并具有对值的可变引用。迭代器元素类型为`(&'a K, &'a mut V)`.

```
use std::collections::HashMap;

let mut map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

// Update all values
for (_, val) in map.iter_mut() {
    *val *= 2;
}

for (key, val) in &map {
    println!("key: {key} val: {val}");
}
```



## 1.12 drain

用于移除并迭代所有的键值对。它会返回一个迭代器，该迭代器会产生被移除的键值对。

```

fn drain(){
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let drained: Vec<(i32, &str)> = map.drain().collect();

    for (key, value) in drained {
        println!("Key: {}, Value: {}", key, value);
    }
    println!("{:?}",map);
}
Key: 1, Value: a
Key: 3, Value: c
Key: 2, Value: b
{}
```



## 1.13 `extract_if`

感谢提供代码示例！这段代码使用了一个名为`hash_extract_if`的实验性特性来实现从`HashMap`中提取满足特定条件的键值对。这个特性目前还没有被合并到Rust标准库中，需要在代码中明确声明使用`#![feature(hash_extract_if)]`来启用。

在这段代码中，首先创建了一个`HashMap`对象`map`，其中包含了0到7的键值对。然后使用`extract_if`方法，传入一个闭包作为参数，该闭包用于指定提取条件。在这个示例中，提取了所有键为偶数的键值对，并将其收集到一个新的`HashMap`对象`extracted`中。

接下来，通过`keys`方法和`copied`方法，分别获取`extracted`和原始`map`中的键，并将它们分别收集到`evens`和`odds`两个`Vec`中。然后对这两个`Vec`进行排序。

最后，使用`assert_eq!`宏来断言提取后的键值对和剩余的键值对是否符合预期。

需要注意的是，`hash_extract_if`特性是实验性的，可能会在未来的Rust版本中发生变化或被移除。因此，在生产环境中使用时需要谨慎考虑。



```
#![feature(hash_extract_if)]
use std::collections::HashMap;

let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
let extracted: HashMap<i32, i32> = map.extract_if(|k, _v| k % 2 == 0).collect();

let mut evens = extracted.keys().copied().collect::<Vec<_>>();
let mut odds = map.keys().copied().collect::<Vec<_>>();
evens.sort();
odds.sort();

assert_eq!(evens, vec![0, 2, 4, 6]);
assert_eq!(odds, vec![1, 3, 5, 7]);
```



## 1.14 reserve 预分配一定空间

分配一定空间

==如果新的分配大小溢出，则会出现恐慌[`usize`](https://doc.rust-lang.org/std/primitive.usize.html)。==

```
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.reserve(100); // 预分配100个空间

    for i in 0..100 {
        map.insert(i, i * 2);
    }

    println!("{:?}", map);
}

```



## 1.15 reserve with_cap区别

在Rust中，`HashMap`提供了两种方法来预分配空间：`reserve`和`with_capacity`。

1. `reserve`方法：
   - `reserve`方法允许你指定预期的键值对数量作为参数，它会预分配足够的空间来容纳这些键值对。
   - `reserve`方法在使用之前，`HashMap`必须已经被创建。
   - 这个方法可以用于在插入大量键值对之前预先分配空间，以避免频繁的重新分配内存。
   - 例如：`map.reserve(100);`

2. `with_capacity`方法：
   - `with_capacity`方法允许你在创建`HashMap`时指定预期的容量大小。
   - `with_capacity`方法会在创建`HashMap`时就分配足够的空间来容纳指定的键值对数量。
   - 这个方法适用于你已经知道要插入多少个键值对，并且想要一次性分配足够的空间。
   - 例如：`let map = HashMap::with_capacity(100);`

这两种方法的区别在于使用的时机和方式：
- 如果你在创建`HashMap`之前就知道要插入多少个键值对，可以使用`with_capacity`方法，在创建时就分配足够的空间。
- 如果你在创建`HashMap`之后才知道要插入多少个键值对，或者想要动态地根据需要预分配空间，可以使用`reserve`方法，在插入之前预分配空间。

需要注意的是，预分配空间并不会限制`HashMap`的容量，它只是为了提前分配内存，以避免频繁的重新分配和复制键值对。

希望这解释清楚了`reserve`和`with_capacity`方法的区别！如果你还有其他问题，请随时提问。



## 1.16 shrink_to_fit 压缩空间

`shrink_to_fit`方法用于缩小`HashMap`的内存占用，使其尽可能接近实际存储的键值对数量所需的最小内存。

使用`shrink_to_fit`方法可以释放`HashMap`中多余的内存空间，但并不保证完全释放所有内存。这是因为`HashMap`需要保持一定的内部结构来支持高效的键值对访问和操作。

```

fn shrink_to_fit(){
    let mut map = HashMap::new();

    for i in 0..100 {
        map.insert(i, i * 2);
    }

    println!("Before shrink: capacity = {}", map.capacity());

    map.shrink_to_fit();

    println!("After shrink: capacity = {}", map.capacity());
}
Before shrink: capacity = 112
After shrink: capacity = 112

```



## 1.17 shrink_to 缩减到指定大小

`shrink_to`方法的作用是将`HashMap`的容量缩小到指定的大小。它接受一个参数，表示希望缩小的目标容量。

`shrink_to`方法会修改`HashMap`，将其容量缩小到指定的大小。如果目标容量小于当前存储的键值对数量，那么`HashMap`的容量将保持不变。

```

fn shrink_to(){
    let mut map = HashMap::with_capacity(300);

    for i in 0..100 {
        map.insert(i, i * 2);
    }

    println!("Before shrink: capacity = {} ", map.capacity());

    map.shrink_to(50);

    println!("Before shrink: capacity = {} ", map.capacity());

}
Before shrink: capacity = 448 
Before shrink: capacity = 112 
```



## 1.18 `get_key_value`

返回key 和 value

```

fn get_key_value(){
let mut map = HashMap::new();
map.insert(1, "a");
    let res = map.get_key_value(&1);
    println!("res {:?}",res);
    let res = map.get_key_value(&2);
    println!("res {:?}",res);
}
res Some((1, "a"))
res None
```



## 1.19 remove_entry 移除并返回

```

fn remove_entry(){
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
Removed: key1 = value1
{"key2": "value2"}
```



# 2 实现的trait

`HashMap`实现了以下几个重要的trait以及它们的一些方法：

1. **`std::iter::FromIterator`：**
   - `from_iter(iter: I) -> Self`：从一个可迭代的对象`iter`构建一个`HashMap`。

2. **`std::iter::IntoIterator`：**
   - `into_iter(self) -> IntoIter`：将`HashMap`转换为一个迭代器`IntoIter`，使得你可以使用`for`循环来遍历`HashMap`的键值对。

3. **`std::iter::Iterator`：**
   - `iter(&self) -> Iter`：返回一个不可变的迭代器`Iter`，用于遍历`HashMap`的键值对。
   - `iter_mut(&mut self) -> IterMut`：返回一个可变的迭代器`IterMut`，用于遍历`HashMap`的键值对。

4. **`std::ops::Index`：**
   - `index(&self, key: K) -> &V`：通过键`key`来索引`HashMap`，返回对应的值的引用。

5. **`std::ops::IndexMut`：**
   - `index_mut(&mut self, key: K) -> &mut V`：通过键`key`来索引`HashMap`，返回对应的可变值的引用。

6. **`std::ops::Deref`：**
   - `deref(&self) -> &HashMap<K, V, H, S>`：返回对`HashMap`的引用。

7. **`std::ops::DerefMut`：**
   - `deref_mut(&mut self) -> &mut HashMap<K, V, H, S>`：返回对`HashMap`的可变引用。

8. **`std::borrow::Borrow`：**
   - `borrow(&self, key: &Q) -> &V`：通过一个实现了`Borrow<Q>` trait的类型`key`来获取`HashMap`中对应的值的引用。

9. **`std::borrow::BorrowMut`：**
   - `borrow_mut(&mut self, key: &Q) -> &mut V`：通过一个实现了`Borrow<Q>` trait的类型`key`来获取`HashMap`中对应的可变值的引用。

10. **`std::clone::Clone`：**
    - `clone(&self) -> HashMap<K, V, H, S>`：创建一个`HashMap`的克隆副本。

11. **`std::fmt::Debug`：**
    - `fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`：将`HashMap`格式化为调试输出的字符串。

这些是`HashMap`实现的一些重要的trait及其方法。通过这些方法，你可以对`HashMap`进行创建、遍历、索引、克隆和格式化等操作。

如果你想了解更多关于`HashMap`的trait和方法，请参阅Rust的文档。

















































