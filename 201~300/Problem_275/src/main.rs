mod binary_search;

///
/// [0,1,3,5,6]
/// [100]
/// [1, 1]
fn main() {
    let v = vec![1, 1];
    println!("{}", binary_search::h_index(v));
}
pub fn h_index(citations: Vec<i32>) -> i32 {
    for i in 0..citations.len() {
        if citations[i] as usize >= citations.len() - i {
            return (citations.len() - i) as i32;
        }    
    }
    0
}