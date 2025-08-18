#![allow(dead_code)]

struct Solution;

use std::cmp::max;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = "".to_string();

        let end = max(word1.len(), word2.len());

        let mut ch1 = word1.chars();
        let mut ch2 = word2.chars();

        for _ in 0..end {
            if let Some(ch) = ch1.next() {
                result.push(ch);
            }
            if let Some(ch) = ch2.next() {
                result.push(ch);
            }
        }

        result
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn case1() {}
}
