
pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut tmp: Vec<(i32, usize)> = intervals.iter().enumerate().map(|(idx, interval)| (interval[0], idx)).collect();
    tmp.sort_by_key(|&(val, _)| val);

    let mut rst = vec![-1; intervals.len()];
    for i in 0..intervals.len() {
        let interval = &intervals[i];
        let target = interval[1];
        let mut lo = 0;
        let mut hi = tmp.len() - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if tmp[mid].0 < target {
                lo = mid + 1;
            } else if tmp[mid].0 >= target && (mid == 0 || tmp[mid - 1].0 < target) {
                rst[i] = tmp[mid].1 as i32;
                break;
            } else {
                hi = mid - 1;
            }
        }
    }
    rst
}
