mod lock_demo;
fn main() {

    let vec = Vec::from("1");
    let vec = Vec::from([(1, 1)]);
    let s = [0, 1, 1, 3, 5, 8, 13, 21, 34, 55];
    println!("{:?}", s.binary_search(&2));
    println!("{:?}", s.binary_search(&-1));
    println!("{:?}", s.binary_search(&21));
}

use std::collections::HashMap;
struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(mut v) = self.map.get_mut(&key) {
            match v.binary_search_by_key(&timestamp, |&(t, _)| t) {
                Ok(idx) => v[idx] = (timestamp, value),
                Err(idx) => v.insert(idx, (timestamp, value)),
            }
        } else {
            self.map.insert(key, Vec::from([(timestamp, value)]));
        } 
    }
    
    fn get(&mut self, key: String, timestamp: i32) -> String {
        if let Some(val) = self.map.get(&key) {
            return match val.binary_search_by_key(&timestamp, |&(t, _)| t) {
                Ok(idx) => val[idx].1.clone(),
                Err(idx) => if idx == 0 {
                    String::new()
                } else {
                    val[idx - 1].1.clone()
                }
            }
        } else {
            String::new()
        }
    }
}
