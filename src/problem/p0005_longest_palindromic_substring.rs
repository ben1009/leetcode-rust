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
        if s.len() < 2 {
            return s;
        }

        let a = s.as_bytes();
        let mut ret = &a[0..1];
        for i in 0..a.len() - 1 {
            let s1 = Self::pali(a, i as i32, i as i32 + 1);
            let s2 = Self::pali(a, i as i32, i as i32);
            if ret.len() < s1.len() {
                ret = s1;
            }
            if ret.len() < s2.len() {
                ret = s2;
            }
        }

        String::from_utf8_lossy(ret).to_string()
    }

    fn pali(a: &[u8], mut i: i32, mut j: i32) -> &[u8] {
        if a[i as usize] != a[j as usize] {
            return &[];
        }

        while i >= 0 && j < a.len() as i32 && a[i as usize] == a[j as usize] {
            i -= 1;
            j += 1;
        }

        &a[(i + 1) as usize..j as usize]
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
