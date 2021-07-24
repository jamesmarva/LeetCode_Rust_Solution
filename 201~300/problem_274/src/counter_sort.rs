pub fn h_index(mut citations: Vec<i32>) -> i32 {
    let l = citations.len();
    let mut buckets: Vec<usize> = vec![0; l + 1];
    for i in 0..l {
        let t = citations[i].min(l as i32);
        buckets[t as usize] += 1;
    }
    println!("{:?}", buckets);
    let mut c = 0;
    for i in (0..buckets.len()).rev() {
        c += buckets[i];
        if c >= i {
            return i as i32;
        }
    }
    0
}