// Given an array of integers, return the maximum sum for a non-empty subarray (contiguous elements)
// with at most one element deletion. In other words, you want to choose a subarray and optionally
// delete one element from it so that there is still at least one element left and the sum of the
// remaining elements is maximum possible.

// Note that the subarray needs to be non-empty after deleting one element.

// Example 1:

// Input: arr = [1,-2,0,3]
// Output: 4
// Explanation: Because we can choose [1, -2, 0, 3] and drop -2, thus the subarray [1, 0, 3] becomes
// the maximum value. Example 2:

// Input: arr = [1,-2,-2,3]
// Output: 3
// Explanation: We just choose [3] and it's the maximum sum.
// Example 3:

// Input: arr = [-1,-1,-1,-1]
// Output: -1
// Explanation: The final subarray needs to be non-empty. You can't choose [-1] and delete -1 from
// it, then get an empty subarray to make the sum equals to 0.

// Constraints:

// 1 <= arr.length <= 105
// -104 <= arr[i] <= 104

// https://leetcode.com/problems/maximum-subarray-sum-with-one-deletion/
// https://leetcode.com/problems/maximum-subarray-sum-with-one-deletion/discuss/1763358/Rust-or-Explanation-or-Implementation

pub struct Solution {}

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return arr[0];
        }

        let mut ret = arr[0];
        let mut no_del = arr[0];
        let mut del = 0;
        for item in arr.iter().skip(1) {
            del = std::cmp::max(del + item, no_del);
            no_del = std::cmp::max(no_del + item, *item);
            ret = std::cmp::max(ret, std::cmp::max(del, no_del));
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1186() {
        assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
        assert_eq!(Solution::maximum_sum(vec![1, -2, -2, 3]), 3);
    }
}
