mod binary_search_code;
fn main() {
    let v = vec![5,7,7,8,8,10];
    println!("{}", binary_search_code::search(v, 7));
}


pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut rst = 0;
    for i in 0..nums.len() {
        if nums[i] == target {
            rst += 1;
        }
    }
    rst
}









