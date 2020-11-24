use std::option::Option;
use std::boxed::Box;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }    
    None
}

fn merge_sort(h: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if h.is_none() || h.as_ref().unwrap().next.is_none() {
        return h;
    }
    let mut dummy = Rc::new(ListNode::new(0));
    dummy.next = h;
    let mut fast = Rc::clone(&dummy);
    let mut slow = Rc::clone(&dummy);
    let mut s = &dummy;
    while fast.next.is_some() && fast.next.as_ref().unwrap().next.is_some() {
      // fast = Rc::fast.next.as_ref().unwrap().next.as_mut().unwrap();
      // fast = fast.next.as_ref().unwrap();
      // slow = Rc::new(ListNode::new(slow.next.as_ref().unwrap().val));
      // slow = Rc::new(*(slow.next.as_ref().unwrap().as_ref()));
    }
    let mut part1 = &*(dummy.next.as_ref().unwrap());
    let mut part2 = &*(slow.next.as_ref().unwrap());
    slow.next = None;

    
    None
}

fn merge_core(i1: &mut ListNode, i2: &mut ListNode) -> ListNode {
  let mut dummy = ListNode::new(0);
  let mut idx = &mut dummy;
  while i1.next.as_ref().is_some() && i2.next.as_ref().is_some() {
    if i1.val <= i2.val {
      (*idx).next = Some(Box::new(i1.clone()));
      idx = idx.next.as_mut().unwrap();
      i1 = i1.next.as_mut().unwrap();
    } else {

    }
  }
  dummy
}

fn main() {

}
