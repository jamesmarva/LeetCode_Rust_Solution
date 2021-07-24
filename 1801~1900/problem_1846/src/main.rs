fn main() {
    let arr = ["hello", "world"];
    for s in arr.iter() {
        println!("{}", &s);
    }
}

/// https://leetcode-cn.com/problems/maximum-element-after-decreasing-and-rearranging/
/// 1846. 减小和重新排列数组后的最大元素
pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort();
    if arr[0] > 1 {
        arr[0] = 1;
    }
    let mut max = 1;
    for i in 1..arr.len() {
        if arr[i - 1] + 1 < arr[i] {
            arr[i] = arr[i - 1]  + 1;
        }
        max = max.max(arr[i]);
    }
    max
}