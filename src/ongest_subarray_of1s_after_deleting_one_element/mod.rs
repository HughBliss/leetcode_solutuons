struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut zero_count = 0;
        let mut max_len = 0;

        for right in 0..n {
            if nums[right] == 0 {
                zero_count += 1;
            }

            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        
        if n == 0 {
            return 0;
        }

        (max_len as i32) - 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution; // Используем super::Solution, так как код внутри модуля test

    #[test]
    fn case1() {
        // [1, 1, 0, 1]. Максимальное окно [1, 1, 0, 1] (длина 4). Результат 4 - 1 = 3.
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
    }
    
    #[test]
    fn case2() {
        // [0, 1, 1, 1, 0, 1, 1, 0, 1]. Максимальное окно [1, 1, 1, 0, 1, 1] (длина 6).
        // Удаление нуля в центре дает 5 единиц. Результат 6 - 1 = 5.
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        )
    }

    #[test]
    fn case3() {
        // [1, 1, 1]. Максимальное окно [1, 1, 1] (длина 3). Результат 3 - 1 = 2.
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
    }
    
    #[test]
    fn case4_zeros() {
        // [0, 0, 0]. Максимальное окно [0] (длина 1). Результат 1 - 1 = 0.
        assert_eq!(Solution::longest_subarray(vec![0, 0, 0]), 0);
    }
}