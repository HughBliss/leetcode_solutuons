#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] == val {
                continue;
            }
            nums[count] = nums[i];
            count += 1;
        }
        count as i32
    }
}
#[cfg(test)]
mod test {
    use crate::remove_element::Solution;

    #[test]
    fn case1() {
        let mut arr = vec![3, 2, 2, 3];
        let val = 3;
        let result = Solution::remove_element(arr.as_mut(), val);
        assert_eq!(result, 2);
        assert_eq!(arr, vec![2, 2, 2, 3])
    }
}
