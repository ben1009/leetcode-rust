/**
 * [10] Regular Expression Matching
 *
 * Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
 *
 *     '.' Matches any single character.​​​​
 *     '*' Matches zero or more of the preceding element.
 *
 * The matching should cover the entire input string (not partial).
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 20
 *     1 <= p.length <= 20
 *     p contains only lowercase English letters, '.', and '*'.
 *     s contains only lowercase English letters.
 *     It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regular-expression-matching/
// discuss: https://leetcode.com/problems/regular-expression-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_match(_s: String, _p: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {

    #[test]
    fn test_10() {}
}
