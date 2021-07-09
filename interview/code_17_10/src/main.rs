use core::num;

fn main() {
    println!("Hello, world!");
}


pub fn majority_element(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    let mut n = nums[0];
    let mut count = 1;
    for i in 1..nums.len() {
        if count == 0 {
            n = nums[i];
            count = 1
        } else { 
            if n == nums[i] {
                count += 1;
            } else {
                count -= 1;
            }
        }
    }

    if count >= 1 {
        let mut count_loc = 0;
        for i in 0..nums.len() {
            if nums[i] == n {
                count_loc += 1;
            }
        }
        if count_loc > nums.len() / 2 {
            return n;
        } else {
            return -1;
        }
    } else {
        -1
    }
}






