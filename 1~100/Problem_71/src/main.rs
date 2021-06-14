mod better_code;
///
/// https://leetcode-cn.com/problems/simplify-path/
/// 
/// examples:
/// /home/
/// /home
/// /
/// /..
/// /...
/// /../
/// "/home//foo/"
/// "/a//b////c/d//././/.."
/// "/a/./b/../../c/"
fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.pop();
    // print!("{}", simplify_path("/home//foo/".to_string()));
    better_code::test_join()
}
pub fn simplify_path(path: String) -> String {
    let mut dir_name: Vec<String> = Vec::new();
    let mut tmp = String::new();
    for c in path.chars() {
        if c == '/' {
            if tmp == "" {
                continue;
            } else if tmp == "." {
                tmp = "".to_string();
            } else if tmp == ".." {
                tmp = "".to_string();
                dir_name.pop();
            } else {
                dir_name.push(tmp.clone());
                tmp = "".to_string();
            }
        } else {
            tmp.push(c);
        }
    }
    if tmp == ".." {
        dir_name.pop();
    } else if tmp != "." && tmp != "" {
        dir_name.push(tmp.clone());
    }
    if dir_name.is_empty() {
        return String::from("/");
    }
    dir_name.into_iter().map(|x: String| {
        let mut t = String::from("/");
        t.push_str(&x);
        t
    }).collect()
    // let mut rst = String::new();
    // for ele in dir_name {
    //     rst.push('/');
    //     rst.push_str(&ele);
    // }
    // rst
}