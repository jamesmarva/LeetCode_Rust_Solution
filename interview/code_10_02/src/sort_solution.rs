use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for s in strs {
        let mut ss = s.chars().collect::<Vec<char>>();
        ss.sort_unstable();
        let v = map.entry(ss).or_insert(Vec::new());
        v.push(s);
    }
    map.into_iter().map(|(_, v)| v).collect()
}