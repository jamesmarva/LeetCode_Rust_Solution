mod one_stack;
fn main() {
    let mut demo = one_stack::MinStack::new();
    demo.push(-1);
    demo.push(0);
    demo.push(-111);
    println!("{}", demo.get_min());
    demo.pop();
    println!("{}", demo.top());
    println!("{}", demo.get_min());

}
/// 双栈的实现
/// 
struct MinStack {
    all: Vec<i32>,
    min_vals: Vec<i32>,
}
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            all: Vec::new(),
            min_vals: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.all.push(val);
        if self.min_vals.is_empty() || *self.min_vals.last().unwrap() >= val {
            self.min_vals.push(val)
        }
    }
    
    fn pop(&mut self) {
        if let Some(x) = self.all.pop() {
            if x == *self.min_vals.last().unwrap() {
                self.min_vals.pop();
            }
        }
    }
    
    fn top(&self) -> i32 {
        *self.all.last().expect("min stack is empty!!!")
    }
    
    fn get_min(&self) -> i32 {
        *self.min_vals.last().expect("min stack is empty!!!")
    }
}