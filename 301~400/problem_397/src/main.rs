
use std::i32::MAX as i32_max;
///
/// 给定一个正整数 n ，你可以做如下操作：
/// - 如果 n 是偶数，则用 n / 2替换 n 。
/// - 如果 n 是奇数，则可以用 n + 1或n - 1替换 n 。
/// n 变为 1 所需的最小替换次数是多少？
/// 链接：https://leetcode-cn.com/problems/integer-replacement
pub fn integer_replacement(mut n: i32) -> i32 {
    let mut rst = 0;
    while n != 1 {
        println!("{:?}", count_1bit_better(n));
        if n & 1 == 0 {
            n >>= 1;
        } else {
            if n == 3 {
                return 2;
            } else if n == i32_max{
                return 32;
            } else if n & 2 == 2 {
                n += 1;
            } else {
                n -= 1;
            }
        }
        rst += 1;
    }
    rst
}

///
/// 
fn main() {
    // println!("{}", integer_replacement(7));
    // println!("{}", integer_replacement(8));
    // println!("{}", integer_replacement(4));
    // println!("{}", integer_replacement(17));
    // println!("{}", integer_replacement(18));
    // println!("{}", integer_replacement(14));
    // println!("{}", integer_replacement(147));
    // println!("{}", integer_replacement(128));
    // println!("{}", integer_replacement(194));
    // 100000000
    // println!("{:?}", count_1bit_better(-2147483648));
    // println!("{:?}", count_1bit_better(2147483647));
    println!("{}", integer_replacement(2147483647));
    // println!("{}", integer_replacement(-2147483648));
    // let mut tt = -2147483648;
    // tt  >>= 1;
    // println!("{:?}", count_1bit_better(tt));
    // let test =  2147483647;
    // let t1 = test;
    // for _ in 0..1 {
    //     println!("{}", test + 1);
    // }
    // println!("{}", t1 + 1);
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


pub fn count_1bit_better(mut num: i32) -> (u32, String) {
    let mut rst = 0;
    let mut display_str = String::new();
    for _ in 0..32 {
        if num ^ 0 == 0 {
            break;
        }
        if num & 1 == 1 {
            display_str = format!("1{}", display_str);
            rst += 1;
        } else {
            display_str = format!("0{}", display_str);
        }
        num >>= 1;
    }
    while display_str.len() < 32 {
        display_str = format!("0{}", display_str);
    }
    return (rst, display_str)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn truncate() {
        println!("{}", count_1bit(1));
        println!("{}", count_1bit(-1));
        println!("{}", count_1bit(111));
        println!("{}", count_1bit(-111));
    
        println!("{}", count_1bit_better(1));
        println!("{}", count_1bit_better(-1));
        println!("{}", count_1bit_better(111));
        println!("{}", count_1bit_better(-111));
 
        println!("!0:{}", count_1bit_better(!0));

        println!("!0 + 1: {}", count_1bit_better(!0 + 1));
        let mut t = 1 << 31;
        let mut max = !t;
        println!("max: {}", max);
        println!("!1 << 31: {}", count_1bit_better(!t));

    }

    #[test]
    fn test1() {
        println!("{}", integer_replacement(8));
    }
}