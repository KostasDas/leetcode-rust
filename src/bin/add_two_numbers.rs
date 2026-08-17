struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut carry = 0;
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        while p1.is_some() || p2.is_some() {
            let d1 = p1.as_ref().map_or(0, |node| node.val);
            let d2 = p2.as_ref().map_or(0, |node| node.val);

            let mut result = carry + d1 + d2;
            carry = 0;
            if result >= 10 {
                result -= 10;
                carry = 1;
            }
            tail.next = Some(Box::new(ListNode::new(result)));
            tail = tail.next.as_mut().unwrap();
            if let Some(node) = p1 {
                p1 = &node.next;
            }
            if let Some(node) = p2 {
                p2 = &node.next;
            }
        }
        if carry == 1 {
            tail.next = Some(Box::new(ListNode::new(carry)));
        }
        dummy.next
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(3))),
        })),
    }));

    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));

    let res = Solution::add_two_numbers(l1, l2);
    println!("{:?}", res);
}
