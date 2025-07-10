// Given an array of integers nums and an integer k, return the total number of subarrays whose sum
// equals to k.

// A subarray is a contiguous non-empty sequence of elements within an array.

// Example 1:

// Input: nums = [1,1,1], k = 2
// Output: 2
// Example 2:

// Input: nums = [1,2,3], k = 3
// Output: 2

// Constraints:

// 1 <= nums.length <= 2 * 104
// -1000 <= nums[i] <= 1000
// -107 <= k <= 107

pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-sum-equals-k/

// submission codes start here

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut sum = 0;
        let mut dic = std::collections::HashMap::from([(0, 1)]);

        for item in nums {
            sum += item;
            // 0...i......j, sum[0..j] - sum[0..i] = k, sum[0..i] + sum[i+1..j] = sum[0..j]
            if let Some(v) = dic.get(&(sum - k)) {
                ret += v;
            }
            *dic.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_560() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::subarray_sum(vec![1], 3), 0);
    }
}
