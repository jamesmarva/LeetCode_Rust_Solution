mod stage_1st;
mod stage_2nd;

fn main() {
    // println!("{}", find_nth_digit(3));
    // println!("{}", find_nth_digit(9));
    println!("{}", find_nth_digit(10)); // 1
    println!("{}", find_nth_digit(11)); // 0
    println!("{}", find_nth_digit(12)); // 1
    println!("{}", find_nth_digit(13)); // 1
    println!("{}", find_nth_digit(14)); // 1
    println!("{}", find_nth_digit(15)); // 1
}

fn test() {
    let mut rst = 0;
    for i in 1..=9 {
        rst += i.to_string().len();
    }
    println!("{}", rst); // 9

    for i in 10..=99 {
        rst += i.to_string().len();
    }
    println!("{}", rst); // 189

    for i in 100..=999 {
        rst += i.to_string().len();
    }
    println!("{}", rst); // 2889

    for i in 1000..=9999 {
        rst += i.to_string().len();
    }
    println!("{}", rst); // 38889

    for i in 10000..=99999 {
        rst += i.to_string().len();
    }
    println!("{}", rst); // 488889

    for i in 100000..=999999 {
        rst += i.to_string().len();
    }
    println!("{}", rst); // 5888889

    for i in 1000000..=9999999 {
        rst += i.to_string().len();
    }
    println!("{}", rst); // 68888889

    println!("{}", rst); // 788888889

    println!("{}", rst); // 2147483647
    println!("{}", rst); // 2147483647


    println!("{}", 2i32.pow(30));
    println!("{}", std::i32::MAX);
    let mut max = std::i32::MAX;
    let mut r = String::new();
    while max > 0 {
        if max & 1 == 1 {
            r.push('1');
        }
        max >>= 1;
    }
    r.push('0');
    r = r.chars().rev().collect();
    print!("{}; r.len(): {};", r, r.len());
}

pub fn find_nth_digit(n: i32) -> i32 {
    
    1
}