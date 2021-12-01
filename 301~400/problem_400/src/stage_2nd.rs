use std::collections::HashMap;
use std::i32::{MAX as max_i32};


pub fn find_nth_digit(n: i32) -> i32 {
    let mut digits_num = HashMap::new();
    digits_num.insert(0u32, 0i32);
    for i in 1..9 {
        let pre_tmp = digits_num.get(&(i - 1)).unwrap();
        let cur_val = i as i32 * 9 * 10i32.pow(i - 1);
        digits_num.insert(i, cur_val + pre_tmp);
    }
    let mut max_digit = 8;
    if *digits_num.get(&max_digit).unwrap() != max_i32 {
        max_digit = max_digit + 1;
        digits_num.insert(max_digit, max_i32);
    }
    let mut th_max_i = 1;
    for i in 0..max_digit {
        if n > *digits_num.get(&i).unwrap() && n <= *digits_num.get(&(i + 1)).unwrap() {
            th_max_i = i;
            break;
        } 
    }
    let after = n - *digits_num.get(&th_max_i).unwrap() - 1;
    let begin = 10i32.pow(th_max_i);
    let add_num = after / (th_max_i as i32 + 1);
    let place_th = after % (th_max_i as i32 + 1);
    (begin + add_num).to_string().chars().nth(place_th as usize).unwrap().to_digit(10).unwrap() as i32
}

pub fn find_nth_digit_right(n: i32) -> i32 {
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
    let mut curr = 0;
    let mut bit_num = 0;
    for (k, v) in arr.iter() {
        if n <= *k {
            curr = *k;
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
    println!("num: {}", &num);

    let s = (10i32.pow(bit_num as u32) + num).to_string();
    println!("s {}", s);
    let th = dis  % (bit_num + 1);
    let s: Vec<char> = s.chars().collect();
    (s[th as usize] as u8 - b'0') as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // println!("{}", i32::MAX);
        println!("d {}", find_nth_digit(222));
        println!("d {}", find_nth_digit_right(222));

        // println!("d {}", find_nth_digit(10));
        // println!("d {}", find_nth_digit(11));
    }

}