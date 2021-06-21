pub fn shortest_palindrome(s: String) -> String {
    
    "".to_string()
}

pub fn check(s: &Vec<char>, mut lt: usize, mut rt: usize) -> usize {
    while s[lt] == s[rt] {
        if lt ==  0 {
            return rt - lt + 1;
        }
        lt += 1;
        rt += 1;
    }
    rt - lt - 1
}