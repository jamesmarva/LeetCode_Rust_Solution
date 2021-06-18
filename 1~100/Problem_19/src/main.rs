fn main() { 
    let mut n0 = ListNode::new(0);
    // println!("n0 address: {:p}", &n0);
    let n1 = ListNode::new(1);
    // println!("n1 address: {:p}", &n1);
    n0.next = Some(Box::new(n1));
    // println!("&n0.next address: {:p}", &n0.next);
    // println!("&(n0.next) address: {:p}", &(n0.next));
    // println!("(&n0).next address: {:p}", &(&n0).next);
    // node.next = Some(Box::new(ListNode::new(1)));
    let n0_next = n0.next.as_ref().unwrap(); // 不论过程，只论类型的结果
    println!("n0_next {}", n0_next.val);
    // println!("n0_next address: {:p}", n0_next);
    // println!("*n0_next address: {:p}", *n0_next);
    // println!("&(**n0_next) address: {:p}", &(**n0_next));

    let n0_clone = n0.clone();
    // println!("n0_clone address: {:p}", &n0_clone);
    // println!("{:p}", &mut node);
    // println!("{:p}, next: {:p}", &node_clone, &node_clone.next);
    // println!("{:p}", &vec![1]);
    n0.next.as_mut().unwrap().val = 444;
    println!("n0 next{} ", n0.next.as_ref().unwrap().val);
    // println!("{}", n0_next.val);

    println!("n0clone next: {}", n0_clone.next.as_ref().unwrap().val);
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32, 
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}
struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let mut fast = dummy.clone();
        let mut slow = &mut dummy;
        for _ in 0..n {
            fast = fast.unwrap().next;
        }
        while fast.as_ref().unwrap().next.is_some() {
            fast = fast.unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.clone(); 
        dummy.unwrap().next
    }
}
