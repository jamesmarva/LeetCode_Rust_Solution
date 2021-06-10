pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    if digits.len() == 0 {
        return digits;
    }
    let mut digits = digits;
    digits.reverse();
    let (mut i, mut carray_one) = (0, false);
    while i < digits.len() {
        let mut tmp = digits[i];
        if i == 0 {
            tmp += 1;
        }
        if carray_one {
            tmp += 1;
        }
        if tmp >= 10 {
            digits[i] = tmp - 10;
            carray_one = true;
        } else {
            digits[i] = tmp;
            carray_one = false;
        }
        i += 1;
    }
    if carray_one {
        digits.push(1);
    }
    digits.reverse();
    digits
}