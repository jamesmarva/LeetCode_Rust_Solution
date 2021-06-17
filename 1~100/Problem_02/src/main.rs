fn main() {

    
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut i1 = l1;
    let mut i2 = l2;
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut i = &mut dummy;
    let mut carry_one = false;
    while i1.is_some() && i2.is_some() {
        let mut tmp = 0;
        if let Some(ref x) = i1 {
            tmp += x.val;
        }
        if let Some(ref x) = i2 {
            tmp += x.val;
        }
        if carry_one {
            tmp += 1;
        }
        if tmp >= 10 {
            tmp -= 10;
            carry_one = true;
        } else {
            carry_one = false;
        }
        i.as_mut().unwrap().next = Some(Box::new(ListNode::new(tmp)));
        i = &mut i.as_mut().unwrap().next;
        i1 = i1.take().unwrap().next;
        i2 = i2.take().unwrap().next;
    }
    while i1.is_some() {
        let mut tmp = 0;
        if let Some(ref x) = i1 {
            tmp += x.val;
        }
        if carry_one {
            tmp += 1;
        }
        if tmp >= 10 {
            tmp -= 10;
            carry_one = true;
        } else {
            carry_one = false;
        }
        i.as_mut().unwrap().next = Some(Box::new(ListNode::new(tmp)));
        i1 = i1.take().unwrap().next;
    }
    while i2.is_some() {
        let mut tmp = 0;
        if let Some(ref x) = i2 {
            tmp += x.val;
        }
        if carry_one {
            tmp += 1;
        }
        if tmp >= 10 {
            tmp -= 10;
            carry_one = true;
        } else {
            carry_one = false;
        }
        i.as_mut().unwrap().next = Some(Box::new(ListNode::new(tmp)));
        i2 = i2.take().unwrap().next;
    }
    if carry_one {
        i.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    }
    dummy.unwrap().next
}



pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}
