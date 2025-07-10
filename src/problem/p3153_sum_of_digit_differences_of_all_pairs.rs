// You are given an array nums consisting of positive integers where all integers have the same
// number of digits.

// The digit difference between two integers is the count of different digits that are in the same
// position in the two integers.

// Return the sum of the digit differences between all pairs of integers in nums.

// Example 1:

// Input: nums = [13,23,12]

// Output: 4

// Explanation:
// We have the following:
// - The digit difference between 13 and 23 is 1.
// - The digit difference between 13 and 12 is 1.
// - The digit difference between 23 and 12 is 2.
// So the total sum of digit differences between all pairs of integers is 1 + 1 + 2 = 4.

// Example 2:

// Input: nums = [10,10,10,10]

// Output: 0

// Explanation:
// All the integers in the array are the same. So the total sum of digit differences between all
// pairs of integers will be 0.

// Constraints:

// 2 <= nums.length <= 105
// 1 <= nums[i] < 10~9
// All integers in nums have the same number of digits.

// problem: https://leetcode.com/problems/sum-of-digit-differences-of-all-pairs/description/
// solution: https://leetcode.com/problems/sum-of-digit-differences-of-all-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut ret = 0;
        let m = nums[0].to_string().len();
        let mut count = vec![vec![0; 10]; m];
        for &n in &nums {
            let mut k = n;
            for c in count.iter_mut() {
                let d = (k % 10) as usize;
                c[d] += 1;
                k /= 10;
            }
        }
        for c in count.iter() {
            for &item in c.iter() {
                ret += item as i64 * (nums.len() as i64 - item as i64);
            }
        }

        ret / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3153() {
        assert_eq!(4, Solution::sum_digit_differences(vec![13, 23, 12]));
        assert_eq!(0, Solution::sum_digit_differences(vec![10, 10, 10, 10]));
    }
}
