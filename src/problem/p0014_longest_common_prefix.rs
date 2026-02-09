use std::{cmp, mem};

/// [14] Longest Common Prefix
///
/// Write a function to find the longest common prefix string amongst an array of strings.
/// If there is no common prefix, return an empty string "".
///  
/// <strong class="example">Example 1:
///
/// Input: strs = ["flower","flow","flight"]
/// Output: "fl"
///
/// <strong class="example">Example 2:
///
/// Input: strs = ["dog","racecar","car"]
/// Output: ""
/// Explanation: There is no common prefix among the input strings.
///
///  
/// Constraints:
///
///     1 <= strs.length <= 200
///     0 <= strs[i].length <= 200
///     strs[i] consists of only lowercase English letters.
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-prefix/
// discuss: https://leetcode.com/problems/longest-common-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return mem::take(&mut strs[0]);
        }

        let mut ret = strs[0].as_bytes();
        for s in strs.iter().skip(1) {
            let mut j = 0;
            let s = s.as_bytes();
            while j < cmp::min(ret.len(), s.len()) {
                if ret[j] != s[j] {
                    break;
                }
                j += 1;
            }
            ret = &ret[..j];
        }

        String::from_utf8_lossy(ret).to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec_string!("dog", "racecar", "car")),
            String::from("")
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_string!("flower", "flow", "flight")),
            String::from("fl")
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_string!("ab", "a")),
            String::from("a")
        );
    }
}
