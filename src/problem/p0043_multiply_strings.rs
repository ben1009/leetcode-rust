/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 * Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
 *  
 * <strong class="example">Example 1:
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 * <strong class="example">Example 2:
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *  
 * Constraints:
 *
 *     1 <= num1.length, num2.length <= 200
 *     num1 and num2 consist of digits only.
 *     Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let mut tmp = vec![];
        for i in (0..num1.len()).rev() {
            let mut s = vec![0; num1.len() - 1 - i];
            let mut num = 0;
            let mut carry = 0;
            for j in (0..num2.len()).rev() {
                num = (num1[i] - b'0') * (num2[j] - b'0') + carry;
                carry = num / 10;
                num %= 10;
                s.push(num);
            }
            if carry != 0 {
                s.push(carry);
            }
            s.reverse();
            tmp.push(s);
        }

        let mut ret = vec![];
        for item in &tmp {
            ret = Self::add(&ret, item);
        }

        ret.iter().map(|x| x.to_string()).collect::<String>()
    }

    fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        let mut ret = vec![];
        let mut carry = 0;
        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut num = 0;
        while i >= 0 || j >= 0 {
            num = carry;
            if i >= 0 {
                num += a[i as usize];
            }
            if j >= 0 {
                num += b[j as usize];
            }
            ret.push(num % 10);
            carry = num / 10;
            i -= 1;
            j -= 1;
        }
        if carry != 0 {
            ret.push(carry);
        }
        ret.reverse();

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}
