mod better;
fn main() {
    
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let (mut slow, mut fast, len) = (0, 0, nums.len());
    while fast < len {
        if nums[fast] == 0 {
            fast += 1;
        }

        if nums[slow] != 0 {
            if slow == fast {
                fast += 1;
            } 
            slow += 1;
        }

        if fast < len && nums[slow] == 0 && nums[fast] != 0 {
            nums.swap(slow, fast);
        }
    }
}
