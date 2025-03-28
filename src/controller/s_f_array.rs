#![allow(unused)]
use std::cmp::Ordering;

pub fn s_f() {
    //704 二分查找
    let arr = vec![-1, 0, 3, 5, 9, 12];
    let res = search(arr, 9);
    println!("二分查找的下标id {:?}", res);

    //移除元素
    // let mut arr =vec![1,2,2,3,4,2,5,2,6];
    // let res = remove_element(&mut arr,2);
    // println!("移除指定元素 {}",res);
    // println!("移除指定元素后 {:?}",arr);

    //有序数组的平方
    // let arr= vec![-4,-1,0,3,10];
    // let res = sorted_squares(arr);
    // println!("有序数组的平方 {:?}",res);

    // 209 长度最小的子数组
    let arr = vec![2, 3, 1, 2, 4, 3];
    let res = min_sub_array_len(7, arr);
    println!("长度最小的子数组 {:?}", res);
}

//******************************************** 209 长度最小的子数组 滑动窗口
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let (mut result, mut sub_length) = (i32::MAX, 0);
    let (mut sum, mut i) = (0, 0);
    for (pos, val) in nums.iter().enumerate() {
        sum += val;
        while sum >= target {
            sub_length = (pos - i + 1) as i32;
            if result.gt(&sub_length) {
                result = sub_length;
            }
            sum -= nums[i];
            i += 1;
        }
    }
    if result == i32::MAX {
        return 0;
    }
    result
}

//******************************************** 977 有序数组的平方 双指针
//双指针
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let (mut i, mut j, mut k) = (0, n - 1, n);
    let mut arr = vec![0; n];
    while i <= j {
        if nums[i].pow(2) < nums[j].pow(2) {
            arr[k - 1] = nums[j].pow(2);
            j -= 1;
        } else {
            arr[k - 1] = nums[i].pow(2);
            i += 1;
        }
        k -= 1;
    }
    arr
}
// 暴力
pub fn sorted_squares_bl(nums: Vec<i32>) -> Vec<i32> {
    let mut arr = Vec::with_capacity(nums.len());
    for vl in nums {
        arr.push(vl * vl);
    }
    arr.sort();
    arr
}

//******************************************** 移除元素
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    //快慢指针
    let mut slow = 0;
    for pos in 0..nums.len() {
        if nums[pos].ne(&val) {
            nums.swap(slow, pos);
            slow += 1;
        }
    }

    slow as i32
}
//******************************************** 704 二分查找
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() as i32 - 1);
    while left <= right {
        let mid = (right + left) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Equal => return mid,
            Ordering::Greater => right -= 1,
        }
    }
    -1
}
