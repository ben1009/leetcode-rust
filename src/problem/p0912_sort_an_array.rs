/// [912] Sort an Array
///
/// Given an array of integers nums, sort the array in ascending order and return it.
/// You must solve the problem without using any built-in functions in O(nlog(n)) time complexity
/// and with the smallest space complexity possible.  
/// <strong class="example">Example 1:
///
/// Input: nums = [5,2,3,1]
/// Output: [1,2,3,5]
/// Explanation: After sorting the array, the positions of some numbers are not changed (for
/// example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).
///
/// <strong class="example">Example 2:
///
/// Input: nums = [5,1,1,2,0,0]
/// Output: [0,0,1,1,2,5]
/// Explanation: Note that the values of nums are not necessarily unique.
///
///  
/// Constraints:
///
///     1 <= nums.length <= 5 * 10^4
///     -5 * 10^4 <= nums[i] <= 5 * 10^4
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-an-array/
// discuss: https://leetcode.com/problems/sort-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return nums;
        }

        let mut max = i32::MIN;
        for i in &nums {
            max = std::cmp::max(max, i.abs());
        }

        let mut i = 0;
        let mut nums = nums;
        while max >= 10_i32.pow(i) {
            nums = Self::sort(&nums, i);
            i += 1;
        }

        nums
    }

    fn sort(nums: &[i32], k: u32) -> Vec<i32> {
        let mut count = [0; 21]; // [-9 - 9]
        for item in nums.iter() {
            let n = item / 10_i32.pow(k) % 10 + 10;
            count[n as usize + 1] += 1;
        }
        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        let mut ret = vec![0; nums.len()];
        for item in nums.iter() {
            let n = item / 10_i32.pow(k) % 10 + 10;
            ret[count[n as usize]] = *item;
            count[n as usize] += 1;
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_912() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(Solution::sort_array(vec![-5, 2, 3, 1]), vec![-5, 1, 2, 3]);
        assert_eq!(
            Solution::sort_array(vec![511, 2, -3, 11]),
            vec![-3, 2, 11, 511]
        );
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
        assert_eq!(Solution::sort_array(vec![2, 1]), vec![1, 2]);
        assert_eq!(Solution::sort_array(vec![1, 1, 2]), vec![1, 1, 2]);
        assert_eq!(
            Solution::sort_array(vec![-1, 2, -8, -10]),
            vec![-10, -8, -1, 2]
        );
    }
}
