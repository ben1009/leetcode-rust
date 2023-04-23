use std::collections::HashSet;

/**
 * [345] Reverse Vowels of a String
 *
 * Given a string s, reverse only all the vowels in the string and return it.
 * The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
 *  
 * <strong class="example">Example 1:
 * Input: s = "hello"
 * Output: "holle"
 * <strong class="example">Example 2:
 * Input: s = "leetcode"
 * Output: "leotcede"
 *  
 * Constraints:
 *
 *     1 <= s.length <= 3 * 10^5
 *     s consist of printable ASCII characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-vowels-of-a-string/
// discuss: https://leetcode.com/problems/reverse-vowels-of-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let dic = HashSet::from([b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U']);
        let mut i: i32 = 0;
        let mut j = s.len() as i32 - 1;
        let mut s = s;
        let ret = unsafe { s.as_bytes_mut() };
        while i < j {
            if !dic.contains(&ret[i as usize]) {
                i += 1;
            }
            if !dic.contains(&ret[j as usize]) {
                j -= 1;
            }
            if dic.contains(&ret[i as usize]) && dic.contains(&ret[j as usize]) {
                ret.swap(i as usize, j as usize);
                i += 1;
                j -= 1;
            }
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
    }
}
