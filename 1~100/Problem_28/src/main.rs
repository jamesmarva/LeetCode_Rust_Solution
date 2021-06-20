fn main() {

}
struct Solution{}

impl Solution {
    pub fn str_str(src: String, mat: String) -> i32 {
        let src_arr: Vec<char> = src.chars().collect();
        let mat_arr: Vec<char> = mat.chars().collect();
        if mat_arr.len() == 0 {
            return 0;
        }
        let next_arr: Vec<isize> = get_next_array(&mat_arr);
        let (mut i1, mut i2) = (0, 0);
        while i1 < src_arr.len() && i2 < mat_arr.len() {
            if src_arr[i1] == mat_arr[i2] {
                i1 += 1;
                i2 += 1;
            // note: now i2 is zero 
            } else if next_arr[i2] == -1 {
                i1 +=1;
            } else {
                i2 = next_arr[i2] as usize;
            }
        }
        if i2 == mat.len() {
            (i1 - i2) as i32
        } else {
            -1
        }
    }
    
}
fn get_next_array(mat: &Vec<char>) -> Vec<isize> {
    if mat.len() == 1 {
        return vec![-1];
    }
    let mut rst: Vec<isize> = vec![0; mat.len()];
    rst[0] = -1;
    rst[1] = 0;
    for i in 2..mat.len() {
        let mut pre_idx = rst[i - 1];
        loop {
            if pre_idx != -1 && mat[pre_idx as usize] == mat[i - 1] {
                rst[i] = pre_idx + 1;
                break;
            } else {
                pre_idx = rst[pre_idx as usize];
                if pre_idx == -1 {
                    rst[i] = 0;
                    break;
                }
            }
        }
    }
    rst
}
 
