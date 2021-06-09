pub fn move_zeroes(nums: &mut Vec<i32>) {
    let (mut slow, len) = (0, nums.len());
    for fast in 0..len {
        if nums[fast] != 0 {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    while slow < len {
        nums[slow] = 0;
        slow += 1;
    }
}