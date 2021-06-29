pub fn convert_to_title(column_number: i32) -> String {
    let mut rst = String::new();
    let mut num = column_number;
    while num > 26 {
        let tmp = (num - 1) % 26 + 1;
        rst.push((tmp as u8 - 1 + b'A') as char);
        num = (num - tmp) / 26;
    }
    rst.push((num as u8  - 1 + b'A') as char);
    unsafe {
        let vec = rst.as_mut_vec();
        vec.reverse();
    }
    rst
}