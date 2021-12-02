use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut map = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut rst = Vec::new();
    for (idx, val) in score.iter().enumerate() {
        map.insert(*val, idx);
        heap.push(*val);
        rst.push(0.to_string());
    }
    let mut rank = 1;
    while heap.len() > 0 {
        let val = heap.pop().unwrap();
        let i = map.get(&val).unwrap();
        match rank {
            1 => rst[*i] = "Gold Medal".to_string(),
            2 => rst[*i] = "Silver Medal".to_string(),
            3 => rst[*i] = "Bronze Medal".to_string(),
            _ => rst[*i] = rank.to_string(),

        }
        rank += 1;
    }
    rst
}
