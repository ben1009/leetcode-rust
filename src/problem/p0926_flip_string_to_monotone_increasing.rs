use std::cmp::min;

/// [926] Flip String to Monotone Increasing
///
/// A binary string is monotone increasing if it consists of some number of 0's (possibly none),
/// followed by some number of 1's (also possibly none). You are given a binary string s. You can
/// flip s[i] changing it from 0 to 1 or from 1 to 0. Return the minimum number of flips to make s
/// monotone increasing.  
/// <strong class="example">Example 1:
///
/// Input: s = "00110"
/// Output: 1
/// Explanation: We flip the last digit to get 00111.
///
/// <strong class="example">Example 2:
///
/// Input: s = "010110"
/// Output: 2
/// Explanation: We flip to get 011111, or alternatively 000111.
///
/// <strong class="example">Example 3:
///
/// Input: s = "00011000"
/// Output: 2
/// Explanation: We flip to get 00000000.
///
///  
/// Constraints:
///
///     1 <= s.length <= 10^5
///     s[i] is either '0' or '1'.
pub struct Solution {}

// problem: https://leetcode.com/problems/flip-string-to-monotone-increasing/
// discuss: https://leetcode.com/problems/flip-string-to-monotone-increasing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        if s.len() == 1 {
            return 0;
        }

        let n = s.len();
        let mut ret = n as i32;
        let mut l = vec![0; n];
        let mut r = vec![0; n];
        let s = s.as_bytes();

        if s[0] == b'1' {
            l[0] = 1;
        }
        for i in 1..n {
            l[i] = l[i - 1];
            if s[i] == b'1' {
                l[i] = l[i - 1] + 1;
            }
        }

        if s[n - 1] == b'0' {
            r[n - 1] = 1;
        }
        for i in (0..n - 1).rev() {
            r[i] = r[i + 1];
            if s[i] == b'0' {
                r[i] = r[i + 1] + 1;
            }
        }
        ret = min(ret, min(l[n - 1], r[0]));
        for i in 1..n - 1 {
            ret = min(ret, l[i - 1] + r[i + 1]);
        }

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
