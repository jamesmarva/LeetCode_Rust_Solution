use crate::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut i1 = l1;
    let mut i2 = l2;
    let mut dummy = ListNode::new(0);
    let mut i0 = &mut dummy;
    let mut carry = false;
    while i1.is_some() || i2.is_some() {
        let mut tmp = 0i32;
        if let Some(curr) = i1.take() {
            tmp += curr.val;
            i1 = curr.next;
        }
        if let Some(curr) = i2.take() {
            tmp += curr.val;
            i2 = curr.next;
        }
        if carry {
            tmp += 1;
        } 
        if tmp >= 10 {
            tmp -= 10;
            carry = true;
        } else {
            carry = false;
        }
        i0.next = Some(Box::new(ListNode::new(tmp)));
        i0 = i0.next.as_mut().unwrap();
    }
    if carry {
        i0.next = Some(Box::new(ListNode::new(1)));
    }
    dummy.next
}