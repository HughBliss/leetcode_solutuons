#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let mut height_start: Option<i32> = None;
        let mut height_end = 0;

        let mut width_start: i32 = grid[0].len() as i32;
        let mut width_end = 0;
        for i in 0..grid.len() {
            let mut has_ones_in_row = false;
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                has_ones_in_row = true;
                if (j as i32 - 1) < width_start {
                    width_start = j as i32 - 1;
                }
                if width_end < j {
                    width_end = j;
                }
            }
            if has_ones_in_row {
                if height_start == None {
                    height_start = Some(i as i32 - 1);
                }
                height_end = i as i32;
            }
        }

        if let Some(hs) = height_start {
            return (width_end as i32 - width_start) * (height_end as i32 - hs);
        }

        0
    }
}

#[cfg(test)]
mod test {
    use crate::find_the_minimum_area_to_cover_all_ones_i::Solution;

    #[test]
    fn case2() {
        assert_eq!(
            Solution::minimum_area(vec![
                //
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0]
            ]),
            4
        )
    }

    #[test]
    fn case1() {
        assert_eq!(
            Solution::minimum_area(vec![
                //
                vec![0, 1, 0],
                vec![1, 0, 1]
            ]),
            6
        )
    }
}
