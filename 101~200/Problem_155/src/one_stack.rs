pub struct MinStack {
    all: Vec<i32>,
    min: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    pub fn new() -> Self {
        MinStack {
            all: Vec::new(),
            min: std::i32::MAX,
        }
    }
    
    pub fn push(&mut self, val: i32) {
        if self.all.is_empty() {
            self.min = val;
        } else if self.min >= val {
            // push previous min.
            self.all.push(self.min);
            self.min = val;
        }
        self.all.push(val);
    }
    
    pub fn pop(&mut self) {
        if let Some(x) = self.all.pop() {
            if x == self.min {
                self.min = self.all.pop().unwrap_or(std::i32::MAX);
            }
        }
    }
    
    pub fn top(&self) -> i32 {
        if let Some (v) = self.all.last() {
            *v
        } else {
            std::i32::MAX
        }
    }
    
    pub fn get_min(&self) -> i32 {
        self.min
    }
}