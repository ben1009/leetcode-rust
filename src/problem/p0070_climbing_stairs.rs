/// [70] Climbing Stairs
///
/// You are climbing a staircase. It takes n steps to reach the top.
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
///  
/// <strong class="example">Example 1:
///
/// Input: n = 2
/// Output: 2
/// Explanation: There are two ways to climb to the top.
/// 1. 1 step + 1 step
/// 2. 2 steps
///
/// <strong class="example">Example 2:
///
/// Input: n = 3
/// Output: 3
/// Explanation: There are three ways to climb to the top.
/// 1. 1 step + 1 step + 1 step
/// 2. 1 step + 2 steps
/// 3. 2 steps + 1 step
///
///  
/// Constraints:
///
///     1 <= n <= 45
pub struct Solution {}

// problem: https://leetcode.com/problems/climbing-stairs/
// discuss: https://leetcode.com/problems/climbing-stairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return n;
        }

        let n = n as usize;
        let mut dp = vec![0; n];
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
