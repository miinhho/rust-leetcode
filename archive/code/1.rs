use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0, 1];
        }

        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&j) = map.get(&complement) {
                return vec![j as i32, i as i32];
            }

            map.insert(num, i);
        }

        vec![0, 0]
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];

        assert_eq!(expected, Solution::two_sum(nums, target));
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];

        assert_eq!(expected, Solution::two_sum(nums, target));
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];

        assert_eq!(expected, Solution::two_sum(nums, target));
    }
}
