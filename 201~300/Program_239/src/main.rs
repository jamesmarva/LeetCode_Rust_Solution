mod better;
fn main() {

}

use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut rst = Vec::new();
    let mut win: VecDeque<i32> = VecDeque::new();
    let k = k as usize;
    let mut max = i32::MIN;
    for i in 0..nums.len() {
        if i >= k {
            rst.push(max);
            if nums[i - k] == *win.front().unwrap() {
                win.pop_front();
            }
        }
        while win.len() > 0 {
            if win.back().is_some() {
                if *win.back().unwrap() < nums[i] {
                    win.pop_back();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        win.push_back(nums[i]);
        max = *win.front().unwrap();
    }
    if k == nums.len() {
        rst.push(max);
    }
    rst
}
