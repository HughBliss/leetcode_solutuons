#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
