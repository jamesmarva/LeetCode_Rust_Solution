use std::usize;

/// 中心扩展法
/// 时间复杂度 O(n^2)
/// 空间复杂度
pub fn longest_palindrome(s: String) -> String {
    let arr: Vec<char> = s.chars().collect();
    let (mut start_idx, mut max) = (0, 1);
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
        .filter(|&(i, _)| i >= start_idx && i < start_idx + max)
        .map(|(_, v)| v)
        .collect()
}

fn check(s: &Vec<char>, mut lt: usize, mut rt: usize) -> (usize, usize) {
    while rt < s.len() && s[lt] == s[rt] {
        if lt == 0 {
            return (0, rt + 1)
        }
        lt -= 1;
        rt += 1;
    }
    (lt + 1, rt - lt - 1)
}


