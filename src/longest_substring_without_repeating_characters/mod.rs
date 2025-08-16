#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let byts = s.as_bytes();
        let mut longest = 0;
        let mut start = 0;
        for (end, ch) in byts.iter().enumerate() {
            for i in start..end {
                if *ch != byts[i] {
                    continue;
                }
                if longest < end - start {
                    longest = end - start;
                }
                start = i + 1;
            }
        }
        if longest < byts.len() - start {
            longest = byts.len() - start;
        }

        return longest as i32;
    }
    pub fn length_of_longest_substring_dumb_solution(s: String) -> i32 {
        let mut longest = 0;
        let mut substr = String::new();
        for c in s.chars() {
            if substr.contains(c) {
                if substr.len() > longest {
                    longest = substr.len();
                }
                substr = c.to_string();
                continue;
            }
            substr.push(c);
        }
        if substr.len() > longest {
            longest = substr.len();
        }
        return longest as i32;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn case1() {}
}
