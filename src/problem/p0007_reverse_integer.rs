/// [7] Reverse Integer
///
/// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the
/// value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0. Assume the
/// environment does not allow you to store 64-bit integers (signed or unsigned).  
/// <strong class="example">Example 1:
///
/// Input: x = 123
/// Output: 321
///
/// <strong class="example">Example 2:
///
/// Input: x = -123
/// Output: -321
///
/// <strong class="example">Example 3:
///
/// Input: x = 120
/// Output: 21
///
///  
/// Constraints:
///
///     -2^31 <= x <= 2^31 - 1
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-integer/
// discuss: https://leetcode.com/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ret = 0;
        let mut x = x;
        while x != 0 {
            ret = ret * 10 + x as i64 % 10;
            if ret > i32::MAX as i64 || ret < i32::MIN as i64 {
                return 0;
            }

            x /= 10;
        }

        ret as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(1), 1);
        assert_eq!(Solution::reverse(12300), 321);
        assert_eq!(Solution::reverse(-123), -321);
    }
}
