#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.capacity());

        let mut buffer = 1;
        for i in 0..nums.len() {
            result.push(buffer);
            buffer *= nums[i];
        }
        buffer = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= buffer;
            buffer *= nums[i];
        }

        result
    }
}
