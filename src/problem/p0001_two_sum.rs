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
///     2 <= nums.length <= 10^4
///     -10^9 <= nums[i] <= 10^9
///     -10^9 <= target <= 10^9
///     Only one valid answer exists.
///
///  
/// Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut ret = vec![];
        for (idx, num) in nums.iter().enumerate() {
            // https://stackoverflow.com/a/43360980
            if let Some(v) = map.get(&(target - num)) {
                ret = vec![idx as i32, *v as i32];
                break;
            }
            map.insert(num, idx);
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let v = vec![1, 2, 3];
        let ret = Solution::two_sum(v, 4);
        assert_eq!(vec![2, 0], ret);

        assert_eq!(vec![1, 0], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![2, 1], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
