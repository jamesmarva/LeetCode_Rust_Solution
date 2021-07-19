面试题 10.02. 变位词组
https://leetcode-cn.com/problems/group-anagrams-lcci/
# 计数
这里居然可以用数组直接用来作为key，不用转换为String
```rust
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    let mut count = [0; 26];
    for s in strs {
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        let v = map.entry(count).or_insert(Vec::new());
        v.push(s.clone());
        count =  [0; 26];
    }
    map.into_iter().map(|(_, v)| v).collect()
}
```

数组转为String的代码
```rust
pub fn convert_to_str(count: &[i32; 26]) -> String {
    let mut rst = String::new();
    for i in 0..26 {
       rst.push(i as u8 as char); 
       rst.push(' ');
       rst.push(count[i] as u8 as char);
    }
    rst
}
```

# 对每个字符串排序


