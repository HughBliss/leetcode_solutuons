#![allow(dead_code)]

use std::collections::HashMap;

/// LeetCode Problem 20: Valid Parentheses
///
/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
/// determine if the input string is valid.
///
/// An input string is valid if:
/// 1. Open brackets must be closed by the same type of brackets.
/// 2. Open brackets must be closed in the correct order.
/// 3. Every close bracket has a corresponding open bracket of the same type.

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut store = HashMap::new();
        store.insert('{', '}');
        store.insert('[', ']');
        store.insert('(', ')');

        let open_parens = "{[(";

        let mut open_parens_stack: Vec<char> = Vec::new();

        for ele in s.as_bytes() {
            if open_parens.contains(*ele as char) {
                open_parens_stack.push(*ele as char);
                continue;
            }
            let last_open_paren = match open_parens_stack.last() {
                Some(v) => v,
                None => return false,
            };

            let expected_close_paren = match store.get(last_open_paren) {
                Some(v) => v,
                None => return false,
            };

            if *ele as char != *expected_close_paren {
                return false;
            }
            open_parens_stack.pop();
        }

        if open_parens_stack.len() != 0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }

    #[test]
    fn test_example_6() {
        assert_eq!(Solution::is_valid("{".to_string()), false);
    }
}
