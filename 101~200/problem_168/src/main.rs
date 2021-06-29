mod right_code;

fn main() {

    println!("1 {}", right_code::convert_to_title(1));
    println!("26 {}", right_code::convert_to_title(26));
    println!("27 {}", right_code::convert_to_title(27));
    println!("2626 {}", right_code::convert_to_title(2626));
}

pub fn convert_to_title(column_number: i32) -> String {
    let mut rst = String::new();
    if column_number < 0 {
        rst.push_str("-");
    }   
    let mut num = column_number.abs() - 1;
    while num >= 26 {
        let tmp = num % 26;
        rst.push((tmp as u8 + b'A' ) as char);
        num = num / 26;
    }
    rst.push((num as u8 + b'A') as char);
    rst
}













