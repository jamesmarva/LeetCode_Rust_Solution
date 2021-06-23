fn main() {
    println!("Hello, world!");
}

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,

}

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

/// 102. 二叉树的层序遍历
/// https://leetcode-cn.com/problems/binary-tree-level-order-traversal/
/// 队列解决
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut rst = Vec::new();
    let mut queue = VecDeque::new();
        if let Some(v) = root {
        queue.push_back(v);
    } else {
        return rst;
    }
    while queue.len() > 0 {
        let mut count = queue.len() as isize;
        let mut tmp = Vec::with_capacity(count as usize);
        while count > 0 {
            if let Some(v) = queue.pop_front() {
                tmp.push(v.borrow().val);
                if let Some(v) = v.borrow_mut().left.take() {
                    queue.push_back(Rc::clone(&v));
                }
                if let Some(v) = v.borrow_mut().right.take() {
                    queue.push_back(Rc::clone(&v));
                }
            }
            count -= 1;
        }
        rst.push(tmp);
    }
    rst
}
