/**
 * [91] Decode Ways
 *
 * A message containing letters from A-Z can be encoded into numbers using the following mapping:
 *
 * 'A' -> "1"
 * 'B' -> "2"
 * ...
 * 'Z' -> "26"
 *
 * To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
 *
 *     "AAJF" with the grouping (1 1 10 6)
 *     "KJF" with the grouping (11 10 6)
 *
 * Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
 * Given a string s containing only digits, return the number of ways to decode it.
 * The test cases are generated so that the answer fits in a 32-bit integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "12"
 * Output: 2
 * Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "226"
 * Output: 3
 * Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "06"
 * Output: 0
 * Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 100
 *     s contains only digits and may contain leading zero(s).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-ways/
// discuss: https://leetcode.com/problems/decode-ways/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.starts_with('0') {
            return 0;
        }

        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=s.len() {
            if Self::is_valid(&s[i - 1..i]) {
                dp[i] += dp[i - 1];
            }
            if Self::is_valid(&s[i - 2..i]) {
                dp[i] += dp[i - 2];
            }
        }

        dp[s.len()]
    }

    fn is_valid(s: &str) -> bool {
        if s.starts_with('0') {
            return false;
        }

        let n = s.parse::<i32>().unwrap();
        if n > 26 {
            return false;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }
}
