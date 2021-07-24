use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    let mut count = [0; 26];
    for s in strs {
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        let v = map.entry(count).or_insert(Vec::new());
        v.push(s.clone());
        count =  [0; 26];
    }
    map.into_iter().map(|(_, v)| v).collect()
}