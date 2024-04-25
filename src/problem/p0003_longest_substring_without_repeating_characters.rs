use std::collections::{hash_map::Entry, HashMap};

/// [3] Longest Substring Without Repeating Characters
///
/// Given a string s, find the length of the longest <span
/// data-keyword="substring-nonempty">substring</span> without repeating characters.  
/// <strong class="example">Example 1:
///
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
///
/// <strong class="example">Example 2:
///
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
///
/// <strong class="example">Example 3:
///
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
///
///  
/// Constraints:
///
///     0 <= s.length <= 5 * 10^4
///     s consists of English letters, digits, symbols and spaces.
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }

        let mut ret = 0;
        let mut pre = 0;
        let mut dic = HashMap::new();
        for (i, b) in s.as_bytes().iter().enumerate() {
            match dic.entry(b) {
                Entry::Occupied(o) => {
                    if pre <= *o.get() {
                        pre = o.get() + 1;
                    }
                }
                Entry::Vacant(_) => {}
            }
            ret = std::cmp::max(ret, i - pre + 1);
            dic.insert(b, i);
        }

        ret as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("abcda".into()), 4);
        assert_eq!(Solution::length_of_longest_substring("abcd".into()), 4);
        assert_eq!(Solution::length_of_longest_substring("".into()), 0);
        assert!(Solution::length_of_longest_substring("tmmzuxt".into()) == 5);
    }
}
