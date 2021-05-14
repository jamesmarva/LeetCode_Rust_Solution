#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
struct Solution {}

impl Solution {
    pub fn merge_two_Lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if (l1.is_none()) {
            return l2
        } else if (l2.is_none()) {
            return l1
        }

        let mut dummy = ListNode::new(0);
        let mut i0 = &mut dummy;
        let mut i1 = &l1;
        let mut i2 = &l2;
        // let i1 : Box<ListNode> = l1.unwrap();
        // let i2 : Box<ListNode> = l2.unwrap();
        while i1.is_some() && i2.is_some() {
            if i1.as_ref().unwrap().val >  i2.as_ref().unwrap().val {
                (i0).next = Some(Box::new(ListNode{
                    val: i2.as_ref().unwrap().val,
                    next: None
                }));
                i0 = i0.next.as_mut().unwrap();
                
            } else {
                
            }
        }

        return dummy.next

    }
    
}

pub fn merge_two_Lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2
    } else if l2.is_none() {
        return l1
    }
    let mut dummy = ListNode::new(0);
    let mut i0 = &mut dummy;
    let mut i1: &ListNode = l1.as_ref().unwrap();
    let mut i2: &ListNode = l2.as_ref().unwrap();
    while !i1.next.is_none() || !i2.next.is_none() {
        if i1.val < i2.val {
            (*i0).next = Some(Box::new(ListNode {
                val: i1.val,
                next: None
            }));
            i1 = i1.next.as_ref().unwrap();
        } else {
            (*i0).next = Some(Box::new(ListNode {
                val: i2.val,
                next: None,
            }));
            i2 = i2.next.as_ref().unwrap();
        }
        i0 = i0.next.as_mut().unwrap();
    }
    while !i1.next.is_none() {
        i0.next = Some(Box::new(ListNode {
            val: i1.val,
            next: None
        }));
        i0 = i0.next.as_mut().unwrap();
        i1 = i1.next.as_ref().unwrap();
    }

    while i2.next.is_some() {
        i0.next = Some(Box::new(ListNode {
            val: i2.val,
            next: None
        }));
        i0 = i0.next.as_mut().unwrap();
        i2 = i2.next.as_ref().unwrap();
    }
    return dummy.next
}


// 如果有了结构体的所有权，那么就有了结构体里面的成员变量的所有权
fn main() {
    println!("Hello, world!");
}



