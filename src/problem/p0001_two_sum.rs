use std::collections::HashMap;

/// [1] Two Sum
///
/// Given an array of integers nums and an integer target, return indices of the two numbers such
/// that they add up to target. You may assume that each input would have exactly one solution, and
/// you may not use the same element twice. You can return the answer in any order.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// <strong class="example">Example 2:
///
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
///
/// <strong class="example">Example 3:
///
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
///
///  
/// Constraints:
///
///    2 <= nums.length <= 10^4
///    -10^9 <= nums[i] <= 10^9
///    -10^9 <= target <= 10^9
///    Only one valid answer exists.
///
///  
/// Follow-up: Can you come up with an algorithm that is less than O(n^2)<font face="monospace">
/// </font>time complexity?
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, item) in nums.iter().enumerate() {
            if map.contains_key(&(target - item)) {
                return vec![map[&(target - item)], i as i32];
            }
            map.insert(item, i as i32);
        }

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1]);
        assert!(Solution::two_sum(vec![3, 2, 4], 6) == vec![1, 2]);
    }
}
