/// [27] Remove Element
///
/// Given an integer array nums and an integer val, remove all occurrences of val in nums <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
/// Consider the number of elements in nums which are not equal to val be k, to get accepted, you
/// need to do the following things:
///
///     Change the array nums such that the first k elements of nums contain the elements which are
/// not equal to val. The remaining elements of nums are not important as well as the size of nums.
///     Return k.
///
/// Custom Judge:
/// The judge will test your solution with the following code:
///
/// int[] nums = [...]; // Input array
/// int val = ...; // Value to remove
/// int[] expectedNums = [...]; // The expected answer with correct length.
///                             // It is sorted with no values equaling val.
/// int k = removeElement(nums, val); // Calls your implementation
/// assert k == expectedNums.length;
/// sort(nums, 0, k); // Sort the first k elements of nums
/// for (int i = 0; i < actualLength; i++) {
///     assert nums[i] == expectedNums[i];
/// }
///
/// If all assertions pass, then your solution will be accepted.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [3,2,2,3], val = 3
/// Output: 2, nums = [2,2,_,_]
/// Explanation: Your function should return k = 2, with the first two elements of nums being 2.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
///
/// <strong class="example">Example 2:
///
/// Input: nums = [0,1,2,2,3,0,4,2], val = 2
/// Output: 5, nums = [0,1,4,0,3,_,_,_]
/// Explanation: Your function should return k = 5, with the first five elements of nums containing
/// 0, 0, 1, 3, and 4. Note that the five elements can be returned in any order.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
///
///  
/// Constraints:
///
///     0 <= nums.length <= 100
///     0 <= nums[i] <= 50
///     0 <= val <= 100
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-element/
// discuss: https://leetcode.com/problems/remove-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // same order
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut lo = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[lo] = nums[i];
                lo += 1;
            }
        }

        lo as i32
    }

    // any order
    pub fn remove_element1(nums: &mut [i32], val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut hi = nums.len() as i32 - 1;
        let mut lo = 0;
        while lo <= hi {
            if nums[lo as usize] == val {
                nums.swap(lo as usize, hi as usize);
                hi -= 1;
            } else {
                lo += 1;
            }
        }

        lo
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        assert_eq!(Solution::remove_element(&mut [3, 2, 2, 3], 3), 2);
        assert_eq!(
            Solution::remove_element(&mut [0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
        assert_eq!(
            Solution::remove_element(&mut [0, 1, 2, 2, 3, 0, 4, 2], 0),
            6
        );

        assert_eq!(Solution::remove_element1(&mut [3, 2, 2, 3], 3), 2);
        assert_eq!(
            Solution::remove_element1(&mut [0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
        assert_eq!(
            Solution::remove_element1(&mut [0, 1, 2, 2, 3, 0, 4, 2], 0),
            6
        );
        assert_eq!(Solution::remove_element(&mut [1], 1), 0);
        assert_eq!(Solution::remove_element1(&mut [1], 1), 0);
    }
}
