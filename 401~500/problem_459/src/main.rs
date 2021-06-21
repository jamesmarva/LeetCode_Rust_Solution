mod better_kmp;


fn main() {
    // println!("{:?}", get_next(&"ab".to_string().chars().collect()));
    // println!("{:?}", better_kmp::get_next(&"ab".to_string().chars().collect()));
    // println!("{:?}", better_kmp::get_next0(&"ab".to_string().chars().collect()));

    // println!("{:?}", get_next(&"aaaa".to_string().chars().collect()));
    // println!("{:?}", get_next(&"bb".to_string().chars().collect()));
    // println!("{:?}", get_next(&"ababab".to_string().chars().collect()));

    // println!("{:?}", get_next(&"a".to_string().chars().collect()));
    // "mississippi"
    // "issip"
    println!("{:?}", better_kmp::get_next0(&"issip".to_string().chars().collect()));
    println!("{:?}", better_kmp::get_next(&"issip".to_string().chars().collect()));
    println!("{:?}", better_kmp::get_next_mine(&"issip".to_string().chars().collect()));

}


pub fn repeated_substring_pattern(s: String) -> bool {
    let src: Vec<char> = s.chars()
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i > 0)
        .map(|(_, v)| v)
        .chain(s.chars().into_iter())
        .collect();
    let idx = kmp_search(&src, &s.chars().collect());
    return idx != -1 && idx < s.len() as isize && idx + s.len() as isize != src.len() as isize
}

pub fn kmp_search(src: &Vec<char>, mat: &Vec<char>) -> isize {
    if src.len() == 0 {
        return -1;
    }
    let next = get_next(mat);
    let mut i1 = 0usize;
    let mut i2 = 0usize;
    while i1 < src.len() && i2 < mat.len() {
        if src[i1] == mat[i2] {
            i1 += 1;
            i2 += 1;
        } else if i2 == 0 {
            i1 += 1;
        } else {
            i2 = next[i2] as usize;
        }
    }
    if i2 == mat.len() {
        (i1 - i2) as isize
    } else {
        -1
    }
}
pub fn get_next(s: &Vec<char>) -> Vec<isize> {
    let mut rst = vec![-1; s.len()];
    if s.len() == 1 {
        return rst;
    }
    rst[0] = -1;
    rst[1] = 0;
    for i in 2..s.len(){
        let mut pre = rst[i - 1];
        while pre != -1 && s[i - 1] != s[pre as usize] {
            pre = rst[pre as usize];
        }
        rst[i] = pre + 1;
    }
    rst
}