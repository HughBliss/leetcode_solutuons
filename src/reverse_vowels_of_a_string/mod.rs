#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: [u8; 10] = [
            'a' as u8, 'e' as u8, 'i' as u8, 'o' as u8, 'u' as u8,
            'A' as u8, 'E' as u8, 'I' as u8, 'O' as u8, 'U' as u8,
        ];

        let mut result: Vec<u8> = Vec::with_capacity(s.capacity());

        let mut i = s.len();
        let chars = s.as_bytes();
        for ch in chars.iter() {
            if !vowels.contains(ch) {
                result.push(*ch);
                continue;
            }
            while !vowels.contains(&(chars[i - 1])) {
                i -= 1;
            }
            result.push(chars[i - 1]);
            i -= 1;
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::reverse_vowels_of_a_string::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_vowels("IceCreAm".to_string()),
            "AceCreIm".to_string()
        )
    }
}
