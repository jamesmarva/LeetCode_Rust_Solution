mod wrong;

use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn main() {
}
pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    for item in nums.iter() {
        heap.push(Reverse(*item));
    }
    let mut turn_ct = 0;
    while heap.len() > 0 && turn_ct < k {
        if (*heap.peek().unwrap()).0 > 0 {
            break;
        }
        let tmp = heap.pop().unwrap().0;
        heap.push(Reverse(-tmp));
        turn_ct += 1;
    }
    let mut rst = 0;
    if (k - turn_ct) & 1 == 1 {
        let t = heap.pop().unwrap().0;
        rst += -t;
    }
    while heap.len() > 0 {
        rst += heap.pop().unwrap().0;
    }
    rst
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn t1() {
        let nums = vec![4,2,3];
        let k = 1;
        println!("t1 {}", largest_sum_after_k_negations(nums, k));
    }

    
    #[test]
    pub fn t2() {
        let nums = vec![3,-1,0,2];
        let k = 3;
        println!("t2 {}", largest_sum_after_k_negations(nums, k));
    }

    #[test]
    pub fn t3() {
        let nums = vec![2,-3,-1,5,-4];
        let k = 2;
        println!("t3 {}", largest_sum_after_k_negations(nums, k));
    }

    #[test]
    pub fn t4() {
        let nums = vec![-4, -2, -3];
        let k = 5;
        println!("t4 {}", largest_sum_after_k_negations(nums, k));
    }

    #[test]
    pub fn t77() {
        let nums = vec![-8,3,-5,-3,-5,-2];
        let k = 6;
        println!("t77 {}", largest_sum_after_k_negations(nums, k));

    }

    #[test]
    pub fn t52() {
        let nums = vec![4,-5,4,-5,9,4,5];
        let k = 1;
        println!("t52 {}", largest_sum_after_k_negations(nums, k));

    }
    


}