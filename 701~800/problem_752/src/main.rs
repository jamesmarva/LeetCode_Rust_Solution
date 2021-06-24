use std::{collections::{HashSet, VecDeque}, iter::FromIterator, usize};

mod both_ends;
fn main() {
    println!("Hello, world!");
}

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let set: HashSet<String>= HashSet::from_iter(deadends);
    if set.contains(&String::from("0000")) || set.contains(&target) {
        return -1;
    }
    let mut deque = VecDeque::new();
    deque.push_back(target);
    let start_str = String::from("0000");
    let mut rst = 0;
    while deque.len() > 0 {
        let mut count = deque.len();
        rst += 1;
        while count > 0 {
            if let Some(v) = deque.pop_front() {
                for i in 0..8 {
                    let s = change_str(v.clone(), i / 2, i % 2 == 1);
                    if s == start_str {
                        return rst;
                    }
                    if set.contains(&s) {
                        continue;
                    } else {
                        deque.push_back(s);
                    }
                }
            }
            count -= 1;
        }
    }
    -1
}

pub fn change(s: String, i: usize) -> String {
    change_str(s, i / 2, i % 2 == 1)
}

pub fn change_str(s: String, i: usize, add: bool) -> String {
    let mut cs: Vec<char> = s.chars().collect();
    let num = cs[i].to_digit(10);
    if let Some(mut v) = num {
        if add {
            if v == 9 {
                v = 0;
            } else {
                v += 1;
            }
        }  else {
            if v == 0 {
                v = 9;
            } else {
                v -= 1;
            }
        }
        cs[i] = v.to_string().chars().next().unwrap();
    }
    cs.iter().collect()
}

