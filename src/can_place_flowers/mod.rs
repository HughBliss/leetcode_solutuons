#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            return true;
        }

        let len = flowerbed.len();
        let mut i = 0;

        while i < len {
            let is_left_empty = (i == 0) || (flowerbed[i - 1] == 0);
            let is_right_empty = (i == len - 1) || (flowerbed[i + 1] == 0);

            if flowerbed[i] == 0 && is_left_empty && is_right_empty {
                n -= 1;
                if n == 0 {
                    return true;
                }

                i += 2;
            } else {
                i += 1;
            }
        }

        n <= 0
    }
    pub fn can_place_flowers_хитровыебаный(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut zeros_count = 1;

        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 {
                zeros_count += 1;
                continue;
            }
            if zeros_count > 2 {
                if zeros_count % 2 != 1 {
                    zeros_count -= 1;
                }
                n -= zeros_count / 2
            }
            zeros_count = 0;
        }
        if zeros_count > 1 {
            n -= zeros_count / 2
        }
        n <= 0
    }
}
