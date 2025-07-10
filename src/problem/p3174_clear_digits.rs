// You are given a string s.

// Your task is to remove all digits by doing this operation repeatedly:

// Delete the first digit and the closest non-digit character to its left.
// Return the resulting string after removing all digits.

// Example 1:

// Input: s = "abc"

// Output: "abc"

// Explanation:

// There is no digit in the string.

// Example 2:

// Input: s = "cb34"

// Output: ""

// Explanation:

// First, we apply the operation on s[2], and s becomes "c4".

// Then we apply the operation on s[1], and s becomes "".

// Constraints:

// 1 <= s.length <= 100
// s consists only of lowercase English letters and digits.
// The input is generated such that it is possible to delete all digits.

// problem: https://leetcode.com/problems/clear-digits/

// submission codes start here
pub struct Solution {}

impl Solution {
    pub fn clear_digits(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let mut ret = Vec::new();
        for c in s.chars() {
            if c.is_alphabetic() {
                ret.push(c);
            } else if ret.last().unwrap_or(&'a').is_alphabetic() {
                ret.pop();
            } else {
                ret.push(c);
            }
        }

        ret.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3174() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc".to_string());
        assert_eq!(Solution::clear_digits("cb34".to_string()), "".to_string());
        assert_eq!(Solution::clear_digits("1".to_string()), "1".to_string());
        assert_eq!(Solution::clear_digits("11".to_string()), "".to_string());
    }
}
