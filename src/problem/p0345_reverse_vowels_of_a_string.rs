use std::collections::HashSet;

/// [345] Reverse Vowels of a String
///
/// Given a string s, reverse only all the vowels in the string and return it.
/// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases,
/// more than once.  
/// <strong class="example">Example 1:
/// Input: s = "hello"
/// Output: "holle"
/// <strong class="example">Example 2:
/// Input: s = "leetcode"
/// Output: "leotcede"
///  
/// Constraints:
///
///     1 <= s.length <= 3 * 10^5
///     s consist of printable ASCII characters.
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-vowels-of-a-string/
// discuss: https://leetcode.com/problems/reverse-vowels-of-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let s1 = unsafe { s.as_bytes_mut() };
        let mut lo = 0;
        let mut hi = s1.len() - 1;
        let dic = HashSet::from([b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U']);
        while lo < hi {
            if !dic.contains(&s1[lo]) {
                lo += 1;
                continue;
            }
            if !dic.contains(&s1[hi]) {
                hi -= 1;
                continue;
            }
            s1.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }

        s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_345() {
        assert_eq!(
            Solution::reverse_vowels(String::from("hello")),
            String::from("holle")
        );
        assert_eq!(
            Solution::reverse_vowels(String::from("leetcode")),
            String::from("leotcede")
        );
        assert_eq!(
            Solution::reverse_vowels(String::from("aA")),
            String::from("Aa")
        );
        assert_eq!(
            Solution::reverse_vowels(String::from("ar")),
            String::from("ar")
        );
        assert_eq!(
            Solution::reverse_vowels(String::from("sfsfsf")),
            String::from("sfsfsf")
        );
    }
}
