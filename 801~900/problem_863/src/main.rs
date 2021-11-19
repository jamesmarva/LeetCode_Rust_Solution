fn main() {
    println!("Hello, world!");
}
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self { 
        TreeNode { val, left: None, right: None}
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
///
/// [0,2,1,null,null,3]
/// 3
/// 3
///
pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
    let mut queue = VecDeque::new();
    let mut father_map = HashMap::new();
    queue.push_back(root.unwrap().clone());
    while queue.len() > 0 {
        let mut count = queue.len();
        while count > 0 {
            if let Some(val) = queue.pop_front() {
                if let Some(left_child) =  val.borrow().left.as_ref() {
                    queue.push_back(left_child.clone());
                    father_map.insert(left_child.borrow().val, val.clone());
                }
                if let Some(right_child) = val.borrow().right.as_ref() {
                    queue.push_back(right_child.clone());
                    father_map.insert(right_child.borrow().val, val.clone());
                }
            }
            count -= 1;
        }
    }
    let mut rst = vec![];
    let t = target.unwrap();
    dfs(t.clone(), 0, k, &father_map, &mut rst, t.borrow().val);
    rst
}

pub fn dfs(node: Rc<RefCell<TreeNode>>, depth: i32, k: i32, father_map: &HashMap<i32, Rc<RefCell<TreeNode>>>, rst: &mut Vec<i32>, src: i32) {
    if depth == k {
        (*rst).push(node.borrow().val);
        return;
    }

    if let Some(left_child) = node.borrow().left.as_ref() {
        if left_child.borrow().val != src {
            dfs(left_child.clone(), depth + 1, k, father_map, rst, node.borrow().val);
        }
    }

    if let Some(right_child) = node.borrow().right.as_ref() {
        if right_child.borrow().val != src {
            dfs(right_child.clone(), depth + 1, k, father_map, rst, node.borrow().val);
        }
    }

    if let Some(father_node) = father_map.get(&node.borrow().val) {
        if father_node.borrow().val != src {
            dfs(father_node.clone(), depth + 1, k, father_map, rst, node.borrow().val);
        }
    }
}
