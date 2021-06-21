use std::{isize, usize};



/// haystack: 大海捞针
/// needle 针
pub fn kmp_search(haystack: String, needle: String) -> isize {
    if needle.len() == 0 {
        return 0;
    }
    let next_array = get_next(&needle.chars().collect());
    let src: Vec<char> = haystack.chars().collect();
    let mat: Vec<char> = needle.chars().collect();
    let mut i1: usize = 0;
    let mut i2 = 0;
    while i1 < haystack.len() && i2 < needle.len() {
        if src[i1] == mat[i2] {
            i1 += 1;
            i2 += 1;
        } else if i2 == 0 {
            i1 += 1;
        } else {
            i2 = next_array[i2] as usize;
        }
    }
    if i2 == mat.len() {
        (i1 - 1) as isize
    } else {
        -1
    }
}

pub fn get_next_mine(s: &Vec<char>) -> Vec<isize> {

    let mut rst: Vec<isize> = vec![-1; s.len()];
    if s.len() == 1 {
        return rst;
    }
    rst[0] = -1;
    rst[1] = 0;
    for i in 2..s.len() {
        let mut pre = rst[i - 1];
        while pre != -1 && s[pre as usize] == s[i - 1] {
            pre = rst[pre as usize];
        }
        rst[i] = pre + 1;
    }
    rst
}

pub fn get_next(s: &Vec<char>) -> Vec<isize> {
    let mut rst: Vec<isize> = vec![-1; s.len()];
    rst[0] = -1;
    for i in 1..s.len() {
        let mut pre = rst[i - 1];
        while pre != -1 && s[pre as usize + 1] == s[i] {
            pre = rst[pre as usize];
        }
        if s[(pre + 1) as usize] == s[i] {
            rst[i] = pre + 1;
        }
    }
    rst
}

pub fn get_next0(s: &Vec<char>) -> Vec<usize> {
    let mut rst = vec![0; s.len()];
    let mut pre = 0usize;
    for i in 1..s.len() {
        while pre > 0 && s[pre as usize] != s[i] {
            pre = rst[pre as usize];
        }
        if s[pre] == s[i] {
            pre += 1;
        }
        rst[i] = pre;
    }
    rst
}