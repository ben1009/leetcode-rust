use std::cmp::min;

/**
 * [926] Flip String to Monotone Increasing
 *
 * A binary string is monotone increasing if it consists of some number of 0's (possibly none), followed by some number of 1's (also possibly none).
 * You are given a binary string s. You can flip s[i] changing it from 0 to 1 or from 1 to 0.
 * Return the minimum number of flips to make s monotone increasing.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "00110"
 * Output: 1
 * Explanation: We flip the last digit to get 00111.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "010110"
 * Output: 2
 * Explanation: We flip to get 011111, or alternatively 000111.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "00011000"
 * Output: 2
 * Explanation: We flip to get 00000000.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flip-string-to-monotone-increasing/
// discuss: https://leetcode.com/problems/flip-string-to-monotone-increasing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }

        let n: usize = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut l = vec![0; n];
        if s[0] == '1' {
            l[0] = 1;
        }
        for i in 1..=n - 1 {
            if s[i] == '1' {
                l[i] = l[i - 1] + 1;
            } else {
                l[i] = l[i - 1];
            }
        }
        let mut r = vec![0; n];
        if s[n - 1] == '0' {
            r[n - 1] = 1;
        }
        for i in (0..=n - 2).rev() {
            if s[i] == '0' {
                r[i] = r[i + 1] + 1;
            } else {
                r[i] = r[i + 1];
            }
        }
        let mut ret: i32 = n as i32;
        for i in 0..n - 1 {
            ret = min(ret, l[i] + r[i + 1]);
        }
        ret = min(ret, min(r[0], l[n - 1]));

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_926() {
        assert_eq!(
            Solution::min_flips_mono_incr("00110".to_string()),
            1,
            "00110"
        );
        assert_eq!(Solution::min_flips_mono_incr("010110".to_string()), 2);
        assert_eq!(Solution::min_flips_mono_incr("00011000".to_string()), 2);
        assert_eq!(Solution::min_flips_mono_incr("010110".to_string()), 2);
    }
}
