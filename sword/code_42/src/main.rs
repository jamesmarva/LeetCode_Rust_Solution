fn main() {
    let v = vec! [-2,1,-3,4,-1,2,1,-5,4];
    println!("{}", max_sub_array(v));
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut tmp = nums[0];
    let mut rst = tmp;
    for i in 1..nums.len() {
        tmp = nums[i].max(tmp + nums[i]);
        rst = rst.max(tmp);
    }
    rst
}



