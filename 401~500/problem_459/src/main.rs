fn main() {
    println!("{:?}", get_next(&"aaaa".to_string().chars().collect()));
    println!("{:?}", get_next(&"ab".to_string().chars().collect()));
    println!("{:?}", get_next(&"bb".to_string().chars().collect()));
    println!("{:?}", get_next(&"ababab".to_string().chars().collect()));

    // println!("{:?}", get_next(&"a".to_string().chars().collect()));

}


pub fn repeated_substring_pattern(s: String) -> bool {
    let next = get_next(&s.chars().collect());
    if s.len() <= 1 {
        return true;
    }
    false

}

pub fn get_next(s: &Vec<char>) -> Vec<isize> {
    let mut rst = vec![0; s.len()];
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