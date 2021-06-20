fn main() {
    let s = String::from("aacecaaa").chars().count();
    println!("{}", s);
    println!("{:?}", make_next(&String::from("aacecaaa").chars().collect()));
    println!("{}", shortest_palindrome(String::from("aacecaaa")));
    println!("{}", shortest_palindrome(String::from("abcd")));
}

/// use kmp solution
/// Thinking?
/// find s prefix-string equals reverse-s suffix-string max length
/// how to code?
/// src: reverse_s
/// match: s
pub fn shortest_palindrome(s: String) -> String {
    let mat: Vec<char> = s.chars().collect();
    if mat.len() <= 1 {
        return s
    }
    let mut src = mat.clone();
    src.reverse();
    let next = make_next(&mat);
    let mut i1: usize = 0;
    let mut i2: usize = 0;
    while i1 < src.len() && i2 < mat.len() {
        if src[i1] == mat[i2] {
            i1 += 1;
            i2 += 1;
        } else if next[i2] == -1 {
            i1 += 1;
        } else {
            i2 = next[i2] as usize;
        }
    }
    if i2 == mat.len() {
        s
    } else {
        src.iter().enumerate()
            .filter(|&(i, _)| i < src.len() - i2)
            .map(|(_, v)| v)
            .chain(mat.iter())
            .collect()
    }
}

/// make next array
/// next[i] means in match[0..i-1], 
/// 以match[0]为开始的前缀字符串（不包括match[i-1]）和 以 match[i-1]为结束的后缀字符串的最大匹配长度。
/// 
pub fn make_next(s: &Vec<char>) -> Vec<isize> {
    let mut rst: Vec<isize> = vec![0; s.len()];
    rst[0] = -1;
    rst[1] = 0;
    for i in 2..s.len() {
        let mut pre = rst[i - 1];
        loop {
            if s[pre as usize] == s[i-1] {
                rst[i] = pre + 1;
                break;
            } else if pre != 0 {
                pre = rst[pre as usize];
            } else {
                rst[i] = 0;
                break;
            }
        }
    }
    rst
}