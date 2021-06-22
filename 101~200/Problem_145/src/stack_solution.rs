use std::{env::var, rc::Rc};
use std::cell::RefCell;
use crate::TreeNode;

pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut rst = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;
    while curr.is_some() || stack.len() > 0 {
        while let Some(val) = curr {
            stack.push(Rc::clone(&val));
            curr = val.borrow_mut().left.take();
        }
        curr = stack.pop();
        if let Some(v) = curr {
            rst.push(v.borrow().val);
            curr = v.borrow_mut().right.take();
        }
    }
    rst
}