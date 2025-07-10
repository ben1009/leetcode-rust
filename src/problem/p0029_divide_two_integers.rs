/// [29] Divide Two Integers
///
/// Given two integers dividend and divisor, divide two integers without using multiplication,
/// division, and mod operator. The integer division should truncate toward zero, which means losing
/// its fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be truncated
/// to -2. Return the quotient after dividing dividend by divisor.
/// Note: Assume we are dealing with an environment that could only store integers within the 32-bit
/// signed integer range: [-2^31, 2^31 - 1]. For this problem, if the quotient is strictly greater
/// than 2^31 - 1, then return 2^31 - 1, and if the quotient is strictly less than -2^31, then
/// return -2^31.  
/// <strong class="example">Example 1:
///
/// Input: dividend = 10, divisor = 3
/// Output: 3
/// Explanation: 10/3 = 3.33333.. which is truncated to 3.
///
/// <strong class="example">Example 2:
///
/// Input: dividend = 7, divisor = -3
/// Output: -2
/// Explanation: 7/-3 = -2.33333.. which is truncated to -2.
///
///  
/// Constraints:
///
///     -2^31 <= dividend, divisor <= 2^31 - 1
///     divisor != 0
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if (dividend as i64 / divisor as i64) >= i32::MAX as i64 {
            return i32::MAX;
        }
        if (dividend as i64 / divisor as i64) <= i32::MIN as i64 {
            return i32::MIN;
        }

        let mut sign = 1;
        if dividend * divisor < 0 {
            sign = -1;
        }

        let mut dd = dividend.abs() as i64;
        let dr = divisor.abs() as i64;
        let mut n = 0;
        while dd >= dr << n {
            n += 1;
        }
        let mut ret = 0;
        // dd=dr*(1<<n + 1<<(n-1) ... 1<<0)
        while n >= 0 {
            if dd >= dr << n {
                ret += 1 << n;
                dd -= dr << n;
            }
            n -= 1;
        }

        ret * sign
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        assert_eq!(Solution::divide(10, 2), 5);
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(
            Solution::divide(-2147483648, 1),
            -2147483648,
            "-2147483648, 1"
        );
        assert_eq!(
            Solution::divide(-2147483648, -1),
            2147483647,
            "-2147483648, -1"
        );
        assert_eq!(Solution::divide(2147483647, 1), 2147483647, "2147483647, 1");
        assert_eq!(
            Solution::divide(2147483647, -1),
            -2147483647,
            "2147483647, -1"
        );
    }
}
