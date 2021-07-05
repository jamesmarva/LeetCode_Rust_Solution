

/// [2,3,2]
/// [2,2,3,4]
/// [1,2,2,4]
/// [1,1]
/// [2,2]
/// https://leetcode-cn.com/problems/set-mismatch/
/// 645. 错误的集合
fn main() {
    println!("Hello, world!");
}
use std::collections::HashSet;
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();
    let mut rst = vec![0, 0];
    let l = nums.len();
    let mut sum = (l * (l + 1) / 2) as i32;
    for i in 0..l {
        if set.contains(&nums[i]) {
            rst[0] = nums[i];
        } else {
            set.insert(nums[i]);
        }
        sum -= nums[i];
    }
    rst[1]= sum + rst[0];
    rst
}   