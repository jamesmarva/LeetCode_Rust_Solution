///
/// 
/// 
/// 
/// [57,42,21,28,30,25,22,12,55,3,47,18,43,29,20,44,59,9,43,7,8,5,42,53,99,34,37,88,87,62,38,68,31,3,11,61,93,34,63,27,20,48,38,5,71,100,88,54,52,15,98,59,74,26,81,38,11,44,25,69,79,81,51,85,59,84,83,99,31,47,31,23,83,70,82,79,86,31,50,17,11,100,55,15,98,11,90,16,46,89,34,33,57,53,82,34,25,70,5,1]
/// [76,3,5,29,18,53,55,79,30,33,87,3,56,93,40,80,9,91,71,38,35,78,32,58,77,41,63,5,21,67,21,84,52,80,65,38,62,99,80,13,59,94,21,61,43,82,29,97,31,24,95,52,90,92,37,26,65,89,90,32,27,3,42,47,93,25,14,5,39,85,89,7,74,38,12,46,40,25,51,2,19,8,21,62,58,29,32,77,62,9,74,98,10,55,25,62,48,48,24,21]
/// [1,7,5]
/// [2,3,5]

fn main() {
    // let arr = vec![1, 2, 34, 55, 66, 777];
    // arr.binary_search(&66);
    // println!("{}", i32::MAX);

    let n1 = vec![57,42,21,28,30,25,22,12,55,3,47,18,43,29,20,44,59,9,43,7,8,5,42,53,99,34,37,88,87,62,38,68,31,3,11,61,93,34,63,27,20,48,38,5,71,100,88,54,52,15,98,59,74,26,81,38,11,44,25,69,79,81,51,85,59,84,83,99,31,47,31,23,83,70,82,79,86,31,50,17,11,100,55,15,98,11,90,16,46,89,34,33,57,53,82,34,25,70,5,1];
    let n2 = vec![76,3,5,29,18,53,55,79,30,33,87,3,56,93,40,80,9,91,71,38,35,78,32,58,77,41,63,5,21,67,21,84,52,80,65,38,62,99,80,13,59,94,21,61,43,82,29,97,31,24,95,52,90,92,37,26,65,89,90,32,27,3,42,47,93,25,14,5,39,85,89,7,74,38,12,46,40,25,51,2,19,8,21,62,58,29,32,77,62,9,74,98,10,55,25,62,48,48,24,21];
    println!("{}", min_absolute_sum_diff(n1, n2));
}


pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let l = nums1.len();
    if l <= 1 {
        return nums1[0] + nums2[0];
    }
    let mut max_abs_sum = (nums1[0] - nums2[0]).abs();
    let mut max_idx_vec = Vec::from([0]);
    for i in 1..nums1.len() {
        let tmp_sum = (nums1[i] - nums2[i]).abs();
        if tmp_sum > max_abs_sum {
            max_abs_sum = tmp_sum;
            max_idx_vec.clear();
            max_idx_vec.push(i);
            println!("i: {}; n1: {}; n2: {}; abs: {}", i, nums1[i], nums2[i], max_abs_sum);
        } else if tmp_sum == max_abs_sum {
            max_idx_vec.push(i);
        }
    }
    let mut arr = nums1.clone();
    arr.sort();
    arr.dedup();
    println!("{:?}", arr);
    println!("max_idx_vec: {:?}", max_idx_vec);
    let mut min_abs_sum = i32::MAX;
    let mut min_idx = 0;
    for i in 0..max_idx_vec.len() {
        let tmp_num = nums2[max_idx_vec[i]];
        let mut lt = 0;
        let mut rt = arr.len();
        let mut tmp_rst = 0;
        while lt < rt {
            let m = lt + ((rt - lt) >> 1);
            if m + 1 < rt && (tmp_num - arr[m]).abs() > (tmp_num - arr[m + 1]).abs() {
                lt = m + 1
            } else if m == rt - 1 || (tmp_num - arr[m]).abs() == 0 {
                tmp_rst = m;
                break;
            } else if (tmp_num - arr[m]).abs() < (tmp_num - arr[m + 1]).abs() {
                rt = m;
            }
        }
        if min_abs_sum > (tmp_num - arr[tmp_rst]).abs() {
            min_abs_sum = (tmp_num - arr[tmp_rst]).abs();
            min_idx = max_idx_vec[i];
        }
    }
    println!("min_idx: {}", min_idx);
    println!("min_abs_sum: {}", min_abs_sum);
    let mut rst = 0;
    for i in 0..nums1.len() {
        if i == min_idx {
            rst += min_abs_sum;
            rst %= 1000_000_000 + 7;
        } else {
            rst += (nums1[i] - nums2[i]).abs();
            rst %= 1000_000_000 + 7;
        }
    }
    rst
}

fn binary_search(arr: Vec<i32>, tmp_num: i32) -> usize {
    let mut lt = 0;
    let mut rt = arr.len();
    let mut tmp_rst = 0;
    while lt < rt {
        let m = lt + ((rt - lt) >> 1);
        if m + 1 < rt && (tmp_num - arr[m]).abs() >= (tmp_num - arr[m + 1]).abs() {
            lt = m + 1
        } else if m == rt - 1 {
            return m;
        } else if (tmp_num - arr[m]).abs() < (tmp_num - arr[m + 1]).abs() {
            rt = m;
        }
    }
    return 0;
}