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

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_with_carry(l1, l2, 0)
    }

    fn add_with_carry(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                // Base case: if carry remains, create a final node
                if carry > 0 {
                    Some(Box::new(ListNode::new(carry)))
                } else {
                    None
                }
            }
            (l1_node, l2_node) => {
                // Sum current nodes + carry
                let sum = carry
                    + l1_node.as_ref().map_or(0, |n| n.val)
                    + l2_node.as_ref().map_or(0, |n| n.val);

                // Move to next nodes
                let next1 = l1_node.and_then(|n| n.next);
                let next2 = l2_node.and_then(|n| n.next);

                // Recursive call with new carry
                let next = Self::add_with_carry(next1, next2, sum / 10);

                // Create new node for current sum digit
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next,
                }))
            }
        }
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    println!("{:?}", Solution::add_two_numbers(l1, l2));
}
