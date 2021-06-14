fn main() {
    
}

pub fn is_valid(s: String) -> bool {
    let chars = s.chars();
    let mut stack: Vec<char> = Vec::new();
    for c in chars {
        if stack.is_empty() {
            stack.push(c);
        } else {
            if c == ')' {
                if '(' != stack.pop().unwrap() {
                    return false;
                }
            } else if c == ']' {
                if '[' != stack.pop().unwrap() {
                    return false;
                }
            } else if c == '}' {
                if '{' != stack.pop().unwrap() {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
    }
    stack.is_empty()
}