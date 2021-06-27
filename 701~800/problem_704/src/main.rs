use core::num;

fn main() {
    println!("Hello, world!");
}


/// https://leetcode-cn.com/problems/binary-search/
/// 704. 二分查找
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len() - 1;
    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target {
            if mid == 0 {
                return -1;
            }
            hi = mid - 1;
        } else if nums[mid] < target {
            lo = mid + 1;
        }
    }
    -1
}
