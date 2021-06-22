mod stack_solution;

use std::rc::Rc;
use std::cell::RefCell;


/// https://leetcode-cn.com/problems/binary-tree-inorder-traversal/
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

/// https://leetcode-cn.com/problems/binary-tree-inorder-traversal/
/// 94. 二叉树的中序遍历
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    inorder_recursion(root, &mut rst);
    rst
}
pub fn inorder_recursion(node: Option<Rc<RefCell<TreeNode>>>, rst: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }
    if let Some(v) = node {
        inorder_recursion(v.borrow_mut().left.take(), rst);
        rst.push(v.borrow().val);
        inorder_recursion(v.borrow_mut().right.take(), rst);
    }
}



fn main() {

}

