// You are given a 0-indexed integer array nums representing the score of students in an exam. The
// teacher would like to form one non-empty group of students with maximal strength, where the
// strength of a group of students of indices i0, i1, i2, ... , ik is defined as nums[i0] * nums[i1]
// * nums[i2] * ... * nums[ik​].

// Return the maximum strength of a group the teacher can create.

// Example 1:

// Input: nums = [3,-1,-5,2,5,-9]
// Output: 1350
// Explanation: One way to form a group of maximal strength is to group the students at indices
// [0,2,3,4,5]. Their strength is 3 * (-5) * 2 * 5 * (-9) = 1350, which we can show is optimal.
// Example 2:

// Input: nums = [-4,-5,-4]
// Output: 20
// Explanation: Group the students at indices [0, 1] . Then, we’ll have a resulting strength of 20.
// We cannot achieve greater strength.

// Constraints:

// 1 <= nums.length <= 13
// -9 <= nums[i] <= 9

// problem: https://leetcode.com/problems/maximum-strength-of-a-group/
// discuss: https://leetcode.com/problems/maximum-strength-of-a-group/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
pub struct Solution {}

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        if nums.len() == 1 {
            return nums[0] as i64;
        }

        let mut ret = 1_i64;
        let mut zero = 0;
        let mut neg_max = -9_i64;
        for &n in &nums {
            if n == 0 {
                zero += 1;
                continue;
            }
            ret *= n as i64;
            if n < 0 {
                neg_max = std::cmp::max(neg_max, n as i64);
            }
        }

        if zero == nums.len() as i32 {
            return 0;
        }
        if ret > 0 {
            return ret;
        }
        if zero == nums.len() as i32 - 1 {
            return 0;
        }

        ret / neg_max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2708() {
        assert_eq!(Solution::max_strength(vec![3, -1, -5, 2, 5, -9]), 1350);
        assert_eq!(Solution::max_strength(vec![-4, -5, -4]), 20);
        assert_eq!(Solution::max_strength(vec![0, 0, 0]), 0);
        assert_eq!(Solution::max_strength(vec![0, 0, 0, -6]), 0);
    }
}
