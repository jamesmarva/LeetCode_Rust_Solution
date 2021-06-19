mod dp_solution;
mod center;
use std::iter::FromIterator;



fn main() {
    let s = "bbbb".to_string();
    let s0 = Solution::longest_palindrome0(s);
    println!("{}", s0);
}


pub struct Solution{}

impl Solution {
    /// this is wrong 
    pub fn longest_palindrome(s: String) -> String {
        let count = s.chars().count();
        let chars: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<bool>> = Vec::new();
        let mut max_len = 1;
        let mut start_idx = 0usize;
        for _ in 0..count {
            let mut tmp = Vec::with_capacity(count);
            for _ in 0..count {
                tmp.push(false);
            }
            dp.push(tmp);
        }
        // init dp 
        for i in 0..count {
             dp[i][i] = true;
        }
        for i in 0..dp.len() {
            let t = &dp[i];
            for j in 0..t.len() {
                print!("{} ", &dp[i][j]);
            }
            println!("");
        }
        // 这样的循环机制会导致[0, 3] 的时候 [1, 2]还是没有验证过的 
        for i in 0..count {
            for j in i + 1..count {
                if chars[i] == chars[j] {
                    if j - i >= 2 {
                        dp[i][j] = dp[i + 1][j - 1];
                    } else {
                        dp[i][j] = true;
                    }
                } else {
                    dp[i][j] = false;
                }
                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    start_idx = i;
                }
            }
        }
        println!("start_idx: {}; max_len: {};", start_idx, max_len);
        let mut rst = String::new();
        for i in 0..max_len {
            rst.push(chars[start_idx + i]);
        }
        rst
    }

    // this is right code
    pub fn longest_palindrome0(s: String) -> String {
        let arr: Vec<char> = s.chars().collect();
        let count = s.chars().count();
        let mut dp: Vec<Vec<bool>> = Vec::with_capacity(count);
        let mut max_len = 1usize;
        let mut start_idx = 0usize;
        for i in 0..count {
            let mut t = vec![false; count];
            t[i] = true;
            dp.push(t);
        }
        // j - i + 1 = size
        for size in 2..=count {
            for i in 0..=count - 1 {
                let j = size + i - 1;
                if j >= count {
                    break;
                }
                if arr[i] == arr[j] {
                    if size >= 4 {
                        dp[i][j] = dp[i + 1][j - 1];
                    } else {
                        dp[i][j] = true;
                    }
                } else {
                    dp[i][j] = false;
                }
                if dp[i][j] && size > max_len {
                    max_len = size;
                    start_idx = i;
                }
            }
        }
        arr.iter().enumerate()
            .filter(|&(i, val)| i >= start_idx && i < start_idx + max_len)
            .map(|(i, val)| val)
            .collect()
    }

}