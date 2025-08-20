#![allow(dead_code)]

struct Solution;

use std::cmp::min;
impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut squares_count = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                for k in 0..min(matrix.len() - i, matrix[i].len() - j) {
                    let mut sum_buffer = 0;
                    for l in 0..=k {
                        for m in 0..=k {
                            sum_buffer += matrix[i + l][j + m];
                        }
                    }

                    if sum_buffer != (k + 1).pow(2) as i32 {
                        break;
                    }
                    squares_count += 1;
                }
            }
        }

        return squares_count;
    }
}

#[cfg(test)]
mod test {
    use crate::count_square_submatrices_with_all_ones::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::count_squares(vec![
                vec![0, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![0, 1, 1, 1]
            ]),
            15
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::count_squares(vec![
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            7
        )
    }
}
