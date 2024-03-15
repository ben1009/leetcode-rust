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
        if s.len() < 2 {
            return s;
        }
        let set = HashSet::from([b'a', b'A', b'e', b'E', b'i', b'I', b'o', b'O', b'u', b'U']);

        let s1 = unsafe { s.as_bytes_mut() };
        let mut i = 0;
        let mut j = s1.len() - 1;
        while i < j {
            if !set.contains(&s1[i]) {
                i += 1;
                continue;
            }
            if !set.contains(&s1[j]) {
                j -= 1;
                continue;
            }
            s1.swap(i, j);
            i += 1;
            j -= 1;
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
