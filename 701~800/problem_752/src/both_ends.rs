use std::{collections::{HashSet, VecDeque}, iter::FromIterator};
use crate::change_str;

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let dead_set: HashSet<String>= HashSet::from_iter(deadends);
    let start_str = String::from("0000");
    if dead_set.contains(&start_str) || dead_set.contains(&target) {
        return -1;
    }
    let mut rst = 0;
    if start_str == target {
        return  rst;
    }
    let mut visited_set: HashSet<String> = HashSet::from_iter(vec![start_str.clone()]);
    let mut end_deque = VecDeque::from(vec![target]);
    let mut start_deque = VecDeque::from(vec![start_str]);
    let mut deque = start_deque.clone();
    let mut prev_set: HashSet<String> = HashSet::from_iter(end_deque.clone());
    let mut forword = true;
    while deque.len() > 0 {
        let mut count = deque.len();
        rst += 1;
        while count > 0 {
            if let Some(v) = deque.pop_front() {
                for i in 0..8 {
                    let s = change_str(v.clone(), i / 2, i % 2 == 1);
                    if prev_set.contains(&s) {
                        return rst;
                    }
                    if !dead_set.contains(&s) && !visited_set.contains(&s) {
                        visited_set.insert(s.clone());
                        deque.push_back(s);
                    } 
                }
            }
            count -= 1;
        }
        if forword {
            start_deque = deque;
            prev_set = HashSet::from_iter(start_deque.clone());
            deque = end_deque.clone();
        } else {
            end_deque = deque;
            prev_set = HashSet::from_iter(end_deque.clone());
            deque = start_deque.clone();
        }
        forword = !forword;
    }
    -1
}