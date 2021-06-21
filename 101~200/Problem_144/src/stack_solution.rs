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

pub fn pre_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut curr = &root;
    let mut rst: Vec<i32> = Vec::new();
    while curr.is_some() || stack.len() > 0 {
        while curr.is_some() {
            rst.push(curr.as_ref().unwrap().borrow().val);
            stack.push(Rc::clone(curr.as_ref().unwrap().borrow().left.as_ref().unwrap()));
            let tmp_left = &curr.as_ref().unwrap().borrow().left;
            curr = tmp_left;
        }
    } 
    rst
}
