pub fn simplify_path(path: String) -> String {
    let mut stack: Vec<&str>  = Vec::new();
    let dirs = path.split("/");
    for dir in dirs {
        match dir {
            "." | "" => continue,
            ".." => {
                stack.pop();
            },
            _ => stack.push(dir),
        }
    }
    // let t: String = stack.join("/");
    "".to_string() + &stack.join("/")
}

pub fn test_join() {
    let mut v: Vec<String> = Vec::new();
    v.push("aaa".to_string());
    v.push("bbb".to_string());
    let t: String = v.join("/");

    let mut v1: Vec<&str> = Vec::new();
    v1.push("aaa");
    v1.push("bbb");
    let t1 = v1.join("/");
    assert_eq!(t1, t);
    // println!("{}", v.join("/"));
    // assert_eq!("asdfasfasfdasdf/qwerqwreq", v.join("/"));
    assert_eq!("asdfasdf", "asdfasdf".to_string());
    // assert_eq!((), v.join("/"));
    // let t2: String = ["hello", "world"].join(" ");
    assert_eq!("aaa", "aaa".to_string());
}