// https://leetcode-cn.com/problems/add-two-numbers/
fn main() {
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current_1 = l1;
    let mut current_2 = l2;
    let mut tmp_sum = 0;
    let mut head = ListNode::new(0);
    let mut idx = &mut head;
    while current_1.is_some() || current_2.is_some() {
        if current_1.is_some() {
            let tmp_node = *current_1.unwrap();
            tmp_sum += &tmp_node.val;
            current_1 = tmp_node.next;
        }
        if current_2.is_some() {
            let tmp_node = *current_2.unwrap();
            tmp_sum += &tmp_node.val;
            current_2 = tmp_node.next;
        }
        idx.next = Some(Box::new(ListNode::new(tmp_sum % 10)));
        idx = idx.next.as_mut().unwrap();
        tmp_sum /= 10;
    }
    if tmp_sum > 0 {
        idx.next = Some(Box::new(ListNode::new(tmp_sum)));
    }
    return head.next;
}

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new (val: i32) -> Self {
        ListNode {
            val, 
            next: None,
        }
    }
}
