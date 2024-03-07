use std::cmp::{self};

/// [152] Maximum Product Subarray
///
/// Given an integer array nums, find a <span data-keyword="subarray-nonempty">subarray</span> that
/// has the largest product, and return the product. The test cases are generated so that the answer
/// will fit in a 32-bit integer.  
/// <strong class="example">Example 1:
///
/// Input: nums = [2,3,-2,4]
/// Output: 6
/// Explanation: [2,3] has the largest product 6.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [-2,0,-1]
/// Output: 0
/// Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
///
///  
/// Constraints:
///
///     1 <= nums.length <= 2 * 10^4
///     -10 <= nums[i] <= 10
///     The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-subarray/
// discuss: https://leetcode.com/problems/maximum-product-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut ret = nums[0];
        let mut pre_min = ret;
        let mut pre_max = ret;
        for n in nums.iter().skip(1) {
            let t_min = pre_min;
            pre_min = cmp::min(cmp::min(pre_min * n, pre_max * n), *n);
            pre_max = cmp::max(cmp::max(t_min * n, pre_max * n), *n);

            ret = cmp::max(ret, pre_max);
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_152() {
        assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
        assert_eq!(0, Solution::max_product(vec![-2, 0, -1]));
        assert_eq!(0, Solution::max_product(vec![-2, 0]));
        assert_eq!(1, Solution::max_product(vec![1]));
    }
}
