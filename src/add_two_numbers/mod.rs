#![allow(dead_code)]

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

    fn from_vec(v: Vec<i32>) -> Option<Box<Self>> {
        let mut current = None;
        for &val in v.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = current;
            current = Some(new_node);
        }
        current
    }
}
struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));

        let mut list_1_iterator = l1.as_ref();
        let mut list_2_iterator = l2.as_ref();
        let mut list_3_iterator = dummy_head.as_mut();

        let mut addition: i32 = 0;
        let dummy_node = Box::new(ListNode::new(0));
        while true {
            let (node1, node2) = match (list_1_iterator, list_2_iterator) {
                (Some(n1), Some(n2)) => (n1, n2),
                (None, Some(n2)) => (&dummy_node, n2),
                (Some(n1), None) => (n1, &dummy_node),
                (None, None) => break,
            };
            let result = node1.val + node2.val + addition;
            addition = result / 10;

            if let Some(node3) = list_3_iterator {
                node3.next = Some(Box::new(ListNode::new(result % 10)));
                list_3_iterator = node3.next.as_mut();
            }

            list_1_iterator = node1.next.as_ref();
            list_2_iterator = node2.next.as_ref();
        }

        if addition != 0 {
            if let Some(node) = list_3_iterator {
                node.next = Some(Box::new(ListNode::new(addition)))
            }
        }

        return dummy_head.unwrap().next;
    }
}

#[cfg(test)]
mod test {
    use crate::add_two_numbers::{ListNode, Solution};

    #[test]
    fn case1() {
        let result = Solution::add_two_numbers(
            ListNode::from_vec(vec![2, 4, 5, 5]),
            ListNode::from_vec(vec![5, 6, 4]),
        );

        assert_eq!(result, ListNode::from_vec(vec![7, 0, 0, 6]))
    }
}
