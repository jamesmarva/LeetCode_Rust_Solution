
fn main() {
    println!("Hello, world!");
}

/// n & (n−1): 把 n 的二进制位中的最低位的 1 变为 0 。
/// 因为如果末尾没有 1，足够被减，那么就会进行借位。
/// 00001110: 00001110 & 00001101 = 00001100
pub fn hammingWeight (n: u32) -> i32 {
    let mut rst = 0;
    let mut n = n;
    while n != 0 {
        n = n & (n - 1);
        rst += 1;
    }
    rst
}
