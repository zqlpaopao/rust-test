#![allow(unused)]

use num_cpus;

pub fn cpus() {
    println!("cpus num {}", num_cpus::get());
    println!("physical cpus num {}", num_cpus::get_physical());
}
