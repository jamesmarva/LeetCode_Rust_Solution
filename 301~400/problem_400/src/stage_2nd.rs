use std::collections::HashMap;
use std::i32::{MAX as max_i32};


pub fn find_nth_digit(n: i32) -> i32 {
    let mut digits_num = HashMap::new();
    digits_num.insert(0u32, 0i32);
    let mut max_digit = 1;
    for i in 1..9 {
        let pre_tmp = digits_num.get(&(i - 1)).unwrap();
        let cur_val = i as i32 * 9 * 10i32.pow(i - 1);
        if max_i32 - cur_val < *pre_tmp {
            max_digit = i;
            break;
        }
        println!("{}", cur_val + pre_tmp);
        digits_num.insert(i, cur_val + pre_tmp);
    }
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
    (begin + add_num).to_string().chars().nth(place_th as usize).unwrap().to_digit(10).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // println!("{}", i32::MAX);
        println!("d {}", find_nth_digit(222));
    }

}