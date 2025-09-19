use std::collections::{HashMap, hash_map::Entry};

/// [205] Isomorphic Strings
///
/// Given two strings s and t, determine if they are isomorphic.
/// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
/// All occurrences of a character must be replaced with another character while preserving the
/// order of characters. No two characters may map to the same character, but a character may map to
/// itself.  
/// <strong class="example">Example 1:
/// Input: s = "egg", t = "add"
/// Output: true
/// <strong class="example">Example 2:
/// Input: s = "foo", t = "bar"
/// Output: false
/// <strong class="example">Example 3:
/// Input: s = "paper", t = "title"
/// Output: true
///  
/// Constraints:
///
///     1 <= s.length <= 5 * 10^4
///     t.length == s.length
///     s and t consist of any valid ascii character.
pub struct Solution {}

// problem: https://leetcode.com/problems/isomorphic-strings/
// discuss: https://leetcode.com/problems/isomorphic-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut ss = HashMap::new();
        let mut tt = HashMap::new();
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            match ss.entry(s[i]) {
                Entry::Occupied(e) => {
                    if e.get() != &t[i] {
                        return false;
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(t[i]);
                }
            }
        }
        for i in 0..t.len() {
            match tt.entry(t[i]) {
                Entry::Occupied(e) => {
                    if e.get() != &s[i] {
                        return false;
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(s[i]);
                }
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
    fn test_205() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
        assert!(Solution::is_isomorphic(
            "foo".to_string(),
            "foo".to_string()
        ));
    }
}
