use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in 0..routes.len() {
        let route = &routes[i];
        let mut set: HashSet<i32> = HashSet::new();
        for e in route {
            set.insert(*e);
        }
        for e in route {
            if let Some(v) = map.get_mut(&e) {
                set.iter().for_each(|o| {
                    v.insert(*o);
                });
            } else {
                map.insert(*e, set.clone());
            }
        }
    }
    println!("{:?}", map);
    let mut visited = HashSet::new();
    let mut start = HashSet::new();
    start.insert(source);
    let mut end = HashSet::new();
    end.insert(target);
    let mut rst = 0;
    while start.len() > 0 && end.len() > 0 {
        let mut tmp = HashSet::new();
        println!("start: {:?}", start);
        println!("end: {:?}", end);
        println!("----------------------");
        for v in start {
            visited.insert(v);
            if end.contains(&v) {
                return rst;
            } else {
                if let Some(next) = map.get(&v) {
                    for e in next {
                        if !visited.contains(e) {
                            tmp.insert(*e);
                        }
                    }
                }
            }
        }
        rst += 1;
        start = end;
        end = tmp;
    }
    -1
}