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

/// https://leetcode-cn.com/problems/nth-digit/
/// 400. 第 N 位数字
/// 1..=9 : 1..=9
/// 10..=99 10..=2*90+9 = 189 
/// 100..=999 189..=3*900 + 189 = 2889
/// 1000..=9999 2889..=4*9000 + 2889 = 36000 + 2889 = 38889
pub fn find_nth_digit(n: i32) -> i32 {
    let mut arr: Vec<(i32, i32)> = Vec::new();
    arr.push((9, 0));
    arr.push((189, 1));
    arr.push((2889, 2));
    arr.push((38889, 3));
    arr.push((488889, 4));
    arr.push((5888889, 5));
    arr.push((68888889, 6));
    arr.push((788888889, 7));
    arr.push((2147483647, 8));
    let mut prev = 0;
    let mut bit_num = 0;
    for (k, v) in arr.iter() {
        if n <= *k {
            bit_num = *v;
            break;
        } else {
            prev = *k;
        }
    }
    // println!("{:?}", (prev, curr, bit_num));
    let dis = n - (prev + 1);
    // println!("dis: {}", &dis);

    let num: i32 = dis / (bit_num + 1);
    // println!("num: {}", &num);

    let s = (10i32.pow(bit_num as u32) + num).to_string();
    // println!("s {}", s);
    let th = dis  % (bit_num + 1);
    let s: Vec<char> = s.chars().collect();
    (s[th as usize] as u8 - b'0') as i32
} 