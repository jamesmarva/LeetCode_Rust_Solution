use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

mod double_end;

fn main() {
    println!("Hello, world!");
    let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];

    println!("{}", double_end::num_buses_to_destination(routes, 1, 6));
}


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
    let mut visited = HashSet::new();
    visited.insert(source);
    let mut queue = VecDeque::new();
    queue.push_back(source);
    let mut rst = 0;
    while queue.len() > 0 {
        let mut count = queue.len();
        while count > 0 {
            if let Some(v) = queue.pop_front() {
                if v == target {
                    return rst;
                } else {
                    if let Some(next) = map.get(&v) {
                        for e in next {
                            if !visited.contains(e) {
                                queue.push_back(*e);
                                visited.insert(*e);
                            }
                        }
                    }
                }

            }
            count -= 1;
        }
        rst += 1;
    }
    -1
}