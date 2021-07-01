fn main() {
}

/// [1,3,2,4,1]
/// 7
/// [10,6,8,7,7,8]
/// 5
///
///

pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
    let mut rst = 0;
    costs.sort_unstable();
    for v in costs {
        if coins >= v {
            coins -= v;
            rst += 1;
        } else {
            return rst;
        }
    }
    rst
}
