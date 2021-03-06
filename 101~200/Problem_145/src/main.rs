mod stack_solution;
/// 145. 二叉树的后序遍历
/// https://leetcode-cn.com/problems/binary-tree-postorder-traversal/


fn main() {
    println!("Hello, world!");
}
use std::rc::Rc;
use std::cell::RefCell;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    postorder(root, &mut rst);
    rst
    
}

pub fn postorder(node: Option<Rc<RefCell<TreeNode>>>, rst: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }
    if let Some(v) = node {
        postorder(v.borrow_mut().left.take(), rst);
        postorder(v.borrow_mut().right.take(), rst);
        rst.push(v.borrow().val);
    }
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