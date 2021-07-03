mod buckets;

fn main() {
}


use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let mut count_map = HashMap::new();
    s.chars().for_each(|o| {
        *count_map.entry(o).or_insert(0u32) += 1;
    });
    let mut list: Vec<(char, u32)> = count_map.iter()
        .map(|(k, v)| {
            (*k, *v)
        }).collect();
    list.sort_by(|a, b| b.1.cmp(&a.1));
    let mut rst = String::new();
    for (c, count) in list {
        for _ in 0..count {
            rst.push(c);
        }
    }
    rst
}
