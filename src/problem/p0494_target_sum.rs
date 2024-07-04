// You are given an integer array nums and an integer target.

// You want to build an expression out of nums by adding one of the symbols '+' and '-' before each
// integer in nums and then concatenate all the integers.

// For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them
// to build the expression "+2-1". Return the number of different expressions that you can build,
// which evaluates to target.

// Example 1:

// Input: nums = [1,1,1,1,1], target = 3
// Output: 5
// Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
// -1 + 1 + 1 + 1 + 1 = 3
// +1 - 1 + 1 + 1 + 1 = 3
// +1 + 1 - 1 + 1 + 1 = 3
// +1 + 1 + 1 - 1 + 1 = 3
// +1 + 1 + 1 + 1 - 1 = 3
// Example 2:

// Input: nums = [1], target = 1
// Output: 1

// Constraints:

// 1 <= nums.length <= 20
// 0 <= nums[i] <= 1000
// 0 <= sum(nums[i]) <= 1000
// -1000 <= target <= 1000

// https://leetcode.com/problems/target-sum
// https://leetcode.com/problems/target-sum/solution

// submission codes start here
pub struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        // s(p) + s(n) = target
        // s(p) - s(n) = sum
        // 2s(p) = target + sum
        if sum + target < 0 || (target + sum) % 2 != 0 {
            return 0;
        }

        let n = nums.len();
        let m = ((target + sum) / 2) as usize;
        let mut dp = vec![vec![0; m + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..n + 1 {
            for j in 0..m + 1 {
                dp[i][j] = dp[i - 1][j];
                if j >= nums[i - 1] as usize {
                    dp[i][j] = dp[i - 1][j - nums[i - 1] as usize] + dp[i - 1][j];
                }
            }
        }

        dp[n][m]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_494() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }
}
