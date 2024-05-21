use std::collections::HashSet;

/// [128] Longest Consecutive Sequence
///
/// Given an unsorted array of integers nums, return the length of the longest consecutive elements
/// sequence. You must write an algorithm that runs in O(n) time.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [100,4,200,1,3,2]
/// Output: 4
/// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is
/// 4.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [0,3,7,2,5,8,4,6,0,1]
/// Output: 9
///
///  
/// Constraints:
///
///     0 <= nums.length <= 10^5
///     -10^9 <= nums[i] <= 10^9
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-consecutive-sequence/
// discuss: https://leetcode.com/problems/longest-consecutive-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: use union-find, kind of showoff ?
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut dic = nums.iter().collect::<HashSet<_>>();
        let mut ret = 0;
        for k in &nums {
            let mut k1 = *k;
            while dic.remove(&(k1 - 1)) {
                k1 -= 1;
            }
            let mut k2 = *k;
            while dic.remove(&(k2 + 1)) {
                k2 += 1;
            }

            ret = ret.max(k2 - k1 + 1);
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
