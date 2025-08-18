#![allow(dead_code)]

use std::vec;

struct Solution;
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let input: Vec<f64> = cards.into_iter().map(|x| x as f64).collect();
        return Self::judge_point24_f64(input);
    }

    pub fn judge_point24_f64(cards: Vec<f64>) -> bool {
        if cards.len() == 1 {
            return (cards[0] - 24.0).abs() < 1e-6;
        }

        for i in 0..cards.len() {
            for j in 0..cards.len() {
                if i == j {
                    continue;
                }

                let mut next_cards_base = vec![];
                for k in 0..cards.len() {
                    if k != i && k != j {
                        next_cards_base.push(cards[k]);
                    }
                }

                let a = cards[i];
                let b = cards[j];

                if Self::judge_point24_f64([vec![a + b], next_cards_base.clone()].concat()) {
                    return true;
                }
                if Self::judge_point24_f64([vec![a * b], next_cards_base.clone()].concat()) {
                    return true;
                }
                if Self::judge_point24_f64([vec![a - b], next_cards_base.clone()].concat()) {
                    return true;
                }
                if Self::judge_point24_f64([vec![b - a], next_cards_base.clone()].concat()) {
                    return true;
                }
                if b.abs() > 1e-6 {
                    if Self::judge_point24_f64([vec![a / b], next_cards_base.clone()].concat()) {
                        return true;
                    }
                }
                if a.abs() > 1e-6 {
                    if Self::judge_point24_f64([vec![b / a], next_cards_base.clone()].concat()) {
                        return true;
                    }
                }
            }
        }

        false
    }
    pub fn judge_point24_dumb(cards: Vec<i32>) -> bool {
        if cards.len() == 4 {
            return Self::judge_point24_dumb(vec![cards[0] + cards[1], cards[2], cards[3]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1] + cards[2], cards[3]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1], cards[2] + cards[3]])
                || Self::judge_point24_dumb(vec![cards[0] - cards[1], cards[2], cards[3]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1] - cards[2], cards[3]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1], cards[2] - cards[3]])
                || Self::judge_point24_dumb(vec![cards[0] * cards[1], cards[2], cards[3]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1] * cards[2], cards[3]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1], cards[2] * cards[3]])
                || !(cards[1] == 0
                    || !Self::judge_point24_dumb(vec![cards[0] / cards[1], cards[2], cards[3]]))
                || !(cards[2] == 0
                    || !Self::judge_point24_dumb(vec![cards[0], cards[1] / cards[2], cards[3]]))
                || !(cards[3] == 0
                    || !Self::judge_point24_dumb(vec![cards[0], cards[1], cards[2] / cards[3]]));
        }
        if cards.len() == 3 {
            return Self::judge_point24_dumb(vec![cards[0] + cards[1], cards[2]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1] + cards[2]])
                || Self::judge_point24_dumb(vec![cards[0] - cards[1], cards[2]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1] - cards[2]])
                || Self::judge_point24_dumb(vec![cards[0] * cards[1], cards[2]])
                || Self::judge_point24_dumb(vec![cards[0], cards[1] * cards[2]])
                || !(cards[1] == 0
                    || !Self::judge_point24_dumb(vec![cards[0] / cards[1], cards[2]]))
                || !(cards[2] == 0
                    || !Self::judge_point24_dumb(vec![cards[0], cards[1] / cards[2]]));
        }
        if cards.len() == 2 {
            if cards[1] == 0 {
                return cards[0] == 24;
            }
            return cards[0] + cards[1] == 24
                || cards[0] - cards[1] == 24
                || cards[0] * cards[1] == 24
                || cards[0] / cards[1] == 24;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::twenty_four_game::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true)
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false)
    }
}
