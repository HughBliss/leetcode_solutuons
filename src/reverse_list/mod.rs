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
// Функция для печати списка, чтобы тебе было удобнее тестировать
fn print_list(list: &Option<Box<ListNode>>) {
    let mut current = list.as_ref();
    print!("[");
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next.as_ref();
    }
    println!("None]");
}

/// Основная функция, которую тебе нужно реализовать
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut current = head;
    while let Some(mut node) = current.take() {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        println!("Original list:");
        print_list(&list);

        let reversed_list = reverse_list(list);
        println!("\nReversed list:");
        print_list(&reversed_list);

        // Ожидаемый вывод:
        // Original list:
        // [1 -> 2 -> 3 -> 4 -> 5 -> None]
        //
        // Reversed list:
        // [5 -> 4 -> 3 -> 2 -> 1 -> None]
    }
}
