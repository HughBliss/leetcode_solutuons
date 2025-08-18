#![allow(dead_code)]

struct Solution;

use std::cmp::min;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let string_slice: &str = s.as_str();
        let mut longest: &str = "";
        let bts: &[u8] = string_slice.as_bytes();
        for i in 0..bts.len() - 1 {
            for even_odd in 0..=1 {
                let mut tmp: &str = "";
                for j in 0..=min(i, bts.len() - i - 1 - even_odd) {
                    let start = i - j;
                    let end = i + j + even_odd;
                    if bts[start] != bts[end] {
                        break;
                    }
                    tmp = &string_slice[start..=end]
                }
                if longest.len() < tmp.len() {
                    longest = tmp;
                }
            }
        }
        longest.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::longest_palindromic_substring::Solution;

    #[test]
    fn case1() {
        let res = Solution::longest_palindrome("babad".to_string());

        assert_eq!(res, "bab".to_string())
    }
    #[test]
    fn case2() {
        let res = Solution::longest_palindrome("cbbd".to_string());

        assert_eq!(res, "bb".to_string())
    }
    #[test]
    fn case3() {
        let res = Solution::longest_palindrome("aacabdkacaa".to_string());

        assert_eq!(res, "aca".to_string())
    }

    #[test]
    fn case4() {
        let res = Solution::longest_palindrome("a".to_string());

        assert_eq!(res, "a".to_string())
    }
    #[test]
    fn case5() {
        let res = Solution::longest_palindrome("aaaa".to_string());

        assert_eq!(res, "aaaa".to_string())
    }
}
