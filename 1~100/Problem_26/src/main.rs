fn main() {
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let (mut slow, mut fast) = (0usize, 0usize);
    while fast < nums.len() {
        if nums[fast] == nums[slow] {
            fast += 1;
        }
        if fast < nums.len() && nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }
    slow as i32 + 1
}