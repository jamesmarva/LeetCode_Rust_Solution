use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut rst = Vec::new();
    let mut win = VecDeque::new();
    let (mut max, k) = (i32::MAX, k as usize);
    for i in 0..nums.len() {
        if i >= k {
            rst.push(max);
            if nums[i - k] == *win.front().unwrap(){
                win.pop_front();
            } 
        }
        while let Some(&x) = win.back() {
            if nums[i] > x {
                win.pop_back();
            } else {
                break;
            }
        }
        win.push_back(nums[i]);
        max = *win.front().unwrap();
    }
    rst.push(max);
    rst
}