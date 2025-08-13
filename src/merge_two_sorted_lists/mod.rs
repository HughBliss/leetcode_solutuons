#![allow(dead_code)]
use std::vec;

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;

        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            let mut next_node;
            if l1.val <= l2.val {
                next_node = list1.take();
                list1 = next_node.as_mut().unwrap().next.take();
            } else {
                next_node = list2.take();
                list2 = next_node.as_mut().unwrap().next.take();
            }
            current.next = next_node;
            current = current.next.as_mut().unwrap();
        }

        current.next = list1.or(list2);

        dummy_head.next
    }

    pub fn my_first_solulion_merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut v: Vec<i32> = vec![];

        while list1.is_some() || list2.is_some() {
            match list1 {
                Some(ref l1) => match list2 {
                    Some(ref l2) => {
                        if l1.val < l2.val {
                            v.push(l1.val);
                            list1 = l1.next.clone();
                        } else {
                            v.push(l2.val);
                            list2 = l2.next.clone();
                        }
                    }
                    None => {
                        v.push(l1.val);
                        list1 = l1.next.clone();
                    }
                },
                None => match list2 {
                    Some(l2) => {
                        v.push(l2.val);
                        list2 = l2.next;
                    }
                    None => {}
                },
            }
        }

        return Self::vec_to_list(v);
    }

    fn vec_to_list(mut v: Vec<i32>) -> Option<Box<ListNode>> {
        if v.is_empty() {
            return None;
        };

        let mut l = ListNode::new(v.pop().unwrap());
        while !v.is_empty() {
            let mut n = ListNode::new(v.pop().unwrap());
            n.next = Some(Box::new(l));
            l = n;
        }

        Some(Box::new(l))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::merge_two_lists(
            Solution::vec_to_list(vec![1, 2, 4]),
            Solution::vec_to_list(vec![1, 3, 4]),
        );

        assert_eq!(result, Solution::vec_to_list(vec![1, 1, 2, 3, 4, 4]))
    }
}
