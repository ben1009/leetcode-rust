// Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]),
// return the next greater number for every element in nums.

// The next greater number of a number x is the first greater number to its traversing-order next in
// the array, which means you could search circularly to find its next greater number. If it doesn't
// exist, return -1 for this number.

// Example 1:

// Input: nums = [1,2,1]
// Output: [2,-1,2]
// Explanation: The first 1's next greater number is 2;
// The number 2 can't find next greater number.
// The second 1's next greater number needs to search circularly, which is also 2.
// Example 2:

// Input: nums = [1,2,3,4,3]
// Output: [2,3,4,-1,4]

// Constraints:

// 1 <= nums.length <= 104
// -109 <= nums[i] <= 109

pub struct Solution {}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; nums.len()];
        let mut stack = vec![];
        for i in 0..2 * nums.len() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i % nums.len()] {
                let j = stack.pop().unwrap();
                ret[j] = nums[i % nums.len()];
            }
            if i < nums.len() {
                stack.push(i);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0503() {
        assert_eq!(Solution::next_greater_elements(vec![1, 2, 1]), vec![
            2, -1, 2
        ]);
        assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 3]), vec![
            2, 3, 4, -1, 4
        ]);
    }
}
