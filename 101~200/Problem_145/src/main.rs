mod stack_solution;
fn main() {
    println!("Hello, world!");
}
use std::rc::Rc;
use std::cell::RefCell;
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    inorder(root, &mut rst);
    rst
}

fn inorder(node: Option<Rc<RefCell<TreeNode>>>, rst: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }
    let n = node.unwrap();
    inorder(n.borrow_mut().left.take(),rst);
    rst.push(n.borrow().val);
    inorder(n.borrow_mut().right.take(),rst);
}

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}