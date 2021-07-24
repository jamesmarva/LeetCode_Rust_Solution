
use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for s in strs {
        let k = s.as_bytes().iter().map(|i| (i - b'a') as usize)
            .fold(vec![0; 26], |mut v, idx| {
                v[idx] += 1;
                v
            });
        map.entry(k).or_insert(Vec::new()).push(s);
    }
    map.into_iter().map(|(_, v)| v).collect()
}