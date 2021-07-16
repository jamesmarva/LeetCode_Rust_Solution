pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let first_idx = find_first(&nums, target);
    let last_idx = find_last(&nums, target);
    if first_idx == -1 {
        0
    } else {
        last_idx - first_idx + 1
    }
}
fn find_first(v: &Vec<i32>, target: i32) -> i32 {
    let mut lt = 0;
    let mut rt = v.len() - 1;
    while lt <= rt {
        let m = lt + ((rt - lt) >> 1);
        if v[m] == target {
            if (m > 0 && v[m - 1] != target) || m == 0 {
                return m as i32;
            } else {
                rt = m - 1;
            }
        } else if v[m] < target {
            lt = m + 1;
        } else if v[m] > target {
            if m == 0 {
                return -1;
            }
            rt = m - 1;
        }
    }
    -1
}

fn find_last(v: &Vec<i32>, target: i32) -> i32 {
    let mut lt = 0;
    let mut rt = v.len() - 1;
    while lt <= rt {
        let m = lt + ((rt - lt) >> 1);
        if v[m] == target {
            if (m + 1 < v.len() && v[m + 1] != target) || m == v.len() - 1 {
                return m as i32;
            } else {
                lt = m + 1;
            }
        } else if v[m] > target {
            if m == 0 {
                return -1;
            }
            rt = m - 1;
        } else if v[m] < target {
            lt = m + 1;
        }
    }
    -1
}