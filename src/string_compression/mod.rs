#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        if n < 2 {
            return n as i32;
        }

        let mut write_idx = 0;
        let mut read_idx = 0;

        while read_idx < n {
            let current_char = chars[read_idx];
            let mut count = 0;

            while read_idx < n && chars[read_idx] == current_char {
                read_idx += 1;
                count += 1;
            }

            chars[write_idx] = current_char;
            write_idx += 1;

            if count > 1 {
                for digit in count.to_string().chars() {
                    chars[write_idx] = digit;
                    write_idx += 1;
                }
            }
        }

        write_idx as i32
    }

    pub fn compress_по_дибильному(chars: &mut Vec<char>) -> i32 {
        if chars.len() < 2 {
            return chars.len() as i32;
        }

        let mut prev = chars[0];
        let mut count = 1;
        let mut i = 0;

        for j in 1..chars.len() {
            if prev == chars[j] {
                count += 1;
                continue;
            }
            chars[i] = prev;
            i += 1;
            prev = chars[j];
            if count == 1 {
                continue;
            }
            for count_char in count.to_string().chars() {
                chars[i] = count_char;
                i += 1;
            }
            count = 1;
        }
        chars[i] = prev;
        i += 1;
        if count == 1 {
            return i as i32;
        }
        for count_char in count.to_string().chars() {
            chars[i] = count_char;
            i += 1;
        }

        return i as i32;
    }
}
