/// https://leetcode-cn.com/problems/insert-into-a-binary-search-tree/
/// 701. 二叉搜索树中的插入操作

fn main() {
}
use std::{cell::Ref, rc::Rc};
use std::cell::RefCell;
pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut prev_val = root.as_ref().unwrap().borrow().val;
    let mut curr = &mut root;
    while let Some(v) = curr {
        if v.borrow().val < val {
            curr = &mut v.get_mut().right;
        } else {
            curr = &mut v.borrow_mut().right;
        }
    }
    root
}


pub fn insert_into_bst0(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut prev_val = root.as_ref().unwrap().borrow().val;
    let mut curr = &root;
    while let Some(node) = curr {
        let mut n = node.borrow_mut();
        let next = if val > n.val {
            &mut n.right
        } else {
            &mut n.left
        };
    }
    root
}

pub struct TreeNode {
    val: i32, 
    left: Option<Rc<RefCell<TreeNode>>>, 
    right: Option<Rc<RefCell<TreeNode>>>,
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