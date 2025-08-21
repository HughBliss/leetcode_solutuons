#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut zeros: i64 = 0;

        let mut founded = 0;

        for num in nums {
            if num == 0 {
                founded += 1;
                zeros += founded;
                continue;
            }
            founded = 0;
        }

        zeros
    }
}
