use std::collections::HashMap;

/// [20] Valid Parentheses
///
/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if
/// the input string is valid. An input string is valid if:
/// <ol>
///     Open brackets must be closed by the same type of brackets.
///     Open brackets must be closed in the correct order.
///     Every close bracket has a corresponding open bracket of the same type.
/// </ol>
///  
/// <strong class="example">Example 1:
///
/// Input: s = "()"
/// Output: true
///
/// <strong class="example">Example 2:
///
/// Input: s = "()[]{}"
/// Output: true
///
/// <strong class="example">Example 3:
///
/// Input: s = "(]"
/// Output: false
///
///  
/// Constraints:
///
///     1 <= s.length <= 10^4
///     s consists of parentheses only '()[]{}'.
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-parentheses/
// discuss: https://leetcode.com/problems/valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() == 1 {
            return false;
        }

        let mut stack = vec![];
        let dic = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
        for b in s.chars() {
            if !stack.is_empty() && dic[&stack[stack.len() - 1]] == b {
                stack.pop();
            } else {
                stack.push(b);
            }
        }

        stack.is_empty()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert!(Solution::is_valid("()".into()));
        assert!(Solution::is_valid("()[]{}".into()));
        assert!(!Solution::is_valid("()[{}".into()));
    }
}
