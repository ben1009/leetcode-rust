// Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the
// array if you can flip at most k 0's.

// Example 1:

// Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
// Output: 6
// Explanation: [1,1,1,0,0,1,1,1,1,1,1]
// Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
// Example 2:

// Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
// Output: 10
// Explanation: [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
// Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.

// Constraints:

// 1 <= nums.length <= 105
// nums[i] is either 0 or 1.
// 0 <= k <= nums.length

// problem: https://leetcode.com/problems/max-consecutive-ones-iii/

// submission codes start here
pub struct Solution {}

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut z = 0;
        let mut ret = 0;
        while r < nums.len() {
            if nums[r] == 0 {
                z += 1;
            }
            while l <= r && z > k {
                if nums[l] == 0 {
                    z -= 1;
                }
                l += 1;
            }
            ret = std::cmp::max(ret, r as i32 - l as i32 + 1);
            r += 1;
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1004() {
        assert_eq!(Solution::longest_ones(vec![0, 0, 0], 0), 0);
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
        assert_eq!(
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
