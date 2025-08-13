struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
    pub fn remove_duplicates_my_first_sol_dont_work_on_leetcode(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut dublicates_count = 0;
        let mut prev: i32 = nums[0];
        let mut i = 1;
        while i < nums.len() {
            if prev == nums[i] {
                dublicates_count += 1;
                nums.remove(i);
                continue;
            }
            prev = nums[i];
            i += 1;
        }

        dublicates_count
    }
}

#[cfg(test)]
mod test {
    use crate::remove_duplicates_from_sorted_array::Solution;

    #[test]
    fn case1() {
        let mut arr = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let dublicates_count = Solution::remove_duplicates(arr.as_mut());
        assert_eq!(dublicates_count, 5);
        assert_eq!(arr, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn case2() {
        let mut arr = vec![1, 1, 2];
        let dublicates_count = Solution::remove_duplicates(arr.as_mut());
        assert_eq!(dublicates_count, 1);
        assert_eq!(arr, vec![1, 2]);
    }
}
