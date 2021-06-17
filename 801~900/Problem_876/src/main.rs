fn main() {

}
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    let test = slow.as_ref().unwrap().next;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow.clone()
}

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