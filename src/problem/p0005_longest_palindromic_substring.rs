/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest <span data-keyword="palindromic-string">palindromic</span> <span data-keyword="substring-nonempty">substring</span> in s.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 1000
 *     s consist of only digits and English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let mut ret: &[u8] = &[];
        let s = s.as_bytes();
        for i in 0..s.len() - 1 {
            let s1 = Self::pali(i, i, s);
            if ret.len() < s1.len() {
                ret = s1;
            }
            let s2 = Self::pali(i, i + 1, s);
            if ret.len() < s2.len() {
                ret = s2;
            }
        }

        String::from_utf8(ret.to_owned()).unwrap()
    }

    fn pali(i: usize, mut j: usize, s: &[u8]) -> &[u8] {
        // kind of ugly
        let mut i = i as i32;
        while i >= 0 && j < s.len() {
            if s[i as usize] != s[j] {
                break;
            }
            i -= 1;
            j += 1;
        }
        if i + 1 > j as i32 - 1 {
            return &[];
        }

        i += 1;
        &s[i as usize..j]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!("a", Solution::longest_palindrome("ab".into()));
        assert_eq!("bab", Solution::longest_palindrome("babad".into()));
    }
}
