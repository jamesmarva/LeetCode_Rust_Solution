use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let mut rst = String::new();
    let mut count_map: HashMap<char, usize> = HashMap::new();
    let mut maxFrequency: usize = 0;
    s.chars().for_each(|o| {
        *count_map.entry(o).or_insert(0usize) += 1;
        maxFrequency = maxFrequency.max(count_map[&o] as usize);
    });
    let mut buckets = vec![Vec::new(); maxFrequency + 1];
    for (k, v) in count_map {
        buckets[v as usize].push(k);
    }
    for i in (0..=maxFrequency).rev() {
        let tmp_count = &buckets[i];
        for c in tmp_count {
            for _ in 0..i {
                rst.push(*c);
            }
        }
    }
    rst
}