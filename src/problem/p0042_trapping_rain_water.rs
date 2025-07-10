// 42. Trapping Rain Water

// Given n non-negative integers representing an elevation map where the width of each bar is 1,
// compute how much water it can trap after raining.

// Example 1:

// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6
// Explanation: The above elevation map (black section) is represented by array
// [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
// Example 2:

// Input: height = [4,2,0,3,2,5]
// Output: 9

// Constraints:

// n == height.length
// 1 <= n <= 2 * 104
// 0 <= height[i] <= 105
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water/
// discuss: https://leetcode.com/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() <= 2 {
            return 0;
        }

        let mut ret = 0;
        let n = height.len();
        let mut l = vec![0; n];
        l[0] = height[0];
        for i in 1..n {
            l[i] = std::cmp::max(l[i - 1], height[i]);
        }

        let mut r = vec![0; n];
        r[n - 1] = height[n - 1];
        for i in (0..=n - 2).rev() {
            r[i] = std::cmp::max(r[i + 1], height[i]);
        }

        for i in 1..n - 1 {
            ret += std::cmp::max(0, std::cmp::min(l[i - 1], r[i + 1]) - height[i]);
        }

        ret
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(Solution::trap(vec![5, 2, 1, 2, 1, 5]), 14);
        assert_eq!(Solution::trap(vec![5, 1]), 0);
    }
}
