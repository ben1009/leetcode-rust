use std::collections::HashMap;

/// [17] Letter Combinations of a Phone Number
///
/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations
/// that the number could represent. Return the answer in any order. A mapping of digits to letters
/// (just like on the telephone buttons) is given below. Note that 1 does not map to any letters. <img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png" style="width: 300px; height: 243px;" />
///  
/// <strong class="example">Example 1:
///
/// Input: digits = "23"
/// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
///
/// <strong class="example">Example 2:
///
/// Input: digits = ""
/// Output: []
///
/// <strong class="example">Example 3:
///
/// Input: digits = "2"
/// Output: ["a","b","c"]
///
///  
/// Constraints:
///
///     0 <= digits.length <= 4
///     digits[i] is a digit in the range ['2', '9'].
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let dic = HashMap::from([
            (b'2', "abc"),
            (b'3', "def"),
            (b'4', "ghi"),
            (b'5', "jkl"),
            (b'6', "mno"),
            (b'7', "pqrs"),
            (b'8', "tuv"),
            (b'9', "wxyz"),
        ]);

        let mut ret = vec![];
        Solution::dfs(digits.as_bytes(), &dic, &mut vec![], &mut ret, 0);

        ret
    }

    fn dfs(
        digits: &[u8],
        dic: &HashMap<u8, &str>,
        tmp: &mut Vec<u8>,
        ret: &mut Vec<String>,
        idx: usize,
    ) {
        if tmp.len() == digits.len() {
            ret.push(String::from_utf8_lossy(tmp).to_string());

            return;
        }

        let str = dic[&digits[idx]].as_bytes();
        for s in str {
            tmp.push(*s);
            Solution::dfs(digits, dic, tmp, ret, idx + 1);
            tmp.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_owned()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("2".to_owned()),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_owned()),
            Vec::<String>::new()
        );
    }
}
