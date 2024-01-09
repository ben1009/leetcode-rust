use std::cmp;

/// [64] Minimum Path Sum
///
/// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right,
/// which minimizes the sum of all numbers along its path. Note: You can only move either down or
/// right at any point in time.  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/minpath.jpg" style="width: 242px; height: 242px;" />
/// Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
/// Output: 7
/// Explanation: Because the path 1 &rarr; 3 &rarr; 1 &rarr; 1 &rarr; 1 minimizes the sum.
///
/// <strong class="example">Example 2:
///
/// Input: grid = [[1,2,3],[4,5,6]]
/// Output: 12
///
///  
/// Constraints:
///
///     m == grid.length
///     n == grid[i].length
///     1 <= m, n <= 200
///     0 <= grid[i][j] <= 100
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-path-sum/
// discuss: https://leetcode.com/problems/minimum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[i][0] = grid[i][0] + dp[i - 1][0];
        }
        for i in 1..n {
            dp[0][i] = grid[0][i] + dp[0][i - 1];
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = cmp::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[m - 1][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_64() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
