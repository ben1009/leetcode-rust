/// [1016] Binary String With Substrings Representing 1 To N
///
/// Given a binary string s and a positive integer n, return true if the binary representation of
/// all the integers in the range [1, n] are substrings of s, or false otherwise. A substring is a
/// contiguous sequence of characters within a string.  
/// <strong class="example">Example 1:
/// Input: s = "0110", n = 3
/// Output: true
/// <strong class="example">Example 2:
/// Input: s = "0110", n = 4
/// Output: false
///  
/// Constraints:
///
///     1 <= s.length <= 1000
///     s[i] is either '0' or '1'.
///     1 <= n <= 10^9
///
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n/
// discuss: https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for i in 1..=n {
            if !s.contains(&format!("{:b}", i)) {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1016() {
        assert!(Solution::query_string("0110".to_string(), 3));
        assert!(!Solution::query_string("0110".to_string(), 4));
        assert!(!Solution::query_string("0110".to_string(), 5));
    }
}
