pub fn h_index(citations: Vec<i32>) -> i32 {
    let l = citations.len();
    let mut rt = l - 1;
    let mut lt = 0;
    while lt <= rt {
        let mid = lt + ((rt - lt) >> 1);
        let tmp = (l - mid) as i32; 
        println!("mid: {}; tmp: {}", mid, tmp);
        if citations[mid] >= tmp && (mid == 0 || citations[mid - 1] < tmp + 1) {
            return tmp;
        } else if citations[mid] < tmp {
            lt = mid + 1;
        } else {
            rt = mid - 1;
        }
    }
    0
}