mod stack_solution;

fn main() {

}


use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution {}

impl  Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut rst: Vec<i32> = Vec::new();
        pre_order(&root, &mut rst);
        rst
    }
}

fn pre_order(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>)  {
    match node {
        Some(curr) => {
            v.push(curr.borrow().val);
            pre_order(&curr.borrow().left, v);
            pre_order(&curr.borrow().right, v);
        }
        None => return
    }
}