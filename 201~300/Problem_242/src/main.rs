fn main() {

}


use std::collections::HashMap;
///
/// 你可以假设字符串只包含小写字母。
pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = [0i32; 26];
    for c in s.chars() {
        map[(c as u8 - 'a' as u8) as usize] += 1;
    }
    for c in t.chars() {
        map[(c as u8 - 'a' as u8) as usize] -= 1;
    }
    for e in map.iter() {
        if *e != 0 {
            return false;
        }
    }
    let mut iter = map.iter();
    while let Some(&val) = iter.next() {
        if val != 0 {
            return false;
        }
    }
    return true;
}

// 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？
pub fn is_anagram_1(s: String, t: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if let Some(val) = map.get_mut(&c) {
            *val += 1;
        } else {
            map.insert(c, 1);
        }
    }
    for c in t.chars() {
        if let Some(val) = map.get_mut(&c) {
            *val -= 1;
        } else {
            map.insert(c, -1);
        }
    }
    for k in map.keys() {
        if *map.get(k).unwrap() != 0 {
            return false;
        }
    }
    true
}