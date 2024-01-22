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

        let mut map = HashMap::new();
        map.insert(b'2', "abc".as_bytes());
        map.insert(b'3', "def".as_bytes());
        map.insert(b'4', "ghi".as_bytes());
        map.insert(b'5', "jkl".as_bytes());
        map.insert(b'6', "mno".as_bytes());
        map.insert(b'7', "qprs".as_bytes());
        map.insert(b'8', "tuv".as_bytes());
        map.insert(b'9', "wxyz".as_bytes());

        Self::dfs(0, digits.as_bytes(), &map, &mut vec![], vec![])
    }

    fn dfs(
        i: usize,
        digits: &[u8],
        map: &HashMap<u8, &[u8]>,
        tmp: &mut Vec<u8>,
        mut ret: Vec<String>,
    ) -> Vec<String> {
        if tmp.len() == digits.len() {
            let s = String::from_utf8_lossy(tmp).to_string();
            ret.push(s);

            return ret;
        }

        let str = map.get(&digits[i]).unwrap();
        for s in str.iter() {
            tmp.push(*s);
            ret = Self::dfs(i + 1, digits, map, tmp, ret);
            tmp.pop();
        }

        ret
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
