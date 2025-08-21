#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        if mat.is_empty() || mat[0].is_empty() {
            return 0;
        }

        let mut rects_count = 0;
        let num_cols = mat[0].len();
        let mut heights = vec![0; num_cols];

        for i in 0..mat.len() {
            for j in 0..num_cols {
                if mat[i][j] == 1 {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }

            for j in 0..num_cols {
                if heights[j] == 0 {
                    continue;
                }
                let mut min_height = heights[j];
                for k in (0..=j).rev() {
                    if heights[k] == 0 {
                        break;
                    }
                    min_height = min_height.min(heights[k]);

                    rects_count += min_height;
                }
            }
        }

        rects_count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case1() {
        let mat = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::num_submat(mat), 13);
    }

    #[test]
    fn case2() {
        let mat = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]];
        assert_eq!(Solution::num_submat(mat), 24);
    }

    #[test]
    fn case3() {
        let mat = vec![vec![1, 1, 1, 1, 1, 1]];
        assert_eq!(Solution::num_submat(mat), 21);
    }

    #[test]
    fn case4() {
        let mat = vec![vec![0]];
        assert_eq!(Solution::num_submat(mat), 0);
    }
}
