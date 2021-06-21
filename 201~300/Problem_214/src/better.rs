use crate::make_next;

pub fn shortest_palindrome(s: String) -> String {
    let mat: Vec<char> = s.chars().collect();
    if mat.len() <= 1 {
        return s;
    }
    let src: Vec<char> = mat.iter().map(|x| x.clone()).rev().collect();
    let (mut i1, mut i2) = (0usize, 0usize);
    let next = get_next(&mat);
    while mat.len() > i2 && src.len() > i1 {
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
        s
    } else {
        src.iter().enumerate()
            .filter(|&(i, _)| i < mat.len() - i2)
            .map(|(_, v)| v)
            .chain(mat.iter())
            .collect()
    }   
}

pub fn get_next(src: &Vec<char>) -> Vec<isize> {
    if src.len() < 1{
        return vec![]
    }
    let mut rst= vec![0isize; src.len()];
    rst[0] = -1;
    rst[1] = 0;
    for i in 2..src.len() {
        let mut pre = rst[i - 1];
        while pre != -1 && src[pre as usize] != src[i - 1] {
            pre = rst[pre as usize];
        }
        rst[i] = pre + 1;
    }
    rst    
}


pub fn get_next0(s: &Vec<char>) -> Vec<isize> {
    if s.len() < 1 {
        return vec![];
    }
    let mut rst = vec![0isize; s.len()];
    rst[0] = -1;
    rst[1] = 0;
    for i in 2..s.len() {
        let mut pre = rst[i - 1];
        loop {
            if s[pre as usize] == s[i - 1] {
                rst[i] = pre + 1;
                break;
            } else if pre == 0 {
                rst[i] = 0;
                break;
            } else {
                pre = rst[pre as usize];
            }
        }
    }
    rst
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn work1() {
        let s = "aacecaaa".to_string();
        // assert_eq!(vec![0isize; 0], get_next(&s.chars().collect()));
        assert_eq!(get_next0(&s.chars().collect()), get_next(&s.chars().collect()));
        assert_eq!(make_next(&s.chars().collect()), get_next(&s.chars().collect()));
    }

    #[test]
    #[ignore]
    fn work2() {
        let s = "abcdabcd".to_string();
        // assert_eq!(vec![0isize; 0], get_next(&s.chars().collect()));
        assert_eq!(get_next0(&s.chars().collect()), get_next(&s.chars().collect()));
        assert_eq!(make_next(&s.chars().collect()), get_next(&s.chars().collect()));
    }

    #[test]
    #[ignore]
    fn work3() {
        let s = "aaaa".to_string();
        // assert_eq!(vec![0isize; 0], get_next(&s.chars().collect()));
        assert_eq!(get_next0(&s.chars().collect()), get_next(&s.chars().collect()));
        assert_eq!(make_next(&s.chars().collect()), get_next(&s.chars().collect()));
    }

    #[test]
    #[ignore]
    fn work4() {
        let s = "aacecaaa".to_string();
        assert_eq!("aaacecaaa", shortest_palindrome(s));
        let s = "abcd".to_string();
        assert_eq!("dcbabcd", shortest_palindrome(s));
    }
}

