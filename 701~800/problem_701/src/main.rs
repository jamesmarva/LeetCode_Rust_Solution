/// https://leetcode-cn.com/problems/insert-into-a-binary-search-tree/
/// 701. 二叉搜索树中的插入操作
fn main() {
}
use std::{cell::Ref, rc::Rc};
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
pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
    let mut curr = Rc::clone(root.as_ref().unwrap());
    loop {
        curr = if curr.borrow().val > val {
            let lt = &mut (curr.borrow_mut().left);
            if lt.is_none() {
                *lt = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                break;
            } 
            Rc::clone(lt.as_ref().unwrap())
        } else {
            let rt = &mut curr.borrow_mut().right;
            if rt.is_none() {
                *rt = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                break;
            }
            Rc::clone(rt.as_ref().unwrap())
        };
    }
    root
}

pub fn insert_into_bst0(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
    let mut curr = Rc::clone(root.as_ref().unwrap());
    loop {
        let tmp_curr;
        if curr.borrow().val > val {
            let lt = &mut (curr.borrow_mut().left);
            if lt.is_none() {
                *lt = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                break;
            } 
            tmp_curr = Rc::clone(lt.as_ref().unwrap())
        } else {
            let rt = &mut curr.borrow_mut().right;
            if rt.is_none() {
                *rt = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                break;
            }
            tmp_curr = Rc::clone(rt.as_ref().unwrap());
        }
        curr = tmp_curr;
    }
    root
}



pub fn insert_into_bst1(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    // let mut prev_val = root.as_ref().unwrap().borrow().val;
    // let mut curr = &root;
    // while let Some(node) = curr {
    //     let mut n = node.borrow_mut();
    //     let target = if val > n.val {
    //         &mut n.right
    //     } else {
    //         &mut n.left
    //     };
    //     if target.is_some() {
    //         curr = target;
    //         continue;
    //     } 
    //     *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    //     break;
    // }
    root
}

