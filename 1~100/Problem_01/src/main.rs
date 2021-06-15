fn main() {

}
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        if let Some(&val) = map.get(&(target - nums[i])) {
            return vec![val as i32, i as i32];
        } else {
            map.insert(nums[i], i);
        }
    }
    panic!("没有找到");
}