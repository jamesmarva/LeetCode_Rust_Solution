pub fn longest_palindrome(s: String) -> String {
    let mut start_idx = 0;
    let mut max_len = 1;
    let mut arr_chars = vec!['#'];
    for c in s.chars() {
        arr_chars.push(c);
        arr_chars.push('#');
    }
    let len =  arr_chars.len();
    let mut center = 0;
    let mut r = 0;
    let mut r_arr = vec![0; len];
    for i in 0..len {
        let mut tmp_len = 0;
        if i < center + r {
            let tmp_dis = (center + r - i).min(r_arr[2 * center - i]);
            tmp_len = check(&arr_chars, i - tmp_dis, i + tmp_dis);
        } else {
            tmp_len = check(&arr_chars, i, i);
        }
        let tmp_r = (tmp_len - 1) / 2;
        r_arr[i] = tmp_r;
        if tmp_len > max_len {
            max_len = tmp_len;
            start_idx = i - (tmp_len - 1) / 2;
        }
        if i + tmp_r + 1 > center + r {
            center = i;
            r = tmp_r;
        }
    }
    arr_chars.iter().enumerate()
        .filter(|&(i, _)| i >= start_idx && i < start_idx + max_len && i & 1 != 0)
        .map(|(_, v)| v)
        .collect()
}

fn check(s: &Vec<char>, mut lt: usize, mut rt: usize) -> usize {
    while rt < s.len() && s[lt] == s[rt] {
        if lt == 0 {
            return rt - lt + 1;
        }
        lt -= 1;
        rt += 1;
    }
    rt - lt - 1
}
