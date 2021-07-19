fn main() {
}

/// 1877. 数组中最大数对和的最小值
/// https://leetcode-cn.com/problems/minimize-maximum-pair-sum-in-array/
/// https://leetcode-cn.com/problems/minimize-maximum-pair-sum-in-array/solution/shu-zu-zhong-zui-da-shu-dui-he-de-zui-xi-cvll/
/// 题目很简单，但是这种解题的思路很重要
pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut rst = 0;
    let mut lt = 0;
    let mut rt = nums.len() - 1;
    while lt < rt {
        rst = rst.max(nums[lt] + nums[rt]);
        lt += 1;
        rt -= 1;
    }
    rst
}