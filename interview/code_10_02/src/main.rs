mod better_code;
use core::str;

fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut count = [0; 26];
    let mut map = HashMap::new();
    for s in strs {
        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }
        let k = convert_to_str(&count);
        let v = map.entry(k).or_insert(Vec::new());
        v.push(s.clone());
        count = [0; 26];
    }
    // map.values().cloned().collect()
    map.into_iter().map(|(_, v)| v).collect()
}

pub fn convert_to_str(count: &[i32; 26]) -> String {
    let mut rst = String::new();
    for i in 0..26 {
       rst.push(i as u8 as char); 
       rst.push(' ');
       rst.push(count[i] as u8 as char);
    }
    rst
}