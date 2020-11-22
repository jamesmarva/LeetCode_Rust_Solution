use std::option::Option;
use std::boxed::Box;

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
    let mut dummy = ListNode::new(0);
    dummy.next = h;
    let mut fast = &dummy;
    let mut slow = &dummy;
    while fast.next.is_some() && fast.next.as_ref().unwrap().next.is_some() {
      fast = fast.next.as_ref().unwrap().next.as_ref().unwrap();
      // fast = fast.next.as_ref().unwrap();
      slow = slow.next.as_ref().unwrap();
    }
    let mut left = &*(dummy.next.as_ref().unwrap());
    let mut right = &*(slow.next.as_ref().unwrap());
    
    None
}

fn main() {

}
