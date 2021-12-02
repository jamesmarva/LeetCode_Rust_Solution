use std::cmp;

/// wrong
pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut rst = 0;
    let mut turn_count = 0;
    for i in 0..cmp::min(k as usize, nums.len()) {
        if nums[i] < 0 {
            rst += -nums[i];
            turn_count += 1;
        } else {
            break;
        }
    }
    if (k - turn_count) & 1 == 1 {
        rst -= nums[turn_count as usize];
        turn_count += 1;
    }
    for i in turn_count as usize..nums.len() {
        rst += nums[i];
    }
    rst
}
