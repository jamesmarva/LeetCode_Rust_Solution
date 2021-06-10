fn main() {
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() == 1 {
        return 1;
    }
    let (mut slow, mut fast, mut pre) = (1usize, 1usize, nums[0]);
    while fast < nums.len() {
        if pre == nums[fast] {
            fast += 1;
        }
    }
    slow as i32 + 1
}