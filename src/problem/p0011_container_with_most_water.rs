/// [11] Container With Most Water
///
/// You are given an integer array height of length n. There are n vertical lines drawn such that
/// the two endpoints of the i^th line are (i, 0) and (i, height[i]). Find two lines that together
/// with the x-axis form a container, such that the container contains the most water. Return the
/// maximum amount of water a container can store. Notice that you may not slant the container.
///  
/// <strong class="example">Example 1:
/// <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg" style="width: 600px; height: 287px;" />
/// Input: height = [1,8,6,2,5,4,8,3,7]
/// Output: 49
/// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this
/// case, the max area of water (blue section) the container can contain is 49.
///
/// <strong class="example">Example 2:
///
/// Input: height = [1,1]
/// Output: 1
///
///  
/// Constraints:
///
///     n == height.length
///     2 <= n <= 10^5
///     0 <= height[i] <= 10^4
pub struct Solution {}

// problem: https://leetcode.com/problems/container-with-most-water/
// discuss: https://leetcode.com/problems/container-with-most-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut t;
        while i != j {
            t = (j - i) as i32;
            if height[i] < height[j] {
                t *= height[i];
                i += 1;
            } else {
                t *= height[j];
                j -= 1;
            }
            ret = ret.max(t);
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 11]), 1);
    }
}
