
use std::{collections::{HashSet}, iter::FromIterator};
use crate::change_str;

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited_set: HashSet<String> = deadends.into_iter().collect();
    if visited_set.contains("0000") {
        return -1;
    }
    let mut rst = 0;
    let mut start: HashSet<String>  = HashSet::from_iter(vec!["0000".to_string()]);
    if start.contains(&target) {
        return rst;
    }
    let mut end: HashSet<String> = HashSet::from_iter(vec![target]);
    while start.len() > 0 && end.len() > 0 {
        let mut t = HashSet::new();
        for s in start.iter() {
            if end.contains(s) {
                return rst;
            } else {
                visited_set.insert(s.clone());
                for i in 0..8 {
                    let new_str = change_str(s.clone(), i / 2, i % 2 == 1);
                    if !visited_set.contains(&new_str) {
                        t.insert(new_str);
                    } 
                } 
            }
        }
        rst += 1;
        start = end;
        end = t;
    }
    -1
}
