
use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;


/// 94. 二叉树的中序遍历
/// https://leetcode-cn.com/problems/binary-tree-inorder-traversal/
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;
    while curr.is_some() || stack.len() > 0 {
        while let Some(v) = curr {
            stack.push(Rc::clone(&v));
            curr = v.borrow_mut().left.take();
        }
        curr = stack.pop();
        if let Some(v) = curr {
            rst.push(v.borrow().val);
            curr = v.borrow_mut().right.take();
        }
    }
    rst
}