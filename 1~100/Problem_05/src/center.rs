use std::usize;

/// 中心扩展法
/// 时间复杂度 O(n^2)
/// 空间复杂度
pub fn longest_palindrome(s: String) -> String {
    let arr: Vec<char> = s.chars().collect();
    let mut start_idx = 0;
    let mut max = 1;
    for i in 0..arr.len() {
        let (start1, len1) = check(&arr, i, i);
        let (start2, len2) = check(&arr, i, i + 1);
        if len2 > len1 {
            if len2 > max {
                start_idx = start2;
                max = len2;
            } 
        } else {
            if len1 > max {
                start_idx = start1;
                max = len1;
            }
        }
    }
    arr.iter().enumerate()
        .filter(|&(i, v)| i >= start_idx && i < start_idx + max)
        .map(|(i, v)| v)
        .collect()
}

fn check(s: &Vec<char>, mut left_start: usize, mut right_start: usize) -> (usize, usize) {
    while right_start < s.len() && s[left_start] == s[right_start] {
        if left_start == 0 {
            return (0, right_start + 1)
        }
        left_start -= 1;
        right_start += 1;
    }
    (left_start + 1, right_start - left_start - 1)
}


