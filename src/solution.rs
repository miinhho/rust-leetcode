struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let s_replaced = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_replaced
            .chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn example_1() {
        let input: String = "III".to_string();
        let expected: i32 = 3;
        assert_eq!(expected, Solution::roman_to_int(input));
    }

    #[test]
    fn example_2() {
        let input: String = "LVIII".to_string();
        let expected: i32 = 58;
        assert_eq!(expected, Solution::roman_to_int(input));
    }

    #[test]
    fn example_3() {
        let input: String = "MCMXCIV".to_string();
        let expected: i32 = 1994;
        assert_eq!(expected, Solution::roman_to_int(input));
    }
}
