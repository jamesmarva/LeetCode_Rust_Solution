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

fn main() {
    println!("Hello, world!");
}



