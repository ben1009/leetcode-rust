/**
 * [29] Divide Two Integers
 *
 * Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
 * The integer division should truncate toward zero, which means losing its fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be truncated to -2.
 * Return the quotient after dividing dividend by divisor.
 * Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [-2^31, 2^31 - 1]. For this problem, if the quotient is strictly greater than 2^31 - 1, then return 2^31 - 1, and if the quotient is strictly less than -2^31, then return -2^31.
 *  
 * <strong class="example">Example 1:
 *
 * Input: dividend = 10, divisor = 3
 * Output: 3
 * Explanation: 10/3 = 3.33333.. which is truncated to 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: dividend = 7, divisor = -3
 * Output: -2
 * Explanation: 7/-3 = -2.33333.. which is truncated to -2.
 *
 *  
 * Constraints:
 *
 *     -2^31 <= dividend, divisor <= 2^31 - 1
 *     divisor != 0
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut dd: i64 = dividend as i64;
        let mut di: i64 = divisor as i64;
        let mut ret: i64 = 0;
        let mut sign = 1;
        if dd * di < 0 {
            sign = -1;
        }
        dd = dd.abs();
        di = di.abs();

        let mut i = 0;
        while dd >= di {
            di <<= 1;
            i += 1;
        }

        while i >= 0 {
            if dd >= di {
                dd -= di;
                ret += 1 << i;
            }
            di >>= 1;
            i -= 1;
        }

        ret *= sign;
        if ret > i32::MAX as i64 {
            return i32::MAX;
        }
        if ret < i32::MIN as i64 {
            return i32::MIN;
        }

        ret as i32
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
