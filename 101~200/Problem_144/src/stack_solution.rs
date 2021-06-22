use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

/// use stack   
/// 
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst: Vec<i32> = Vec::new();
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut curr = root;
    while curr.is_some() || stack.len() > 0 {
        while let Some(rc_val) = curr {
            rst.push(rc_val.borrow().val);
            stack.push(rc_val.clone());
            curr = rc_val.borrow().left.clone();
        }
        if let Some(rc_val) = stack.pop() {
            curr = rc_val.borrow().right.clone();
        }
    }
    rst
}

pub fn preorder_traversal0(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    if root.is_none() {
        return rst;
    }
    let mut curr = root;
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    while stack.len() > 0 || curr.is_some() {
        while let Some(n) = curr {
            rst.push(n.borrow().val); 
            curr = n.borrow_mut().left.take();// &mut Option -> Option<T>
            stack.push(n);
        }
        if let Some(n) = stack.pop() {
            curr = n.borrow_mut().right.take();
        }
    }
    rst
}
