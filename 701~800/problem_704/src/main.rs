use core::num;

fn main() {
    let mut v = vec![1,12,12,12,12,121234,124,1234,124,12,34,2134,1234,1234,51,5,2345,23,45,2345,23];
    v.sort();
    v.binary_search(&1234);

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
