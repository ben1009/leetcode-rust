/// [5] Longest Palindromic Substring
///
/// Given a string s, return the longest <span data-keyword="palindromic-string">palindromic</span>
/// <span data-keyword="substring-nonempty">substring</span> in s.  
/// <strong class="example">Example 1:
///
/// Input: s = "babad"
/// Output: "bab"
/// Explanation: "aba" is also a valid answer.
///
/// <strong class="example">Example 2:
///
/// Input: s = "cbbd"
/// Output: "bb"
///
///  
/// Constraints:
///
///     1 <= s.length <= 1000
///     s consist of only digits and English letters.
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let s = s.as_bytes();
        let mut ret: &[u8] = &[];
        for i in 0..s.len() - 1 {
            let s1 = Solution::pali(s, i as i32, i);
            let s2 = Solution::pali(s, i as i32, i + 1);
            if s1.len() > ret.len() {
                ret = s1;
            }
            if s2.len() > ret.len() {
                ret = s2;
            }
        }

        String::from_utf8_lossy(ret).to_string()
    }

    fn pali(s: &[u8], mut l: i32, mut r: usize) -> &[u8] {
        while l >= 0 && r < s.len() && s[l as usize] == s[r] {
            l -= 1;
            r += 1;
        }

        &s[(l + 1) as usize..r]
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
