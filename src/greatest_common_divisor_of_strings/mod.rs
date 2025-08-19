#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // Проверяем необходимое условие: если общего делителя нет,
        // то str1 + str2 != str2 + str1.
        if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
            return "".to_string();
        }

        // Если условие выполнено, то общий делитель точно есть,
        // и его длина равна НОД длин исходных строк.
        let len1 = str1.len();
        let len2 = str2.len();

        // Находим НОД для длин.
        let gcd_len = Self::gcd(len1, len2);

        // Результатом будет префикс любой из строк длиной gcd_len.
        // Берем срез (slice) из str1.
        str1[..gcd_len].to_string()
    }

    // Вспомогательная функция для нахождения НОД (алгоритм Евклида).
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    pub fn gcd_of_strings_pizdec(str1: String, str2: String) -> String {
        let bts1 = str1.as_bytes();
        let bts2 = str2.as_bytes();

        let mut gcd: Vec<u8> = vec![];

        let mut iter1 = bts1.into_iter();
        let mut iter2 = bts2.into_iter();

        let mut is_valid = false;
        while let (Some(ch1), Some(ch2)) = (iter1.next(), iter2.next()) {
            if ch1 != ch2 {
                break;
            }
            gcd.push(*ch1);

            for (i, ch) in gcd.iter().enumerate() {
                match (bts2.get(gcd.len() + i), bts1.get(gcd.len() + i)) {
                    (Some(b1), Some(b2)) => {
                        if b1 == ch && b2 == ch {
                            is_valid = true;
                            continue;
                        }
                    }
                    (Some(b1), None) => {
                        if b1 == ch {
                            is_valid = true;
                            continue;
                        }
                    }
                    (None, Some(b2)) => {
                        if b2 == ch {
                            is_valid = true;
                            continue;
                        }
                    }
                    (None, None) => {}
                }
                is_valid = false;
                break;
            }

            if is_valid {
                break;
            }
        }
        if !is_valid {
            return "".to_string();
        }

        String::from_utf8(gcd).unwrap()
    }
}
#[cfg(test)]
mod test {
    use crate::greatest_common_divisor_of_strings::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::gcd_of_strings("ABC".to_string(), "ABCABC".to_string()),
            "ABC".to_string()
        )
    }
}
