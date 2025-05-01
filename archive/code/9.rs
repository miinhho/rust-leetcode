struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut x_temp: i64 = x.into();
        let mut temp: i64 = 0;

        while x_temp > 0 {
            temp = temp * 10 + x_temp % 10;
            x_temp /= 10;
        }

        temp == i64::from(x)
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_palindrome(121));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_palindrome(-121));
    }

    #[test]
    fn example_3() {
        assert_eq!(false, Solution::is_palindrome(10));
    }
}
