use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub fn preorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;
    while curr.is_some() || stack.len() > 0 {
        while let Some(v) = curr {
            rst.push(v.borrow().val);
            stack.push(Rc::clone(&v));
            curr = v.borrow_mut().left.take();
        }
        if let Some(v) = stack.pop() {
            curr = v.borrow_mut().right.take();
        }
    }
    rst
}

/// inorder 
pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;
    while curr.is_some() || stack.len() > 0 {
        while let Some(v) = curr {
            stack.push(Rc::clone(&v));
            curr = v.borrow_mut().left.take();
        }
        if let Some(v) = stack.pop() {
            rst.push(v.borrow().val);
            curr = v.borrow_mut().right.take();
        }
    }
    rst
}


pub fn postorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;
    while curr.is_some() || stack.len() > 0 {
        while let Some(v) = curr {
            stack.push(Rc::clone(&v));
            curr = v.borrow_mut().left.take();
        }
        if let Some(v) = stack.pop() {
            if v.borrow().right.is_some() {
                stack.push(Rc::clone(&v));
                curr = v.borrow_mut().right.take();
            } else {
                rst.push(v.borrow().val);
            }
        }
    }
    rst
}
