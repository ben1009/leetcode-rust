/// [22] Generate Parentheses
///
/// Given n pairs of parentheses, write a function to generate all combinations of well-formed
/// parentheses.  
/// <strong class="example">Example 1:
/// Input: n = 3
/// Output: ["((()))","(()())","(())()","()(())","()()()"]
/// <strong class="example">Example 2:
/// Input: n = 1
/// Output: ["()"]
///  
/// Constraints:
///
///     1 <= n <= 8
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-parentheses/
// discuss: https://leetcode.com/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = vec![];
        Self::gen(n, 0, 0, &mut String::new(), &mut ret);

        ret
    }

    pub fn gen(n: i32, l: i32, r: i32, tmp: &mut String, ret: &mut Vec<String>) {
        if tmp.len() == n as usize * 2 {
            ret.push(tmp.to_owned());
            return;
        }

        if l < n {
            tmp.push('(');
            Self::gen(n, l + 1, r, tmp, ret);
            tmp.pop();
        }
        if r < l {
            tmp.push(')');
            Self::gen(n, l, r + 1, tmp, ret);
            tmp.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
    }
}
