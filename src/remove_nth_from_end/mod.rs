#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {}

// pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
// let mut dummy_head: Box<ListNode> = Box::new(ListNode { val: 0, next: head });

// let mut fast: *mut ListNode = dummy_head.as_mut();
// let mut slow: *mut ListNode = dummy_head.as_mut();

// let mut i = 0;
// while let Some(node) = (&mut *fast).next {
//     i += 1;
//     fast = node.as_ref();
//     if i < n {
//         continue;
//     }
//     slow = node.as_mut()
// }
// let node_to_remove = slow.next.take().unwrap();
// slow.next = node_to_remove.next;

// dummy_head.next
// None
// }

pub struct MyList {
    head: Option<Box<ListNode>>,
}

impl MyList {
    pub fn new(vec: Vec<i32>) -> Self {
        // Вспомогательная функция для создания списка
        let mut current = None;
        for &val in vec.iter().rev() {
            current = Some(Box::new(ListNode { val, next: current }));
        }
        MyList { head: current }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref()
        }
        count
    }

    pub fn add_to_all(&mut self, value: i32) {
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            node.val += value;
            current = node.next.as_mut()
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next.as_ref();
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let list = MyList::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(list.len(), 5);

        let empty_list = MyList::new(vec![]);
        assert_eq!(empty_list.len(), 0);
    }

    #[test]
    fn test_add_to_all() {
        let mut list = MyList::new(vec![1, 2, 3]);
        list.add_to_all(10);
        assert_eq!(list.to_vec(), vec![11, 12, 13]);
    }
}
