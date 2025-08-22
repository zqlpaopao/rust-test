// https://mp.weixin.qq.com/s/AY-29Xgfimp34hvu6Jhb6w
#![allow(unused)]
use dashmap::DashMap;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use whirlwind::ShardMap;
use tokio::sync::RwLock;

pub async fn dash_map_test() {
    //基础测试
    // base_test()

    //线程安全 dashmap 在异步中 下面的会死锁
    // test_thread_hash().await


    // test_whirlwind().await;
    test_flurry().await;

}

async fn test_flurry() {
    // 使用String作为值类型，避免引用问题
    let map = Arc::new(RwLock::new(HashMap::<i32, String>::new()));

    // 写入数据
    {
        let mut write_guard = map.write().await;
        write_guard.insert(100, "a".to_string());
        write_guard.insert(200, "b".to_string());
    }

    let mut handles = vec![];
    for i in 0..10 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            // 读取
            {
                let read_guard = map.read().await;
                if let Some(v) = read_guard.get(&1) {
                    println!("Task {} read: {}", i, v);
                }
            }

            // 写入 - 先创建持久化的String
            let value = format!("value-{}", i);
            {
                let mut write_guard = map.write().await;
                write_guard.insert(i, value); // 转移所有权
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    let final_map = map.read().await;
    println!("Final: {:?}", *final_map);
}


// 还有papaya
async  fn test_whirlwind(){
    // Initialize ShardMap manually since it doesn't implement FromIterator
    let maps = ShardMap::with_capacity(3);
    maps.insert(100, "one").await;
    maps.insert(200, "two").await;
    maps.insert(300, "three").await;

    let mut handles = vec![];

    for i in 0..10 {
        // Clone the Arc inside ShardMap for each task
        let maps = maps.clone();

        handles.push(tokio::spawn(async move {
            // Read operation example
            if let Some(value) = maps.get(&100).await {
                println!("Thread {:#?} read: {:#?}", i, value.to_string());
            }

            maps.insert(i, "lk").await;

            if let Some(mut value) = maps.get_mut(&0).await {
                *value = "modified";
            }

            // tokio::time::sleep(Duration::from_secs(1)).await;
        }));
    }

    // Need to clone for use outside the loop
    let maps_clone = maps.clone();
    if let Some(mut value) = maps_clone.get_mut(&0).await {
        *value = "main thread modified";
    }

    // tokio::time::sleep(Duration::from_secs(1)).await;

    for handle in handles {
        let _ = handle.await;
    }

    // Print contents manually since ShardMap doesn't implement Debug
    println!("Final map contents:");

    // 记录大小即可
    println!("ShardMap size: {}", maps.len().await);

}


//

async fn  test_thread_hash(){
    let maps: Arc<DashMap<i32, &str>> = Arc::new(
        vec![(100, "one"), (200, "two"), (300, "three")]
            .into_iter()
            .collect()
    );

    let mut handles = vec![];

    for i in 0..10 {
        let map = Arc::clone(&maps);

        handles.push(tokio::spawn(async move {
            // Read operation example
            if let Some(value) = map.get(&1) {
                println!("Thread {} read: {}", i, *value);
            }
            map.insert(i,"lk");
            let _x = map.get_mut(&0);
            tokio::time::sleep(Duration::from_secs(10)).await;
        }));
    }

    let _x = maps.get_mut(&0);
    tokio::time::sleep(Duration::from_secs(10)).await;

    for handle in handles {
        let _ = handle.await;
    }
    println!("{:#?}", maps);
}

fn base_test() {
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
