pub fn longest_palindrome(s: String) -> String {
    let arr: Vec<char> = s.chars().collect();
    let mut dp: Vec<Vec<bool>> = Vec::new();
    let mut start_idx = 0usize;
    let mut max = 1usize;
    let count = s.chars().count();
    // 初始化dp的是时候，就插入表示 每个位置的单个字符就是回文
    dp = (0..count).map(|x| {
        let mut rst =  vec![false; count];
        rst[x] = true;
        rst
    }).collect();

    for size in 2..=count {
        for i in 0..count {
            let j = size + i - 1;
            if j >= count {
                break;
            }
            if arr[i] == arr[j] {
                // 如果是3的话，那毫无疑问，相等就是回文，当然这里如果是3，应该是也没啥问题，就是重复了一次判断逻辑
                if size >= 4 {
                    dp[i][j] = dp[i + 1][j - 1];
                } else {
                    dp[i][j] = true;
                }
            } else {
                dp[i][j] = false;
            }
            if dp[i][j] && size > max {
                max = size;
                start_idx = i;
            }
        }
    }
    arr.iter().enumerate().filter(|&(i, v)| i >= start_idx && i < start_idx + max)
        .map(|(i, v)| v)
        .collect()
}


/// 时间复杂度 O(n^2)，空间复杂度 O(n^2)
///
pub fn longest_palindrome0(s: String) -> String {
    let arr: Vec<char> = s.chars().collect();
    let mut dp: Vec<Vec<bool>> = Vec::new();
    let mut start_idx = 0usize;
    let mut max = 1usize;
    let count = s.chars().count();
    dp = (0..count).map(|x| { vec![false; count] }).collect();
    for size in 2..=count {
        for i in 0..count {
            let j = size + i - 1;
            if j >= count {
                break;
            }
            if arr[i] == arr[j] {
                // 如果是3的话，那毫无疑问，相等就是回文，当然这里如果是3，应该是也没啥问题，就是重复了一次判断逻辑
                if size >= 4 {
                    dp[i][j] = dp[i + 1][j - 1];
                } else {
                    dp[i][j] = true;
                }
            } else {
                dp[i][j] = false;
            }
            if dp[i][j] && size > max {
                max = size;
                start_idx = i;
            }
        }
    }
    arr.iter().enumerate().filter(|&(i, v)| i >= start_idx && i < start_idx + max)
        .map(|(i, v)| v)
        .collect()
}

