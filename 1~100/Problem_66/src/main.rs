mod reverse_plus_one;

fn main() {
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    if digits.len() == 0 {
        return digits;
    }
    let (mut i, mut carry_one) = (digits.len() as i32 - 1 , false);
    let mut rst: Vec<i32> = digits;
    while i >= 0 {
        let mut tmp = rst[i as usize];
        if i == rst.len() as i32  - 1 {
            tmp += 1 ;
        }
        if carry_one {
            tmp += 1;
        }
        if tmp >= 10 {
            rst[i as usize] = tmp - 10;
            carry_one = true;
        } else {
            rst[i as usize] = tmp;
            carry_one = false;
        }
        i -= 1;
    }
    if carry_one {
        rst.insert(0, 1);
    }
    rst
}

