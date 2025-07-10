// Given an integer array nums, return true if you can partition the array into two subsets such
// that the sum of the elements in both subsets is equal or false otherwise.

// Example 1:

// Input: nums = [1,5,11,5]
// Output: true
// Explanation: The array can be partitioned as [1, 5, 5] and [11].
// Example 2:

// Input: nums = [1,2,3,5]
// Output: false
// Explanation: The array cannot be partitioned into equal sum subsets.

// Constraints:

// 1 <= nums.length <= 200
// 1 <= nums[i] <= 100

// https://leetcode.com/problems/partition-equal-subset-sum
// https://leetcode.com/problems/partition-equal-subset-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }

        let n = nums.len();
        let m = sum as usize / 2;
        let mut dp = vec![vec![0; m + 1]; n + 1];

        dp[0][0] = 1;
        for i in 1..n + 1 {
            for j in 0..m + 1 {
                dp[i][j] = dp[i - 1][j];
                if j >= nums[i - 1] as usize {
                    dp[i][j] = dp[i - 1][j - nums[i - 1] as usize] + dp[i - 1][j];
                }
            }
            if dp[i][m] != 0 {
                return true;
            }
        }

        dp[n][m] != 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0416() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
        assert!(!Solution::can_partition(vec![1, 2, 5]));
    }
}
