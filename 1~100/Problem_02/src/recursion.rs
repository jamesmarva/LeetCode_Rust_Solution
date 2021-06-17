use crate::ListNode;

pub struct Solution{}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut i1), Some(i2)) => {
                let mut tmp = &i1.val + &i2.val;
                if tmp > 10 {
                    tmp -= 10;
                    if i1.next.is_none() {
                        i1.next = Some(Box::new(ListNode::new(1)));
                    } else {
                        i1.next.as_mut().unwrap().val += 1;
                    }
                } 
                let mut rst = Some(Box::new(ListNode::new(tmp)));
                rst.as_mut().unwrap().next = Solution::add_two_numbers(i1.next, i2.next);
                return rst;
            },
            (Some(mut i1), None) => {
                let mut tmp = i1.val;
                if tmp >= 10 {
                    tmp -= 10;
                    if i1.next.is_none() {
                        i1.next = Some(Box::new(ListNode::new(1)));
                    } else {
                        i1.next.as_mut().unwrap().val += 1;
                    }
                } 
                let mut rst = Some(Box::new(ListNode::new(tmp)));
                rst.as_mut().unwrap().next = Solution::add_two_numbers(i1.next, None);
                return rst;
            },
            (None, Some(mut i2)) => {
                let mut tmp = i2.val;
                if tmp >= 10 {
                    tmp -= 10;
                    if i2.next.is_none() {
                        i2.next = Some(Box::new(ListNode::new(1)));
                    } else {
                        i2.next.as_mut().unwrap().val += 1;
                    }
                }
                let mut rst = Some(Box::new(ListNode::new(tmp)));
                rst.as_mut().unwrap().next = Solution::add_two_numbers(None, i2.next);
                return rst;
            },
            (None, None) => None
        }
    }
}
