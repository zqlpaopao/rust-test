#![allow(unused)]
// https://mp.weixin.qq.com/s/FSI_1OR9zLh99zRPAoIXJA
// 文件数据读取优化

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub fn test_file_reader() {
    //写文件
    ready_file()
}

//解析行
fn read_line(data: String) -> (String, f32) {
    let parts: Vec<&str> = data.split(':').collect();
    let station_name = parts[0].to_string();
    let value = parts[1].parse::<f32>().unwrap();
    (station_name, value)
}

struct StationValues {
    min: f32,
    max: f32,
    mean: f32,
    count: u32,
}

// 计算站点差值
fn calculate_station_values(data: String) -> HashMap<String, StationValues> {
    let mut result: HashMap<String, StationValues> = HashMap::new();

    for line in data.lines() {
        let line = line.trim();
        let (station_name, value) = read_line(line.to_string());
        result
            .entry(station_name.clone())
            .and_modify(|e| e.min = e.min.min(value))
            .or_insert(StationValues {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                count: 0,
            });
    }
    result
}

// 写文件
fn ready_file() {
    let path = "./file_reader";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let value = vec![17, 19, 23, 25];
    let addr_value = vec!["朝阳", "海淀", "昌平", "通州", "亦庄", "石景山", "东城"];
    let mut rng = thread_rng();
    for i in 1..1000_000_000 {
        let addr = addr_value.choose(&mut rng).unwrap();
        let value = value.choose(&mut rng).unwrap();

        let s = format!("{}:{}", addr, value);
        writeln!(file, "{}", s).unwrap();
    }
}
