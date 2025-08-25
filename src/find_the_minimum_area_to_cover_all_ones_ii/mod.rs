#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // Вспомогательное замыкание для вычисления минимальной площади
        // для покрытия всех '1' в заданной подсетке.
        // Границы r_end и c_end не включаются.
        let calculate_area = |r_start: usize, c_start: usize, r_end: usize, c_end: usize| -> i32 {
            let mut min_r = m as i32;
            let mut max_r = -1;
            let mut min_c = n as i32;
            let mut max_c = -1;

            for r in r_start..r_end {
                for c in c_start..c_end {
                    if grid[r][c] == 1 {
                        min_r = std::cmp::min(min_r, r as i32);
                        max_r = std::cmp::max(max_r, r as i32);
                        min_c = std::cmp::min(min_c, c as i32);
                        max_c = std::cmp::max(max_c, c as i32);
                    }
                }
            }

            // Если в подсетке не найдено ни одной единицы
            if max_r == -1 {
                return 0;
            }
            (max_r - min_r + 1) * (max_c - min_c + 1)
        };

        let mut min_total_area = i32::MAX;

        // Шаблон 1: Два горизонтальных разреза (три горизонтальных прямоугольника)
        for i in 1..m {
            for j in (i + 1)..m {
                let area1 = calculate_area(0, 0, i, n);
                let area2 = calculate_area(i, 0, j, n);
                let area3 = calculate_area(j, 0, m, n);
                min_total_area = std::cmp::min(min_total_area, area1 + area2 + area3);
            }
        }

        // Шаблон 2: Два вертикальных разреза (три вертикальных прямоугольника)
        for i in 1..n {
            for j in (i + 1)..n {
                let area1 = calculate_area(0, 0, m, i);
                let area2 = calculate_area(0, i, m, j);
                let area3 = calculate_area(0, j, m, n);
                min_total_area = std::cmp::min(min_total_area, area1 + area2 + area3);
            }
        }

        // Шаблоны 3 и 4: Один горизонтальный разрез, затем вертикальный
        for i in 1..m {
            // Позиция горизонтального разреза
            for j in 1..n {
                // Позиция вертикального разреза
                // Случай 3: Прямоугольник сверху, два снизу (левый и правый)
                let a1 = calculate_area(0, 0, i, n);
                let a2 = calculate_area(i, 0, m, j);
                let a3 = calculate_area(i, j, m, n);
                min_total_area = std::cmp::min(min_total_area, a1 + a2 + a3);

                // Случай 4: Прямоугольник снизу, два сверху (левый и правый)
                let b1 = calculate_area(i, 0, m, n);
                let b2 = calculate_area(0, 0, i, j);
                let b3 = calculate_area(0, j, i, n);
                min_total_area = std::cmp::min(min_total_area, b1 + b2 + b3);
            }
        }

        // Шаблоны 5 и 6: Один вертикальный разрез, затем горизонтальный
        for i in 1..n {
            // Позиция вертикального разреза
            for j in 1..m {
                // Позиция горизонтального разреза
                // Случай 5: Прямоугольник слева, два справа (верхний и нижний)
                let a1 = calculate_area(0, 0, m, i);
                let a2 = calculate_area(0, i, j, n);
                let a3 = calculate_area(j, i, m, n);
                min_total_area = std::cmp::min(min_total_area, a1 + a2 + a3);

                // Случай 6: Прямоугольник справа, два слева (верхний и нижний)
                let b1 = calculate_area(0, i, m, n);
                let b2 = calculate_area(0, 0, j, i);
                let b3 = calculate_area(j, 0, m, i);
                min_total_area = std::cmp::min(min_total_area, b1 + b2 + b3);
            }
        }

        min_total_area
    }
}

#[cfg(test)]
mod test {
    use crate::find_the_minimum_area_to_cover_all_ones_ii::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::minimum_sum(vec![
                //
                vec![1, 0, 1],
                vec![1, 1, 1],
            ]),
            5
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::minimum_sum(vec![
                //
                vec![1, 0, 1, 0],
                vec![0, 1, 0, 1]
            ]),
            5
        )
    }
}
