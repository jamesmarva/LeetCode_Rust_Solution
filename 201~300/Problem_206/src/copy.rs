use crate::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    // take 相当于进行了一次值的转移，但是没有进行所有权的转移
    // 也就是保留了对于Option所有权，以及类型，
    // 以及注意unwrap() 的使用，因为unwrap的参数是self
    while let Some(mut curr_take) = curr.take() {
        let next = curr_take.next.take();
        curr_take.next = prev;
        prev = Some(curr_take);
        curr = next;
    }
    prev
}

pub fn reverse_list_0(head: Option<Box<ListNode>>) ->Option<Box<ListNode>> {
    let mut prev = Option::None;
    let mut curr = head;
    while curr.is_some() {
       if let Some(mut curr_val) = curr.take() {
           let next = curr_val.next.take();
           curr_val.next = prev;
           prev = Some(curr_val);
           curr = next;
       }
    }
    prev
}