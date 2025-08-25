#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        if mat.is_empty() || mat[0].is_empty() {
            return vec![];
        }

        let n = mat.len();
        let m = mat[0].len();

        let mut result = Vec::<i32>::with_capacity(mat.len() * mat[0].len());

        let mut i = 0;
        let mut j = 0;

        let mut moving_forward = true;

        while i < n && j < m {
            result.push(mat[i][j]);
            if moving_forward {
                if i == 0 && j == m - 1 {
                    i += 1;
                    moving_forward = false;
                    continue;
                }
                if i == 0 {
                    j += 1;
                    moving_forward = false;
                    continue;
                }
                if j == m - 1 {
                    i += 1;
                    moving_forward = false;
                    continue;
                }
                i -= 1;
                j += 1;
                continue;
            }
            if j == 0 && i == n - 1 {
                j += 1;
                moving_forward = true;
                continue;
            }
            if j == 0 {
                i += 1;
                moving_forward = true;
                continue;
            }
            if i == n - 1 {
                j += 1;
                moving_forward = true;
                continue;
            }
            i += 1;
            j -= 1;
        }

        result
    }
}
