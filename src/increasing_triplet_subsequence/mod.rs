#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut candidate1 = nums[0];
        let mut candidate2: Option<i32> = None;

        for i in 1..nums.len() {
            if nums[i] <= candidate1 {
                candidate1 = nums[i];
                // candidate2 = None;
                continue;
            }
            if candidate2 == None {
                candidate2 = Some(nums[i]);
                continue;
            }
            if let Some(c2) = candidate2 {
                if nums[i] <= c2 {
                    candidate2 = Some(nums[i]);
                    continue;
                }
                return true;
            }
        }

        false
    }
    pub fn increasing_triplet_slow(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                if nums[i] >= nums[j] {
                    continue;
                }
                for k in j + 1..nums.len() {
                    if nums[j] < nums[k] {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use crate::increasing_triplet_subsequence::Solution;

    #[test]
    fn case2() {
        assert_eq!(Solution::increasing_triplet(vec![1]), false);
    }

    #[test]
    fn case1() {
        assert_eq!(Solution::increasing_triplet(vec![1, 1, -2, 6]), false);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]),
            true
        )
    }
}
