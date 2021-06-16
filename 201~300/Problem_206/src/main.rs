fn main() {
    println!("Hello, world!");
}
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = Option::None;
    let mut cur = head;
    while let Some(mut tmp) = cur.take() {
        let next = tmp.next.take();
        tmp.next = pre.take();
        pre = Some(tmp);
        cur = next;
    }
    pre
}

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