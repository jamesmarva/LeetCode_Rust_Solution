/// https://leetcode-cn.com/problems/sort-colors/
/// 荷兰国旗问题
/// 75. 颜色分类
/// 其实三路快排的思想
/// 
fn main() {
    let mut v = vec![2, 2   ];
    sort_colors(&mut v);
    v.iter().for_each(|o| print!("{} ", o));
}
pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut l = 0;
    let mut i = 0;
    let mut r = nums.len() - 1;
    while i <= r {
        if nums[i] == 1 {
            i += 1;
        } else if nums[i] == 0 {
            nums.swap(i, l);
            l += 1;
            i += 1;
        } else if nums[i] == 2 {
            nums.swap(i, r);
            if r == 0 {
                break;
            }
            r -= 1;
        }
    }
}

pub fn sort_colors_1(nums: &mut Vec<i32>) {
    let mut l = -1;
    let mut r: usize = nums.len();
    let mut i: usize = 0;
    while i < r {
        if nums[i] == 2 {
            r -= 1;
            nums.swap(r, i);
        } else if nums[i] == 1 {
            i += 1;
        } else if nums[i] == 0 {
            l += 1;
            nums.swap(i, l as usize);
            i += 1;
        }
    }
}