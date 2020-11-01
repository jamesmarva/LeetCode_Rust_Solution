// 
fn main() {
    let res = two_sum_v_1(vec!(1, 2, 3, 4), 3);
    println!("{:?}", res);
}

use std::vec::Vec;
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let l = nums.len();
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(l);
    for i in 0..l {
        let tmp = nums.get(i).unwrap();
        let o = target - tmp;
        let value = map.get(&o);
        match value {
            Some(t) => return vec!(*t, i as i32),
            None => {
                map.insert(*tmp, i as i32);
            },
        }
    }
    vec!()
}



pub fn two_sum_v_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let l = nums.len();
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(l);
    for i in 0..l {
        let val = nums.get(i).unwrap();
        let other = target - val;
        let otherVal = map.get(&other);
        if otherVal.is_some() {
            return vec!(*otherVal.unwrap(), i as i32);
        } else {
            map.insert(*val, i as i32);
        }
    }
    vec!()
}

