/// [41] First Missing Positive
///
/// Given an unsorted integer array nums, return the smallest missing positive integer.
/// You must implement an algorithm that runs in O(n) time and uses constant extra space.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [1,2,0]
/// Output: 3
/// Explanation: The numbers in the range [1,2] are all in the array.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [3,4,-1,1]
/// Output: 2
/// Explanation: 1 is in the array but 2 is missing.
///
/// <strong class="example">Example 3:
///
/// Input: nums = [7,8,9,11,12]
/// Output: 1
/// Explanation: The smallest positive integer 1 is missing.
///
///  
/// Constraints:
///
///     1 <= nums.length <= 10^5
///     -2^31 <= nums[i] <= 2^31 - 1
pub struct Solution {}

// problem: https://leetcode.com/problems/first-missing-positive/
// discuss: https://leetcode.com/problems/first-missing-positive/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] <= 0
                || nums[i] >= nums.len() as i32
                || nums[i] == nums[nums[i] as usize]
                || nums[i] == i as i32 + 1
            {
                i += 1;
                continue;
            }
            let t = nums[i];
            nums.swap(t as usize, i);
        }

        for (i, item) in nums.iter().enumerate() {
            if *item != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41() {
        assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
