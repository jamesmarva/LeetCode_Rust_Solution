
mod counter_sort;


fn main() {
    let t = vec![3,0,6,1,5];
    println!("{}", counter_sort::h_index(t));
}

pub fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort();
    let l = citations.len();
    for i in 0..l {
        if citations[i] >= (l - i) as i32 {
            return (l - i) as i32;
        }
    }
    0
}


