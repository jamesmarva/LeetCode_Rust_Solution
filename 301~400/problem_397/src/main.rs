fn main() {

    println!("{}", count_1bit(1));
    println!("{}", count_1bit(-1));
    println!("{}", count_1bit(111));
    println!("{}", count_1bit(-111));

    println!("{}", count_1bit_better(1));
    println!("{}", count_1bit_better(-1));
    println!("{}", count_1bit_better(111));
    println!("{}", count_1bit_better(-111));
}

pub fn count_1bit(mut num: i32) -> u32 {
    let mut rst = 0;
    for _ in 0..32 {
        if num & 1 == 1 {
            rst += 1;
        }
        num >>= 1;
    }
    return rst
}


pub fn count_1bit_better(mut num: i32) -> u32 {
    let mut rst = 0;
    for _ in 0..32 {
        if num ^ 0 == 0 {
            return rst
        }
        if num & 1 == 1 {
            rst += 1;
        }
        num >>= 1;
    }
    return rst
}

#[test]
mod test {
    
}