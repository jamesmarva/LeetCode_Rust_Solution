pub fn shortest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s
    }
    let mut ns: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);
    s.chars().for_each(|o| {
        ns.push('#'); 
        ns.push(o);
    });
    ns.push('#');
    let mut center = 0;
    let mut radius = 0;
    let mut radius_arr = vec![0; ns.len()];
    let mut max_len = 1;
    for i in 0..ns.len() {
        let tmp_len = if center + radius > i {
            let symmetry_i = 2 * center - i;
            let tmp_rad = radius_arr[symmetry_i];
            let ignore_dis = tmp_rad.min(center + radius - i);
            get_palindrome_len(&ns, i - ignore_dis, i + ignore_dis)
        } else {
            get_palindrome_len(&ns, i, i)
        };
        let tmp_radius = tmp_len / 2;
        radius_arr[i] = tmp_radius;
        if i - tmp_radius == 0 {
            max_len = tmp_len;
        }
        if i + tmp_radius > center + radius {
            radius = tmp_radius;
            center = i;
        }
    }
    let len = max_len / 2;
    let tmp_rst: Vec<char> = s.chars().rev().enumerate().filter(|&(i, _)| i < s.len() - len).map(|(_, v)| v).collect();
    tmp_rst.into_iter().chain(s.chars().into_iter()).collect()
}

pub fn get_palindrome_len(s: &Vec<char>, mut lt: usize, mut rt: usize) -> usize {
    while rt < s.len() && s[lt] == s[rt] {
        if lt ==  0 {
            return rt - lt + 1;
        }
        lt -= 1;
        rt += 1;
    }
    rt - lt - 1
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn work4() {
        let s = "aacecaaa".to_string();
        // aaacecaaa
        assert_eq!("aaacecaaa", shortest_palindrome(s));
        let s = "abcd".to_string();
        assert_eq!("dcbabcd", shortest_palindrome(s));
    }
}