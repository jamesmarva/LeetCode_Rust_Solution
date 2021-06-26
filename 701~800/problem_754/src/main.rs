fn main() {
    let mut rst = 0;
    for i in 1..=100 {
        rst += i;
        println!("i: {}; rst: {}", i, rst);
    }
}
/// 1+2+3+...+k = k * (k + 1) / 2
pub fn reach_number(target: i32) -> i32 {
    let target = target.abs();
    let mut tmp = 0;
    let mut i = 0;
    loop {
        i += 1;
        tmp += i;
        if tmp == target || (tmp > target && (tmp - target) & 1 == 0) {
            break;
        }
        
    } 
    i
}
