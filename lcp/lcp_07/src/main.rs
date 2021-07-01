/// 5
/// [[0,2],[2,1],[3,4],[2,3],[1,4],[2,0],[0,4]]
/// 3
/// 3
/// [[0,2],[2,1]]
/// 2

fn main() {
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// 
/// https://leetcode-cn.com/problems/chuan-di-xin-xi/
/// LCP 07. 传递信息
pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let start = 0; 
    let mut targets = HashMap::new();
    for v in relation {
        let tmp_set = targets.entry(v[0]).or_insert(HashSet::new());
        tmp_set.insert(v[1]);
    }
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut level = k;
    let mut rst = 0;
    while queue.len() > 0 && level > 0 {
        let mut ct = queue.len();
        while ct > 0 {
            let key = queue.pop_front().unwrap();
            if let Some(tartget_set) = targets.get(&key) {
                for tt in tartget_set {
                    queue.push_back(*tt);
                }
            }
            ct -= 1;
        }
        level -= 1;
    }
    while let Some(v) = queue.pop_front() {
        if v == n - 1 {
            rst += 1;
        }
    }
    rst
}