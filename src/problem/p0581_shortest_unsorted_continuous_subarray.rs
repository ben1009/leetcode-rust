/**
 * [581] Shortest Unsorted Continuous Subarray
 *
 * Given an integer array nums, you need to find one continuous subarray such that if you only sort this subarray in non-decreasing order, then the whole array will be sorted in non-decreasing order.
 * Return the shortest such subarray and output its length.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,6,4,8,10,9,15]
 * Output: 5
 * Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3,4]
 * Output: 0
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^4
 *     -10^5 <= nums[i] <= 10^5
 *
 *  
 * Follow up: Can you solve it in O(n) time complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-unsorted-continuous-subarray/
// discuss: https://leetcode.com/problems/shortest-unsorted-continuous-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }

        let mut m = -1;
        let mut n = -1;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                m = i as i32 - 1;
                break;
            }
        }
        if m == -1 {
            return 0;
        }

        for i in (1..nums.len()).rev() {
            if nums[i] < nums[i - 1] {
                n = i as i32;
                break;
            }
        }
        let mut min = nums[m as usize];
        let mut max = nums[m as usize];
        for i in m..n + 1 {
            if nums[i as usize] > max {
                max = nums[i as usize];
            }
            if nums[i as usize] < min {
                min = nums[i as usize];
            }
        }
        let mut lo = 0;
        let mut hi = nums.len() as i32 - 1;
        for i in (0..m).rev() {
            if nums[i as usize] <= min {
                lo = i + 1;
                break;
            }
        }
        for i in n + 1..nums.len() as i32 {
            if nums[i as usize] >= max {
                hi = i - 1;
                break;
            }
        }

        hi - lo + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_581() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
            5
        );
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 116, 117, 4, 8, 10, 9, 15]),
            7
        );
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
        assert_eq!(Solution::find_unsorted_subarray(vec![2, 1]), 2);
    }
}
