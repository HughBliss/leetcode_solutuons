#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap_or(&0);

        let mut result: Vec<bool> = Vec::with_capacity(candies.len());

        for c in candies.iter() {
            result.push(c + extra_candies >= *max)
        }

        result
    }
}
